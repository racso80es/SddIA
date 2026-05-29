#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Triaje lab: purga JSON estancados en colas fractal activas (.events/)."""

from __future__ import annotations

import argparse
import json
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from eda_bus_utils import (
    delivery_stamp_terminal_ok,
    ensure_event_bus_topology,
    ensure_fractal_bus_topology,
    list_witnesses,
    load_eda_bus,
    load_eda_fractal,
    required_subscriber_ids,
    safe_remove_path,
)


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace")


def _parse_timestamp(ts: str) -> datetime | None:
    if not isinstance(ts, str) or not ts.strip():
        return None
    text = ts.strip().replace("Z", "+00:00")
    try:
        return datetime.fromisoformat(text)
    except ValueError:
        return None


def _load_registry(repo: Path, subscriptions_rel: str) -> dict[str, Any]:
    path = repo / subscriptions_rel
    return json.loads(path.read_text(encoding="utf-8-sig"))


def _delivery_complete(body: dict[str, Any], registry: dict[str, Any]) -> bool:
    event_type = body.get("event_type")
    if not isinstance(event_type, str) or not event_type:
        return False
    required = required_subscriber_ids(registry, event_type)
    if not required:
        return False
    ds = body.get("delivery_state")
    if not isinstance(ds, dict):
        return False
    for sid in required:
        st = ds.get(sid)
        if not isinstance(st, str) or not delivery_stamp_terminal_ok(st):
            return False
    return True


def _has_processed_witnesses(repo: Path, bus: dict[str, str], event_uuid: str) -> bool:
    return bool(list_witnesses(repo, bus, "processed_subscribers", event_uuid))


def _has_in_flight_witnesses(repo: Path, bus: dict[str, str], event_uuid: str) -> bool:
    return bool(list_witnesses(repo, bus, "processing_subscribers", event_uuid))


def _has_dead_letter_witnesses(repo: Path, bus: dict[str, str], event_uuid: str) -> bool:
    return bool(list_witnesses(repo, bus, "dead_letter_subscribers", event_uuid))


def _scan_fractal_queue(
    repo: Path,
    bus: dict[str, str],
    fractal: dict[str, str],
    *,
    max_age_hours: float | None,
) -> list[dict[str, Any]]:
    families = (
        ("telemetry", fractal["telemetry"], fractal["telemetry_subscriptions"]),
        ("orchestration", fractal["orchestration"], fractal["orchestration_subscriptions"]),
        ("domain", fractal["domain"], fractal["domain_subscriptions"]),
    )
    now = datetime.now(timezone.utc)
    candidates: list[dict[str, Any]] = []

    for queue, rel_dir, subs_rel in families:
        queue_path = repo / rel_dir
        if not queue_path.is_dir():
            continue
        try:
            registry = _load_registry(repo, subs_rel)
        except (OSError, json.JSONDecodeError):
            continue
        for path in sorted(queue_path.glob("*.json")):
            event_uuid = path.stem
            if _has_dead_letter_witnesses(repo, bus, event_uuid):
                continue
            if _has_in_flight_witnesses(repo, bus, event_uuid):
                continue
            try:
                body = json.loads(path.read_text(encoding="utf-8"))
            except (OSError, json.JSONDecodeError):
                continue

            reasons: list[str] = []
            if _delivery_complete(body, registry):
                reasons.append("delivery_complete")
            if _has_processed_witnesses(repo, bus, event_uuid):
                reasons.append("orphan_with_processed_witnesses")

            ts = body.get("timestamp")
            if isinstance(ts, str) and max_age_hours is not None:
                parsed = _parse_timestamp(ts)
                if parsed is not None:
                    age_h = (now - parsed).total_seconds() / 3600.0
                    if age_h >= max_age_hours:
                        reasons.append(f"max_age_hours>={max_age_hours}")

            if not reasons:
                continue
            candidates.append(
                {
                    "event_id": event_uuid,
                    "queue": queue,
                    "path": path.relative_to(repo).as_posix(),
                    "reason": "+".join(reasons),
                }
            )
    return candidates


def purge_stale_events(
    repo: Path,
    *,
    apply: bool = False,
    max_age_hours: float | None = None,
) -> dict[str, Any]:
    bus = ensure_event_bus_topology(repo)
    fractal = ensure_fractal_bus_topology(repo)
    candidates = _scan_fractal_queue(repo, bus, fractal, max_age_hours=max_age_hours)
    purged = 0
    errors: list[dict[str, str]] = []

    for item in candidates:
        path = repo / item["path"]
        if not apply:
            continue
        if safe_remove_path(path):
            purged += 1
        else:
            errors.append({"event_id": item["event_id"], "error": "safe_remove_path failed"})

    return {
        "dry_run": not apply,
        "scanned_queues": ["telemetry", "orchestration", "domain"],
        "candidates": candidates,
        "candidate_count": len(candidates),
        "purged": purged,
        "errors": errors,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description="Purge stale events en colas fractal (lab)")
    parser.add_argument("--apply", action="store_true", help="Ejecutar purga (default: dry-run)")
    parser.add_argument("--json", action="store_true", help="Salida JSON")
    parser.add_argument("--max-age-hours", type=float, default=None, help="Candidato por antigüedad")
    args = parser.parse_args()

    repo = _repo_root()
    report = purge_stale_events(
        repo,
        apply=args.apply,
        max_age_hours=args.max_age_hours,
    )
    if args.json:
        sys.stdout.write(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
    else:
        mode = "APPLY" if args.apply else "DRY-RUN"
        print(f"[purge_stale_events] {mode} candidates={report['candidate_count']}")
        for item in report["candidates"]:
            print(f"  - {item['queue']}/{item['event_id']}: {item['reason']}")
        if args.apply:
            print(f"[purge_stale_events] purged={report['purged']} errors={len(report['errors'])}")
    sys.exit(0)


if __name__ == "__main__":
    main()
