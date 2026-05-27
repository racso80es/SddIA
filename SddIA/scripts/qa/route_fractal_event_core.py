# -*- coding: utf-8 -*-
"""Enrutador bus EDA fractal (telemetry / orchestration / domain)."""

from __future__ import annotations

import json
import os
from pathlib import Path
from typing import Any

from eda_bus_utils import load_eda_fractal, subscriber_id
from route_domain_event_core import dispatch_subscriber


def _rel_event_path(repo: Path, event_path: Path) -> str:
    try:
        return event_path.resolve().relative_to(repo.resolve()).as_posix()
    except ValueError:
        return str(event_path.resolve())


def _dispatch_fractal_subscriber(
    repo: Path, subscriber: dict[str, Any], event: dict[str, Any], rel_path: str
) -> tuple[str, str, str | None, int]:
    process_name = subscriber.get("process")
    if isinstance(process_name, str) and process_name.strip() == "telemetry-batch-stub":
        process_inputs = {
            "event_file_path": rel_path,
            "correlation_id": event.get("event_id") or "",
        }
        try:
            from execute_process_capsules import run_process

            envelope = run_process(repo, "telemetry-batch-stub", process_inputs)
        except Exception as exc:
            return subscriber_id(subscriber), "failed", str(exc), 1
        exit_code = int(envelope.get("status_code", 1 if not envelope.get("success") else 0))
        ok = bool(envelope.get("success")) and exit_code == 0
        sid = subscriber_id(subscriber)
        if ok:
            return sid, "success", None, 0
        return sid, "failed", envelope.get("error") or "telemetry-batch-stub failed", exit_code

    return dispatch_subscriber(repo, subscriber, event)


def route_fractal_event(
    repo: Path,
    event_file_path: str,
    subscriptions_rel: str,
    *,
    purge_after: bool = False,
    skip_ecst_gate: bool = False,
) -> dict[str, Any]:
    raw_path = Path(event_file_path)
    event_path = (repo / raw_path).resolve() if not raw_path.is_absolute() else raw_path.resolve()

    if not event_path.is_file():
        return {
            "success": False,
            "exitCode": 1,
            "data": None,
            "error": f"event file not found: {event_path}",
        }

    try:
        event = json.loads(event_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        return {
            "success": False,
            "exitCode": 1,
            "data": None,
            "error": f"invalid event JSON: {exc}",
        }

    event_type = event.get("event_type")
    if not isinstance(event_type, str) or not event_type:
        return {"success": False, "exitCode": 1, "data": None, "error": "event_type missing"}

    subs_path = repo / subscriptions_rel
    try:
        registry = json.loads(subs_path.read_text(encoding="utf-8-sig"))
    except (OSError, json.JSONDecodeError) as exc:
        return {
            "success": False,
            "exitCode": 1,
            "data": None,
            "error": f"cannot read subscriptions: {exc}",
        }

    subscribers = [sub for sub in (registry.get(event_type) or []) if isinstance(sub, dict)]
    rel_path = _rel_event_path(repo, event_path)
    delivery_status: dict[str, str] = {}
    all_ok = True

    if not subscribers:
        if purge_after:
            event_path.unlink(missing_ok=True)
        return {
            "success": True,
            "exitCode": 0,
            "data": {
                "success": True,
                "delivery_status": {},
                "parent_path": rel_path,
                "purged": purge_after,
                "skip_ecst_gate": skip_ecst_gate,
            },
        }

    sync = os.environ.get("SDDIA_LAB_ROUTE_SYNC", "").strip().lower() in ("1", "true", "yes")
    if sync:
        for sub in subscribers:
            sid, status, err, _code = _dispatch_fractal_subscriber(repo, sub, event, rel_path)
            delivery_status[sid] = status
            if status not in ("success", "skipped-topology", "skipped-backfill", "skipped-pre-anchored", "skipped-dlt-threshold"):
                all_ok = False
    else:
        for sub in subscribers:
            sid, status, _err, _code = _dispatch_fractal_subscriber(repo, sub, event, rel_path)
            delivery_status[sid] = status
            if status not in ("success", "skipped-topology", "skipped-backfill", "skipped-pre-anchored", "skipped-dlt-threshold"):
                all_ok = False

    if all_ok and purge_after and event_path.is_file():
        event_path.unlink(missing_ok=True)

    return {
        "success": all_ok,
        "exitCode": 0 if all_ok else 1,
        "data": {
            "success": all_ok,
            "delivery_status": delivery_status,
            "parent_path": rel_path,
            "purged": purge_after and all_ok,
            "skip_ecst_gate": skip_ecst_gate,
        },
        "error": None if all_ok else "one or more subscribers failed",
    }


def route_telemetry_event(repo: Path, event_file_path: str) -> dict[str, Any]:
    fractal = load_eda_fractal(repo)
    return route_fractal_event(
        repo,
        event_file_path,
        fractal["telemetry_subscriptions"],
        purge_after=False,
        skip_ecst_gate=True,
    )


def route_orchestration_event(repo: Path, event_file_path: str) -> dict[str, Any]:
    fractal = load_eda_fractal(repo)
    return route_fractal_event(
        repo,
        event_file_path,
        fractal["orchestration_subscriptions"],
        purge_after=True,
        skip_ecst_gate=False,
    )


def route_domain_fractal_event(repo: Path, event_file_path: str) -> dict[str, Any]:
    fractal = load_eda_fractal(repo)
    return route_fractal_event(
        repo,
        event_file_path,
        fractal["domain_subscriptions"],
        purge_after=False,
        skip_ecst_gate=False,
    )
