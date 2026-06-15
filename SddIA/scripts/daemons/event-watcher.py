#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Despertador inerte: monitoriza .events/pending/ y delega en proceso route-domain-event.

Ola C V3+: topología simétrica; orquestación vía execute-process (proceso SddIA).

Variables de entorno:
  SDDIA_LAB_SIMULATE_IOTA=0           Anclaje IOTA Testnet real (default bóveda producción).
  SDDIA_LAB_SIMULATE_IOTA=1           Simula iota-immutable-publisher (solo lab/CI).
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
import os
import subprocess
import sys
import time
from pathlib import Path
from typing import Any

_SCRIPT_DIR = Path(__file__).resolve().parent
_QA_DIR = _SCRIPT_DIR.parent / "qa"
if str(_QA_DIR) not in sys.path:
    sys.path.insert(0, str(_QA_DIR))

from eda_bus_utils import ensure_event_bus_topology, list_witnesses, load_eda_bus, load_eda_fractal
from daemon_centinel_runtime import centinela_runtime
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


def _invoke_route_process(repo: Path, rel_path: str, process_name: str) -> subprocess.CompletedProcess[str]:
    runner = repo / "SddIA" / "scripts" / "qa" / "execute-process.py"
    payload = json.dumps({"event_file_path": rel_path}, ensure_ascii=False)
    return subprocess.run(
        [sys.executable, str(runner), "--process", process_name, "--inputs", payload],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )


def _fractal_watch_enabled() -> bool:
    return os.environ.get("SDDIA_LAB_WATCH_FRACTAL", "1").strip().lower() not in (
        "0",
        "false",
        "no",
    )


def _watch_targets(repo: Path, bus: dict[str, str]) -> list[tuple[Path, str]]:
    targets: list[tuple[Path, str]] = [(repo / bus["pending"], "route-domain-event")]
    if not _fractal_watch_enabled():
        return targets
    fractal = load_eda_fractal(repo)
    targets.extend(
        [
            (repo / fractal["telemetry"], "route-telemetry"),
            (repo / fractal["orchestration"], "route-orchestration"),
            (repo / fractal["domain"], "route-domain"),
        ]
    )
    return targets


def _run_route_cli() -> None:
    parser = argparse.ArgumentParser(description="route-domain-event (compat CLI)")
    parser.add_argument("--event-file-path", required=True)
    args = parser.parse_args()
    repo = _repo_root()
    out = route_domain_event(repo, args.event_file_path)
    _emit_route_result(out)
    sys.exit(0 if out.get("exitCode") == 0 else 1)


def _prune_routed_ok_pending_absent(
    routed_ok_pending_absent: set[str],
    watch_targets: list[tuple[Path, str]],
) -> None:
    """Retira UUIDs del set D3 cuando el archivo ya no está en ninguna cola vigilada."""
    still_present: set[str] = set()
    for watch_dir, _ in watch_targets:
        if not watch_dir.is_dir():
            continue
        for path in watch_dir.glob("*.json"):
            still_present.add(path.stem)
    routed_ok_pending_absent.intersection_update(still_present)


def _watcher_skip_reason(
    *,
    event_uuid: str,
    key: str,
    process_name: str,
    path: Path,
    processing_uuids: set[str],
    routed_ok_pending_absent: set[str],
    repo: Path,
    bus: dict[str, str],
    attempts: dict[str, int],
) -> str | None:
    if event_uuid in processing_uuids:
        return f"in-flight uuid={event_uuid}"
    if event_uuid in routed_ok_pending_absent and path.is_file():
        return f"routed-ok pending file uuid={event_uuid}"
    if process_name == "route-domain-event" and _has_dead_letter_witnesses(
        repo, bus, event_uuid
    ):
        return "dead-letter kaizen"
    if attempts.get(key, 0) >= MAX_ROUTE_ATTEMPTS:
        return f"max attempts ({MAX_ROUTE_ATTEMPTS})"
    return None


def _run_watcher(*, once: bool = False) -> None:
    repo = _repo_root()
    centinela = centinela_runtime(repo, "event-watcher")
    centinela.bootstrap()
    bus = ensure_event_bus_topology(repo)
    attempts: dict[str, int] = {}
    processing_uuids: set[str] = set()
    routed_ok_pending_absent: set[str] = set()
    targets = _watch_targets(repo, bus)

    print("[WATCHER] Iniciado. roots=", [str(t[0]) for t in targets], flush=True)
    while True:
        _prune_routed_ok_pending_absent(routed_ok_pending_absent, targets)
        for watch_dir, process_name in targets:
            if not watch_dir.is_dir():
                continue
            for path in sorted(watch_dir.glob("*.json")):
                key = f"{watch_dir.name}/{path.name}"
                event_uuid = path.stem
                skip = _watcher_skip_reason(
                    event_uuid=event_uuid,
                    key=key,
                    process_name=process_name,
                    path=path,
                    processing_uuids=processing_uuids,
                    routed_ok_pending_absent=routed_ok_pending_absent,
                    repo=repo,
                    bus=bus,
                    attempts=attempts,
                )
                if skip:
                    if skip.startswith("in-flight"):
                        print(f"[WATCHER] skip {skip}", flush=True)
                    elif skip.startswith("routed-ok"):
                        print(f"[WATCHER] skip {skip}", flush=True)
                    elif skip.startswith("max attempts"):
                        print(f"[WATCHER] Skip {key}: {skip}", flush=True)
                    continue

                rel = _rel_event_path(repo, path)
                print(f"[WATCHER] Detectado nuevo evento: {key} → {process_name}", flush=True)
                centinela.note_stimulus()
                processing_uuids.add(event_uuid)
                attempts[key] = attempts.get(key, 0) + 1

                proc = _invoke_route_process(repo, rel, process_name)
                processing_uuids.discard(event_uuid)

                if proc.returncode == 0:
                    if path.is_file():
                        routed_ok_pending_absent.add(event_uuid)
                    else:
                        routed_ok_pending_absent.discard(event_uuid)
                        attempts.pop(key, None)

                if process_name == "route-domain-event":
                    if proc.returncode == 0 and not _has_dead_letter_witnesses(
                        repo, bus, event_uuid
                    ):
                        if not path.is_file():
                            attempts.pop(key, None)
                    _log_route_outcome(repo, bus, path.name, event_uuid, proc)
                elif proc.returncode == 0:
                    if not path.is_file():
                        attempts.pop(key, None)
                    purged_note = ""
                    if path.is_file():
                        purged_note = " (archivo persiste — D3 activo)"
                    print(
                        f"[WATCHER] {key}: enrutado ({process_name}){purged_note}",
                        flush=True,
                    )
                else:
                    routed_ok_pending_absent.discard(event_uuid)
                    print(
                        f"[WATCHER] {process_name} falló ({key}): "
                        f"{(proc.stderr or proc.stdout or '').strip()}",
                        flush=True,
                    )

        centinela.tick()
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
            centinela.shutdown()
            print("[WATCHER] Detenido.", flush=True)
            sys.exit(0)


if __name__ == "__main__":
    main()
