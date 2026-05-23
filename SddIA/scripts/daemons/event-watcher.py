#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Despertador inerte: monitoriza .events/pending/ y delega en proceso route-domain-event.

Ola C V3+: topología simétrica; orquestación vía execute-process (proceso SddIA).

Variables de entorno:
  SDDIA_LAB_SIMULATE_IOTA=1           Simula iota-immutable-publisher.
  SDDIA_LAB_SIMULATE_SYNC_INDEX=1     Simula sync-entity-index.
  SDDIA_LAB_ROUTE_SYNC=1              Fan-out secuencial (regresión CI).
  SDDIA_IOTA_TIMEOUT_SECONDS=N        Timeout IOTA (default 45).

Uso:
  python SddIA/scripts/daemons/event-watcher.py
  python SddIA/scripts/daemons/event-watcher.py --once
  python SddIA/scripts/daemons/event-watcher.py --event-file-path .events/pending/x.json
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import time
from pathlib import Path
from typing import Any

_SCRIPT_DIR = Path(__file__).resolve().parent
_QA_DIR = _SCRIPT_DIR.parent / "qa"
if str(_QA_DIR) not in sys.path:
    sys.path.insert(0, str(_QA_DIR))

from eda_bus_utils import ensure_event_bus_topology, list_witnesses, load_eda_bus
from env_loader import load_hierarchical_env
from route_domain_event_core import route_domain_event

POLL_SECONDS = 2
MAX_ROUTE_ATTEMPTS = 3
_SUBPROCESS_UTF8 = {"text": True, "encoding": "utf-8", "errors": "replace"}


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace")


def _rel_event_path(repo: Path, event_path: Path) -> str:
    try:
        return event_path.resolve().relative_to(repo.resolve()).as_posix()
    except ValueError:
        return str(event_path.resolve())


def _emit_route_result(out: dict[str, Any]) -> None:
    sys.stdout.write(json.dumps(out, ensure_ascii=False))


def _has_dead_letter_witnesses(repo: Path, bus: dict[str, str], event_uuid: str) -> bool:
    return bool(list_witnesses(repo, bus, "dead_letter_subscribers", event_uuid))


def _extract_sweep_from_route_stdout(stdout: str) -> dict[str, Any] | None:
    line = (stdout or "").strip()
    if not line:
        return None
    try:
        body = json.loads(line.splitlines()[-1])
    except json.JSONDecodeError:
        return None
    data = body.get("data")
    if isinstance(data, dict):
        sweep = data.get("sweep")
        if isinstance(sweep, dict):
            return sweep
    return None


def _log_route_outcome(
    repo: Path,
    bus: dict[str, str],
    key: str,
    event_uuid: str,
    proc: subprocess.CompletedProcess[str],
) -> None:
    if proc.returncode != 0:
        print(
            f"[WATCHER] route-domain-event falló ({key}): "
            f"{(proc.stderr or proc.stdout or '').strip()}",
            flush=True,
        )
        return
    sweep = _extract_sweep_from_route_stdout(proc.stdout or "")
    pending_path = repo / bus["pending"] / key
    if sweep and sweep.get("status") == "kaizen-finalized":
        print(
            f"[WATCHER] {key}: Kaizen terminalizado — padre retirado de pending",
            flush=True,
        )
        return
    if _has_dead_letter_witnesses(repo, bus, event_uuid):
        print(
            f"[WATCHER] {key}: testigo dead-letter — padre permanece en pending (Kaizen)",
            flush=True,
        )
        return
    if sweep and sweep.get("status") == "purged":
        print(f"[WATCHER] {key}: enrutado y purgado de pending", flush=True)
    elif not pending_path.is_file():
        print(f"[WATCHER] {key}: enrutado y purgado de pending", flush=True)
    elif sweep and sweep.get("status") == "awaiting":
        pending_subs = sweep.get("pending_subscribers") or []
        print(
            f"[WATCHER] {key}: enrutado — suscriptores pendientes: {pending_subs}",
            flush=True,
        )
    else:
        print(f"[WATCHER] {key}: enrutado — consenso pendiente (sweeper)", flush=True)


def _invoke_route_process(repo: Path, rel_path: str) -> subprocess.CompletedProcess[str]:
    runner = repo / "SddIA" / "scripts" / "qa" / "execute-process.py"
    payload = json.dumps({"event_file_path": rel_path}, ensure_ascii=False)
    return subprocess.run(
        [sys.executable, str(runner), "--process", "route-domain-event", "--inputs", payload],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )


def _run_route_cli() -> None:
    parser = argparse.ArgumentParser(description="route-domain-event (compat CLI)")
    parser.add_argument("--event-file-path", required=True)
    args = parser.parse_args()
    repo = _repo_root()
    out = route_domain_event(repo, args.event_file_path)
    _emit_route_result(out)
    sys.exit(0 if out.get("exitCode") == 0 else 1)


def _run_watcher(*, once: bool = False) -> None:
    repo = _repo_root()
    bus = ensure_event_bus_topology(repo)
    pending = repo / bus["pending"]
    attempts: dict[str, int] = {}
    in_flight: set[str] = set()

    print("[WATCHER] Iniciado. pending=", pending, flush=True)
    while True:
        for path in sorted(pending.glob("*.json")):
            key = path.name
            event_uuid = path.stem
            if key in in_flight:
                continue
            if _has_dead_letter_witnesses(repo, bus, event_uuid):
                continue
            n = attempts.get(key, 0)
            if n >= MAX_ROUTE_ATTEMPTS:
                print(f"[WATCHER] Skip {key}: max attempts ({MAX_ROUTE_ATTEMPTS})", flush=True)
                continue

            rel = _rel_event_path(repo, path)
            print(f"[WATCHER] Detectado nuevo evento: {key}", flush=True)
            in_flight.add(key)
            attempts[key] = n + 1

            proc = _invoke_route_process(repo, rel)
            in_flight.discard(key)

            if proc.returncode == 0 and not _has_dead_letter_witnesses(repo, bus, event_uuid):
                attempts.pop(key, None)
            _log_route_outcome(repo, bus, key, event_uuid, proc)

        time.sleep(POLL_SECONDS)
        if once:
            print("[WATCHER] Ciclo único (--once). Fin.", flush=True)
            break


def main() -> None:
    load_hierarchical_env(_repo_root())
    if "--event-file-path" in sys.argv:
        _run_route_cli()
    else:
        try:
            _run_watcher(once="--once" in sys.argv)
        except KeyboardInterrupt:
            print("[WATCHER] Detenido.", flush=True)
            sys.exit(0)


if __name__ == "__main__":
    main()
