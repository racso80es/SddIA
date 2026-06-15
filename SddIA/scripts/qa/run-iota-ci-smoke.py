#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Smoke CI E.1: pending → route-domain-event (watcher) → IOTA cumulo success.

Modos:
  --simulate          Fuerza SDDIA_LAB_SIMULATE_IOTA=1 (default en lab local).
  --require-physical  Prohíbe simulate; exige IOTA_WALLET_SECRET o wallet.key.

Salida: JSON en stdout (última línea) con success, event_id, delivery_status,
transaction_digest (si disponible).
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import uuid
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

SCRIPT = Path(__file__).resolve()
if str(SCRIPT.parent) not in sys.path:
    sys.path.insert(0, str(SCRIPT.parent))

from eda_bus_utils import ensure_event_bus_topology, list_witnesses, load_eda_bus
from env_loader import load_hierarchical_env


def _resolve_watcher(repo: Path) -> list[str]:
    from capsule_resolve import resolve_daemon_capsule

    try:
        return [str(resolve_daemon_capsule(repo, "event-watcher"))]
    except FileNotFoundError:
        sh = repo / "SddIA" / "daemons" / "event-watcher.sh"
        if sh.is_file():
            return [str(sh)]
        legacy = repo / "SddIA" / "scripts" / "limbo" / "daemons" / "event-watcher.py"
        if legacy.is_file():
            return [sys.executable, str(legacy)]
        raise


IOTA_SUBSCRIBER = "cumulo.iota-immutable-publisher"
FIXTURE_REL = "docs/features/e1-iota-ci/_smoke-iota-ci-merged.json"


def _repo_root() -> Path:
    for parent in SCRIPT.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace")


def _iso_now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).strftime("%Y-%m-%dT%H:%M:%SZ")


def _truthy_env(name: str) -> bool:
    return os.environ.get(name, "").strip().lower() in ("1", "true", "yes")


def _wallet_available() -> bool:
    if os.environ.get("IOTA_WALLET_SECRET", "").strip():
        return True
    repo = _repo_root()
    wallet = repo / ".SddIA" / ".dev" / "wallet.key"
    return wallet.is_file() and bool(wallet.read_text(encoding="utf-8").strip())


def _load_fixture(repo: Path) -> dict[str, Any]:
    path = repo / FIXTURE_REL
    if not path.is_file():
        raise FileNotFoundError(f"fixture ausente: {FIXTURE_REL}")
    template = json.loads(path.read_text(encoding="utf-8"))
    event_id = str(uuid.uuid4())
    correlation = template.get("correlation_id") or "iota-ci-smoke"
    payload = dict(template.get("payload") or {})
    payload.setdefault("merge_commit_hash", "0" * 40)
    payload.setdefault("source_branch", "feat/e1-iota-ci-smoke")
    payload.setdefault("target_branch", "main")
    payload.setdefault("author", "iota-ci-smoke@sddia.local")
    payload.setdefault("security_clearance", "lab-smoke")
    return {
        "event_id": event_id,
        "event_type": "PullRequest_Merged",
        "timestamp": _iso_now(),
        "emitter_agent": "iota-ci-smoke",
        "correlation_id": f"{correlation}-{event_id[:8]}",
        "payload": payload,
        "delivery_state": {},
    }


