# -*- coding: utf-8 -*-
"""Materialización ECST PullRequest_Presented vía anclaje DLT (oráculo sensor)."""

from __future__ import annotations

import json
import os
import shutil
import subprocess
import time
import uuid
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from eda_bus_utils import _iso_now, _write_json_atomic, ensure_event_bus_topology

_SUBPROCESS_UTF8 = {"text": True, "encoding": "utf-8", "errors": "replace"}
IOTA_RETRIES = 3
IOTA_BACKOFF_SECONDS = (1, 2, 4)
SIGNER_RBAC = "Vertice_Biologico_Relay"
FALLBACK_FLAG = "FALLBACK_LOCAL_SIGNATURE"


def _iota_timeout_seconds() -> int:
    return int(os.environ.get("SDDIA_IOTA_TIMEOUT_SECONDS", "45"))


def _run_subprocess(cmd: list[str], *, input_text: str | None = None, **kwargs: Any) -> subprocess.CompletedProcess[str]:
    opts = {**_SUBPROCESS_UTF8, "capture_output": True, "check": False}
    opts.update(kwargs)
    if input_text is not None:
        opts["input"] = input_text
    return subprocess.run(cmd, **opts)


def load_wallet_secret(repo: Path) -> None:
    """Inyecta IOTA_WALLET_SECRET desde .SddIA/.dev/wallet.key si no está en env."""
    if os.environ.get("IOTA_WALLET_SECRET", "").strip():
        return
    wallet_path = repo / ".SddIA" / ".dev" / "wallet.key"
    if not wallet_path.is_file():
        return
    secret = wallet_path.read_text(encoding="utf-8").strip()
    if secret:
        os.environ["IOTA_WALLET_SECRET"] = secret


def compose_pre_anchor_event(pr: dict[str, Any]) -> dict[str, Any]:
    """Payload ECST pre-anclaje (sin event_id definitivo)."""
    repository = pr.get("repository") or ""
    branch = pr.get("branch") or ""
    pr_url = pr.get("pr_url") or ""
    origin_agent = pr.get("origin_agent") or "jules"
    return {
        "event_type": "PullRequest_Presented",
        "timestamp": _iso_now(),
        "emitter_agent": "github-bridge-watcher",
        "payload": {
            "repository": repository,
            "branch": branch,
            "pr_url": pr_url,
            "status": "presented",
            "origin_agent": origin_agent,
            "signer_identity_rbac": SIGNER_RBAC,
        },
    }


def invoke_iota_publisher(repo: Path, event: dict[str, Any]) -> tuple[bool, str, str | None, str | None]:
    """Publica en IOTA; retorna (ok, feedback, transaction_digest, object_id)."""
    if os.environ.get("SDDIA_LAB_SIMULATE_IOTA", "").strip().lower() in ("1", "true", "yes"):
        digest = f"lab-sim-{uuid.uuid4().hex[:24]}"
        return True, "lab-simulated", digest, None

    tool_dir = repo / "SddIA" / "scripts" / "tools" / "iota-immutable-publisher"
    entry = tool_dir / "index.ts"
    if not entry.is_file():
        return False, "iota-immutable-publisher entry not found", None, None
    npx = shutil.which("npx")
    if not npx:
        return False, "npx not found on PATH", None, None

    payload = {
        "action": "publish_immutable_data",
        "network": "testnet",
        "payload": json.dumps(event, ensure_ascii=False),
    }
    try:
        proc = _run_subprocess(
            [npx, "tsx", str(entry)],
            input_text=json.dumps(payload),
            cwd=str(tool_dir),
            timeout=_iota_timeout_seconds(),
            shell=False,
        )
    except subprocess.TimeoutExpired:
        return False, "iota-immutable-publisher timeout", None, None
    except OSError as e:
        return False, str(e), None, None

    if proc.returncode != 0:
        return False, (proc.stderr or proc.stdout or "iota publish failed").strip(), None, None

    try:
        body = json.loads(proc.stdout.strip() or "{}")
    except json.JSONDecodeError:
        return False, "invalid JSON from iota-immutable-publisher", None, None

    if not body.get("success"):
        return False, body.get("feedback", "iota publish failed"), None, None

    result = body.get("result") or {}
    digest = result.get("transaction_digest")
    object_id = result.get("object_id")
    if not isinstance(digest, str) or not digest.strip():
        return False, "missing transaction_digest", None, None
    return True, body.get("feedback", "ok"), digest.strip(), (
        object_id.strip() if isinstance(object_id, str) and object_id.strip() else None
    )


def publish_with_retries(repo: Path, event: dict[str, Any]) -> tuple[str | None, str | None, str | None]:
    """Reintentos ×3; retorna (digest, object_id, last_error)."""
    last_error: str | None = None
    for attempt, delay in enumerate(IOTA_BACKOFF_SECONDS, start=1):
        ok, feedback, digest, object_id = invoke_iota_publisher(repo, event)
        if ok and digest:
            return digest, object_id, None
        last_error = feedback
        if attempt < IOTA_RETRIES:
            time.sleep(delay)
    return None, None, last_error


def build_bus_event(
    pre_event: dict[str, Any],
    transaction_digest: str,
    object_id: str | None,
) -> dict[str, Any]:
    payload = dict(pre_event.get("payload") or {})
    payload["dlt_anchor_address"] = object_id or transaction_digest
    return {
        "event_id": transaction_digest,
        "event_type": pre_event.get("event_type", "PullRequest_Presented"),
        "timestamp": pre_event.get("timestamp") or _iso_now(),
        "emitter_agent": pre_event.get("emitter_agent", "github-bridge-watcher"),
        "payload": payload,
        "delivery_state": {
            "argos": "pending",
            "cumulo": "success",
        },
    }


def materialize_to_bus(repo: Path, bus_event: dict[str, Any]) -> Path | None:
    """Escribe .events/pending/<digest>.json; idempotente."""
    event_id = bus_event.get("event_id")
    if not isinstance(event_id, str) or not event_id.strip():
        raise ValueError("event_id inválido para materialización")
    bus = ensure_event_bus_topology(repo)
    target = repo / bus["pending"] / f"{event_id.strip()}.json"
    if target.is_file():
        return None
    _write_json_atomic(target, bus_event)
    return target


def write_fallback_dead_letter(repo: Path, pr: dict[str, Any], error: str) -> Path:
    bus = ensure_event_bus_topology(repo)
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    name = f"github-bridge-{stamp}-{uuid.uuid4().hex[:8]}.json"
    target = repo / bus["dead_letter"] / name
    record = {
        "flag": FALLBACK_FLAG,
        "timestamp": _iso_now(),
        "source": "github-bridge-watcher",
        "error": error,
        "pr": {
            "repository": pr.get("repository"),
            "branch": pr.get("branch"),
            "pr_url": pr.get("pr_url"),
            "origin_agent": pr.get("origin_agent"),
        },
    }
    _write_json_atomic(target, record)
    return target
