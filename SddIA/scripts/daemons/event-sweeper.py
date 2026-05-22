#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Recolector inerte del bus EDA V3+: purga padres completados y alerta dead-letter."""

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
    ensure_event_bus_topology,
    list_witnesses,
    try_sweep_event,
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
    report: dict[str, Any] = {"purged": [], "kaizen_alerts": [], "skipped": []}

    if not pending_dir.is_dir():
        return report

    for parent_path in sorted(pending_dir.glob("*.json")):
        event_uuid = parent_path.stem
        result = try_sweep_event(repo, bus, event_uuid, registry=registry)
        status = result.get("status")

        if status == "purged":
            report["purged"].append(
                {
                    "event_uuid": event_uuid,
                    "witnesses": result.get("witnesses", 0),
                    "headers": result.get("headers", 0),
                    "pending": result.get("pending", 0),
                }
            )
            continue

        if status == "kaizen":
            dead = list_witnesses(repo, bus, "dead_letter_subscribers", event_uuid)
            _emit_kaizen_alert(
                event_uuid,
                str(result.get("event_type") or ""),
                dead,
            )
            report["kaizen_alerts"].append(event_uuid)
            continue

        if status in ("invalid-json", "missing-event_type", "no-subscribers", "absent", "invalid-registry"):
            report["skipped"].append({"event_uuid": event_uuid, "reason": status})
            continue

        if status == "in-flight":
            report["skipped"].append(
                {
                    "event_uuid": event_uuid,
                    "reason": "subscribers-in-flight",
                    "in_flight": result.get("in_flight", []),
                }
            )
            continue

        if status == "awaiting":
            report["skipped"].append(
                {
                    "event_uuid": event_uuid,
                    "reason": "awaiting-subscribers",
                    "pending": result.get("pending_subscribers", []),
                }
            )

    return report


def main() -> None:
    parser = argparse.ArgumentParser(description="event-sweeper — recolector bus EDA V3+")
    parser.add_argument("--once", action="store_true")
    parser.add_argument("--json", action="store_true")
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