def _write_pending(repo: Path, event: dict[str, Any]) -> Path:
    bus = ensure_event_bus_topology(repo)
    pending_dir = repo / bus["pending"]
    pending_dir.mkdir(parents=True, exist_ok=True)
    dest = pending_dir / f"{event['event_id']}.json"
    dest.write_text(json.dumps(event, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    try:
        rel = dest.resolve().relative_to(repo.resolve()).as_posix()
    except ValueError:
        rel = str(dest)
    return Path(rel)


def _route_event(repo: Path, rel_path: str, env: dict[str, str]) -> dict[str, Any]:
    proc = subprocess.run(
        _resolve_watcher(repo) + ["--event-file-path", rel_path],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        env=env,
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if not line:
        raise RuntimeError(proc.stderr or "watcher sin salida JSON")
    body = json.loads(line)
    if proc.returncode != 0 and not body.get("success"):
        err = body.get("error") or proc.stderr or "route failed"
        raise RuntimeError(err)
    return body


def _cleanup_smoke_artifacts(repo: Path, event_id: str) -> None:
    bus = load_eda_bus(repo)
    patterns = [
        repo / bus["pending"] / f"{event_id}.json",
        repo / bus["processing"] / f"{event_id}.json",
        repo / bus["processed"] / f"{event_id}.json",
        repo / bus["dead_letter"] / f"{event_id}.json",
    ]
    for path in patterns:
        path.unlink(missing_ok=True)
    for state_key in (
        "processing_subscribers",
        "processed_subscribers",
        "dead_letter_subscribers",
    ):
        for witness in list_witnesses(repo, bus, state_key, event_id):
            witness.unlink(missing_ok=True)


def run_smoke(*, simulate: bool, require_physical: bool) -> dict[str, Any]:
    repo = _repo_root()
    load_hierarchical_env(repo)
    env = os.environ.copy()

    if require_physical:
        if simulate:
            raise ValueError("--simulate incompatible con --require-physical")
        env.pop("SDDIA_LAB_SIMULATE_IOTA", None)
        if _truthy_env("SDDIA_LAB_SIMULATE_IOTA"):
            raise RuntimeError("SDDIA_LAB_SIMULATE_IOTA activo — aborto modo físico")
        if not _wallet_available():
            raise RuntimeError(
                "IOTA_WALLET_SECRET o .SddIA/.dev/wallet.key requeridos para modo físico"
            )
    elif simulate:
        env["SDDIA_LAB_SIMULATE_IOTA"] = "1"

    event = _load_fixture(repo)
    event_id = event["event_id"]
    rel_path = str(_write_pending(repo, event))

    try:
        route_out = _route_event(repo, rel_path, env)
        data = route_out.get("data") or {}
        delivery_status = data.get("delivery_status") or {}
        cumulo_status = delivery_status.get(IOTA_SUBSCRIBER)
        if cumulo_status != "success":
            raise RuntimeError(
                f"{IOTA_SUBSCRIBER} status={cumulo_status!r} delivery_status={delivery_status}"
            )

        digest = data.get("transaction_digest")
        mode = "physical" if require_physical else ("simulate" if simulate else "default")

        if require_physical:
            if not isinstance(digest, str) or not digest.strip():
                raise RuntimeError("transaction_digest ausente en modo físico")
            if digest.startswith("lab-sim-"):
                raise RuntimeError(f"digest simulado en modo físico: {digest}")

        bus = load_eda_bus(repo)
        witnesses = list_witnesses(repo, bus, "processed_subscribers", event_id)
        witness_ok = any(IOTA_SUBSCRIBER in p.name for p in witnesses)

        return {
            "success": True,
            "mode": mode,
            "event_id": event_id,
            "event_type": event["event_type"],
            "delivery_status": delivery_status,
            "transaction_digest": digest,
            "witness_processed": witness_ok,
            "parent_path": data.get("parent_path"),
        }
    finally:
        _cleanup_smoke_artifacts(repo, event_id)


def main() -> int:
    parser = argparse.ArgumentParser(description="Smoke CI IOTA E.1")
    parser.add_argument("--simulate", action="store_true", help="Forzar SDDIA_LAB_SIMULATE_IOTA=1")
    parser.add_argument(
        "--require-physical",
        action="store_true",
        help="Anclaje Testnet real (sin simulate)",
    )
    parser.add_argument("--json", action="store_true", help="Imprimir JSON (default)")
    args = parser.parse_args()

    try:
        result = run_smoke(simulate=args.simulate, require_physical=args.require_physical)
        sys.stdout.write(json.dumps(result, ensure_ascii=False) + "\n")
        return 0
    except Exception as exc:
        err = {"success": False, "error": str(exc)}
        sys.stdout.write(json.dumps(err, ensure_ascii=False) + "\n")
        sys.stderr.write(f"run-iota-ci-smoke: {exc}\n")
        return 1


if __name__ == "__main__":
    sys.exit(main())
