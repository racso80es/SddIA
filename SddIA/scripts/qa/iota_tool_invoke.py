# -*- coding: utf-8 -*-
"""Invocación iota-immutable-publisher — Rust cápsula con fallback TypeScript legacy."""

from __future__ import annotations

import json
import os
import shutil
import subprocess
import uuid
from pathlib import Path
from typing import Any

from capsule_resolve import invoke_tool_capsule_json


def _iota_payload(event: dict[str, Any]) -> dict[str, str]:
    return {
        "action": "publish_immutable_data",
        "network": "testnet",
        "payload": json.dumps(event, ensure_ascii=False),
    }


def _parse_iota_body(body: dict[str, Any]) -> tuple[bool, str, int, str | None]:
    ok = bool(body.get("success"))
    digest = (body.get("result") or {}).get("transaction_digest")
    if isinstance(digest, str) and digest.strip():
        digest = digest.strip()
    else:
        digest = None
    feedback = body.get("feedback") or body.get("message") or ("ok" if ok else "iota publish failed")
    if not isinstance(feedback, str):
        feedback = str(feedback)
    return ok, feedback, 0 if ok else 1, digest


def _iota_legacy_dir(repo: Path) -> Path:
    return repo / "SddIA" / "scripts" / "limbo" / "tools" / "iota-immutable-publisher"


def _invoke_iota_typescript_legacy(repo: Path, event: dict[str, Any]) -> tuple[bool, str, int, str | None]:
    tool_dir = _iota_legacy_dir(repo)
    entry = tool_dir / "index.ts"
    if not entry.is_file():
        return False, "iota-immutable-publisher entry not found", 1, None
    local_ts_node = tool_dir / "node_modules" / ".bin" / "ts-node"
    npx = shutil.which("npx")
    if local_ts_node.is_file():
        runner_cmd: list[str] = [str(local_ts_node), str(entry)]
    elif npx:
        runner_cmd = [npx, "ts-node", str(entry)]
    else:
        return False, "npx not found on PATH (ejecute npm install en iota-immutable-publisher)", 1, None
    payload = _iota_payload(event)
    proc = subprocess.run(
        runner_cmd,
        input=json.dumps(payload),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(tool_dir),
        check=False,
    )
    if proc.returncode != 0:
        return (
            False,
            (proc.stderr or proc.stdout or "iota publish failed").strip(),
            proc.returncode,
            None,
        )
    try:
        body = json.loads(proc.stdout.strip() or "{}")
    except json.JSONDecodeError:
        return False, "invalid JSON from iota-immutable-publisher", 1, None
    if not isinstance(body, dict):
        return False, "invalid envelope from iota-immutable-publisher", 1, None
    return _parse_iota_body(body)


def invoke_iota_immutable_publisher(
    repo: Path,
    event: dict[str, Any],
    *,
    timeout_seconds: int | None = None,
) -> tuple[bool, str, int, str | None]:
    """Publica evento inmutable; simula, Rust nativo/WASI o fallback TS legacy."""
    if os.environ.get("SDDIA_LAB_SIMULATE_IOTA", "").strip().lower() in ("1", "true", "yes"):
        digest = f"lab-sim-{uuid.uuid4().hex[:24]}"
        return True, "lab-simulated", 0, digest

    payload = _iota_payload(event)
    native_error: str | None = None
    try:
        _rc, body = invoke_tool_capsule_json(
            repo,
            "iota-immutable-publisher",
            payload,
            prefer_wasm=False,
        )
        if isinstance(body, dict) and body.get("success"):
            return _parse_iota_body(body)
        if isinstance(body, dict):
            native_error = str(
                body.get("error")
                or body.get("message")
                or body.get("feedback")
                or "iota rust capsule failed"
            )
    except FileNotFoundError:
        native_error = "iota-immutable-publisher native capsule not found"

    tool_dir = _iota_legacy_dir(repo)
    local_ts_node = tool_dir / "node_modules" / ".bin" / "ts-node"
    npx = shutil.which("npx")
    node = shutil.which("node")
    has_ts_toolchain = local_ts_node.is_file() or (bool(npx) and bool(node))

    if not has_ts_toolchain:
        return (
            False,
            native_error
            or "iota-immutable-publisher unavailable (no native success, no node toolchain)",
            1,
            None,
        )

    if timeout_seconds is not None:
        entry = tool_dir / "index.ts"
        if not entry.is_file():
            return False, "iota-immutable-publisher entry not found", 1, None
        if local_ts_node.is_file():
            runner_cmd = [str(local_ts_node), str(entry)]
        else:
            runner_cmd = [npx, "ts-node", str(entry)]
        try:
            proc = subprocess.run(
                runner_cmd,
                input=json.dumps(payload),
                capture_output=True,
                text=True,
                encoding="utf-8",
                errors="replace",
                cwd=str(tool_dir),
                timeout=timeout_seconds,
                check=False,
            )
        except subprocess.TimeoutExpired:
            return False, "iota-immutable-publisher timeout", 1, None
        except OSError as e:
            return False, str(e), 1, None
        if proc.returncode != 0:
            return (
                False,
                (proc.stderr or proc.stdout or "iota publish failed").strip(),
                proc.returncode,
                None,
            )
        try:
            body = json.loads(proc.stdout.strip() or "{}")
        except json.JSONDecodeError:
            return False, "invalid JSON from iota-immutable-publisher", 1, None
        if not isinstance(body, dict):
            return False, "invalid envelope from iota-immutable-publisher", 1, None
        return _parse_iota_body(body)

    return _invoke_iota_typescript_legacy(repo, event)
