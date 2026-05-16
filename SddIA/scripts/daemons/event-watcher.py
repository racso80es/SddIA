#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Despertador inerte: monitoriza .SddIA/events/pending/ y delega en route-domain-event.

Variables de entorno:
  SDDIA_LAB_SIMULATE_IOTA=1     Simula éxito de iota-immutable-publisher (laboratorio).
  SDDIA_IOTA_TIMEOUT_SECONDS=N  Timeout de publicación IOTA (default 45).

Uso:
  python SddIA/scripts/daemons/event-watcher.py           # bucle continuo
  python SddIA/scripts/daemons/event-watcher.py --once  # un ciclo de sondeo
  python SddIA/scripts/daemons/event-watcher.py --event-file-path .SddIA/events/pending/x.json
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import time
from pathlib import Path
from typing import Any

POLL_SECONDS = 2
MAX_ROUTE_ATTEMPTS = 3
IOTA_TIMEOUT_SECONDS = int(os.environ.get("SDDIA_IOTA_TIMEOUT_SECONDS", "45"))


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    _fail("No se encontró raíz del workspace (SddIA/core/cumulo.paths.json)")


def _fail(msg: str) -> None:
    print(json.dumps({"success": False, "exitCode": 1, "error": msg}), file=sys.stderr)
    sys.exit(1)


def _emit_route_result(out: dict[str, Any]) -> None:
    sys.stdout.write(json.dumps(out, ensure_ascii=False))


def _rel_event_path(repo: Path, event_path: Path) -> str:
    try:
        return event_path.resolve().relative_to(repo.resolve()).as_posix()
    except ValueError:
        return str(event_path.resolve())


def _invoke_iota_publisher(repo: Path, event: dict[str, Any]) -> tuple[bool, str]:
    if os.environ.get("SDDIA_LAB_SIMULATE_IOTA", "").strip().lower() in (
        "1",
        "true",
        "yes",
    ):
        return True, "lab-simulated"
    tool_dir = repo / "SddIA" / "scripts" / "tools" / "iota-immutable-publisher"
    entry = tool_dir / "index.ts"
    if not entry.is_file():
        return False, "iota-immutable-publisher entry not found"
    npx = shutil.which("npx")
    if not npx:
        return False, "npx not found on PATH"
    payload = {
        "action": "publish_immutable_data",
        "network": "testnet",
        "payload": json.dumps(event, ensure_ascii=False),
    }
    try:
        proc = subprocess.run(
            [npx, "tsx", str(entry)],
            input=json.dumps(payload),
            capture_output=True,
            text=True,
            encoding="utf-8",
            cwd=str(tool_dir),
            timeout=IOTA_TIMEOUT_SECONDS,
            shell=False,
        )
    except subprocess.TimeoutExpired:
        return False, "iota-immutable-publisher timeout"
    except OSError as e:
        return False, str(e)
    if proc.returncode != 0:
        return False, (proc.stderr or proc.stdout or "iota publish failed").strip()
    try:
        body = json.loads(proc.stdout.strip() or "{}")
    except json.JSONDecodeError:
        return False, "invalid JSON from iota-immutable-publisher"
    return bool(body.get("success")), body.get("feedback", "ok")


def _dispatch_subscriber(
    repo: Path, subscriber: dict[str, Any], event: dict[str, Any]
) -> tuple[str, str]:
    agent = subscriber.get("agent")
    if not isinstance(agent, str) or not agent:
        return "unknown", "failed"
    tool = subscriber.get("tool")
    if tool == "iota-immutable-publisher":
        ok, _ = _invoke_iota_publisher(repo, event)
        return agent, "success" if ok else "failed"
    action = subscriber.get("action")
    if action:
        return agent, "failed"
    return agent, "failed"


