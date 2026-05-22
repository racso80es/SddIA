#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Recolector inerte del bus EDA V3: purga padres completados y alerta dead-letter.

Escanea .events/pending/ y cruza suscriptores requeridos (event-subscriptions.json)
con testigos en subscribers/processed/ y subscribers/dead-letter/.

Uso:
  python SddIA/scripts/daemons/event-sweeper.py --once
  python SddIA/scripts/daemons/event-sweeper.py          # bucle continuo
"""

from __future__ import annotations

import argparse
import json
import sys
import time
from pathlib import Path
from typing import Any

_SCRIPT_DIR = Path(__file__).resolve().parent
_QA_DIR = _SCRIPT_DIR.parent / "qa"
if str(_QA_DIR) not in sys.path:
    sys.path.insert(0, str(_QA_DIR))

from eda_bus_utils import (  # noqa: E402
    archive_processed_witnesses,
    ensure_event_bus_topology,
    list_witnesses,
    required_subscriber_ids,
)

POLL_SECONDS = 5


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    print("[SWEEPER] No se encontró raíz del workspace", file=sys.stderr)
    sys.exit(1)


def _load_registry(repo: Path, bus: dict[str, str]) -> dict[str, Any]:
    path = repo / bus["subscriptions"]
    return json.loads(path.read_text(encoding="utf-8-sig"))


def _processed_subscriber_names(repo: Path, bus: dict[str, str], event_uuid: str) -> set[str]:
    names: set[str] = set()
    for path in list_witnesses(repo, bus, "subscriber_processed", event_uuid):
        # {uuid}.{subscriber}.json
        suffix = path.name[len(event_uuid) + 1 : -5]
        if suffix:
            names.add(suffix)
    return names


def _emit_kaizen_alert(event_uuid: str, event_type: str, witnesses: list[Path]) -> None:
    details = []
    for path in witnesses:
        try:
            body = json.loads(path.read_text(encoding="utf-8"))
            details.append(
                {
                    "witness": path.name,
                    "subscriber": body.get("subscriber"),
                    "error_trace": body.get("error_trace"),
                }
            )
        except (OSError, json.JSONDecodeError):
            details.append({"witness": path.name})
    alert = {
        "alert_type": "kaizen_eda_dead_letter",
        "event_uuid": event_uuid,
        "event_type": event_type,
        "message": "Testigo en dead-letter — padre NO purgado",
        "witnesses": details,
    }
    print(json.dumps(alert, ensure_ascii=False), file=sys.stderr, flush=True)


def sweep_once(repo: Path) -> dict[str, Any]:
    bus = ensure_event_bus_topology(repo)
    pending_dir = repo / bus["pending"]
    registry = _load_registry(repo, bus)
    report: dict[str, Any] = {
        "purged": [],
        "kaizen_alerts": [],
        "skipped": [],
    }

    if not pending_dir.is_dir():
        return report

    for parent_path in sorted(pending_dir.glob("*.json")):
        event_uuid = parent_path.stem
        try:
            event = json.loads(parent_path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            report["skipped"].append({"event_uuid": event_uuid, "reason": "invalid-json"})
            continue

        event_type = event.get("event_type")
        if not isinstance(event_type, str) or not event_type:
            report["skipped"].append({"event_uuid": event_uuid, "reason": "missing-event_type"})
            continue

        dead = list_witnesses(repo, bus, "subscriber_dead_letter", event_uuid)
        if dead:
            _emit_kaizen_alert(event_uuid, event_type, dead)
            report["kaizen_alerts"].append(event_uuid)
            continue

        required = required_subscriber_ids(registry, event_type)
        if not required:
            report["skipped"].append({"event_uuid": event_uuid, "reason": "no-subscribers"})
            continue

        done = _processed_subscriber_names(repo, bus, event_uuid)
        if set(required).issubset(done):
            parent_path.unlink(missing_ok=True)
            archived = archive_processed_witnesses(repo, bus, event_uuid)
            report["purged"].append({"event_uuid": event_uuid, "witnesses_archived": archived})
        else:
            pending = sorted(set(required) - done)
            report["skipped"].append(
                {"event_uuid": event_uuid, "reason": "awaiting-subscribers", "pending": pending}
            )

    return report


def main() -> None:
    parser = argparse.ArgumentParser(description="event-sweeper — recolector bus EDA V3")
    parser.add_argument("--once", action="store_true", help="Un solo ciclo de barrido")
    parser.add_argument("--json", action="store_true", help="Imprimir informe JSON en stdout")
    args = parser.parse_args()

    repo = _repo_root()
    print("[SWEEPER] Iniciado.", flush=True)

    while True:
        report = sweep_once(repo)
        if args.json:
            print(json.dumps(report, ensure_ascii=False), flush=True)
        else:
            if report["purged"]:
                print(f"[SWEEPER] Purgados: {report['purged']}", flush=True)
            if report["kaizen_alerts"]:
                print(f"[SWEEPER] Alertas Kaizen: {report['kaizen_alerts']}", flush=True)

        if args.once:
            print("[SWEEPER] Ciclo único (--once). Fin.", flush=True)
            break
        time.sleep(POLL_SECONDS)


if __name__ == "__main__":
    main()