def route_domain_event(event_file_path: str) -> dict[str, Any]:
    repo = _repo_root()
    processed = repo / ".SddIA" / "events" / "processed"
    dead_letter = repo / ".SddIA" / "events" / "dead-letter"
    processed.mkdir(parents=True, exist_ok=True)
    dead_letter.mkdir(parents=True, exist_ok=True)

    raw_path = Path(event_file_path)
    if not raw_path.is_absolute():
        event_path = (repo / raw_path).resolve()
    else:
        event_path = raw_path.resolve()

    if not event_path.is_file():
        _fail(f"event file not found: {event_path}")

    try:
        event = json.loads(event_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as e:
        _fail(f"invalid event JSON: {e}")

    event_type = event.get("event_type")
    if not isinstance(event_type, str) or not event_type:
        _fail("event_type missing")

    subs_path = repo / "SddIA" / "core" / "event-subscriptions.json"
    try:
        registry = json.loads(subs_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as e:
        _fail(f"cannot read event-subscriptions.json: {e}")

    subscribers = registry.get(event_type) or []
    delivery_status: dict[str, str] = {}

    if isinstance(subscribers, list):
        for sub in subscribers:
            if not isinstance(sub, dict):
                continue
            agent, status = _dispatch_subscriber(repo, sub, event)
            delivery_status[agent] = status

    event["delivery_state"] = {**event.get("delivery_state", {}), **delivery_status}
    all_success = not delivery_status or all(v == "success" for v in delivery_status.values())
    dest_dir = processed if all_success else dead_letter
    dest = dest_dir / event_path.name
    dest.write_text(json.dumps(event, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    event_path.unlink()

    result = {
        "success": all_success,
        "exitCode": 0 if all_success else 1,
        "data": {
            "success": all_success,
            "delivery_status": delivery_status,
            "target_path": _rel_event_path(repo, dest),
        },
    }
    if not all_success:
        result["error"] = "one or more subscribers failed"
    return result


def _run_route_cli() -> None:
    parser = argparse.ArgumentParser(description="route-domain-event (cápsula física)")
    parser.add_argument(
        "--event-file-path",
        required=True,
        help="Ruta relativa o absoluta al JSON en pending/",
    )
    args = parser.parse_args()
    out = route_domain_event(args.event_file_path)
    _emit_route_result(out)
    sys.exit(0 if out.get("exitCode") == 0 else 1)


def _run_watcher(*, once: bool = False) -> None:
    repo = _repo_root()
    pending = repo / ".SddIA" / "events" / "pending"
    pending.mkdir(parents=True, exist_ok=True)
    script = Path(__file__).resolve()
    attempts: dict[str, int] = {}
    in_flight: set[str] = set()

    print("[WATCHER] Iniciado. pending=", pending, flush=True)
    while True:
        for path in sorted(pending.glob("*.json")):
            key = path.name
            if key in in_flight:
                continue
            n = attempts.get(key, 0)
            if n >= MAX_ROUTE_ATTEMPTS:
                if path.is_file():
                    print(
                        f"[WATCHER] Skip {key}: max attempts ({MAX_ROUTE_ATTEMPTS})",
                        flush=True,
                    )
                continue

            rel = _rel_event_path(repo, path)
            print(f"[WATCHER] Detectado nuevo evento: {key}", flush=True)
            in_flight.add(key)
            attempts[key] = n + 1

            proc = subprocess.run(
                [
                    sys.executable,
                    str(script),
                    "--event-file-path",
                    rel,
                ],
                capture_output=True,
                text=True,
                encoding="utf-8",
                shell=False,
            )
            in_flight.discard(key)

            if proc.returncode != 0:
                print(
                    f"[WATCHER] route-domain-event falló ({key}): "
                    f"{(proc.stderr or proc.stdout or '').strip()}",
                    flush=True,
                )
            elif not path.is_file():
                attempts.pop(key, None)
            else:
                print(
                    f"[WATCHER] {key} sigue en pending tras enrutar (intento {attempts[key]})",
                    flush=True,
                )

        time.sleep(POLL_SECONDS)
        if once:
            print("[WATCHER] Ciclo único (--once). Fin.", flush=True)
            break


def main() -> None:
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
