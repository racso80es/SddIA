#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Triaje Argos: latido térmico Centinelas (CEN-05)."""

from __future__ import annotations

import json
import os
import uuid
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from daemon_kill_switch_core import list_indexed_daemon_ids
from eda_bus_utils import (
    _write_json_atomic,
    ensure_event_bus_topology,
    stamp_fractal_delivery_state,
)
from execute_process_core import parse_frontmatter
from governance_daemon_manager_core import (
    _daemons_dir,
    _load_cumulo,
    _pid_alive,
    _read_lock,
)

SUBSCRIBER_KEY = "argos.daemon-heartbeat-audit"
MISSED_CYCLES_THRESHOLD = 3


def _state_path(repo: Path) -> Path:
    cfg = _load_cumulo(repo)
    rel = (cfg.get("daemons_instance") or {}).get("state", ".SddIA/daemons/state")
    return repo / rel / "heartbeat-audit.json"


def _load_state(repo: Path) -> dict[str, Any]:
    path = _state_path(repo)
    if not path.is_file():
        return {"daemons": {}}
    try:
        body = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {"daemons": {}}
    if not isinstance(body, dict):
        return {"daemons": {}}
    daemons = body.get("daemons")
    if not isinstance(daemons, dict):
        body["daemons"] = {}
    return body


def _save_state(repo: Path, state: dict[str, Any]) -> None:
    path = _state_path(repo)
    path.parent.mkdir(parents=True, exist_ok=True)
    _write_json_atomic(path, state)


def _parse_iso(value: str | None) -> datetime | None:
    if not isinstance(value, str) or not value.strip():
        return None
    try:
        return datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        return None


def _daemon_interval(repo: Path, daemon_id: str) -> int:
    md = _daemons_dir(repo) / f"{daemon_id}.md"
    if not md.is_file():
        return 30
    fm = parse_frontmatter(md)
    execution = fm.get("execution") or {}
    try:
        return max(5, int(execution.get("heartbeat_interval_seconds", 30)))
    except (TypeError, ValueError):
        return 30


def _now_utc() -> datetime:
    return datetime.now(timezone.utc)


def _emit_system_fracture(
    repo: Path,
    *,
    daemon_id: str,
    daemon_uuid: str,
    missed_cycles: int,
    last_heartbeat_at: str | None,
) -> dict[str, str]:
    bus = ensure_event_bus_topology(repo)
    event_id = str(uuid.uuid4())
    error_trace = (
        f"Centinela {daemon_id} omitió {missed_cycles} ciclos consecutivos de Daemon_Heartbeat "
        f"(umbral={MISSED_CYCLES_THRESHOLD}). last_heartbeat={last_heartbeat_at or 'never'}"
    )
    event = {
        "event_id": event_id,
        "event_type": "System_Fracture_Detected",
        "timestamp": _now_utc().strftime("%Y-%m-%dT%H:%M:%SZ"),
        "emitter_agent": "argos",
        "payload": {
            "process_name": daemon_id,
            "error_trace": error_trace,
            "agent_emitter": "argos",
            "attempted_action": "daemon-heartbeat-audit",
            "daemon_uuid": daemon_uuid,
            "missed_cycles": missed_cycles,
        },
    }
    target = repo / bus["pending"] / f"{event_id}.json"
    _write_json_atomic(target, event)
    return {
        "event_id": event_id,
        "target_path": str(target.relative_to(repo)).replace("\\", "/"),
    }


def _resolve_daemon_uuid(repo: Path, daemon_id: str) -> str:
    md = _daemons_dir(repo) / f"{daemon_id}.md"
    if md.is_file():
        fm = parse_frontmatter(md)
        uid = fm.get("uuid")
        if isinstance(uid, str):
            return uid
    return ""


def _record_heartbeat(repo: Path, state: dict[str, Any], payload: dict[str, Any]) -> None:
    daemon_name = payload.get("daemon_name")
    if not isinstance(daemon_name, str) or not daemon_name.strip():
        return
    daemon_id = daemon_name.strip()
    daemons = state.setdefault("daemons", {})
    entry = daemons.get(daemon_id) if isinstance(daemons.get(daemon_id), dict) else {}
    entry["last_heartbeat_at"] = _now_utc().strftime("%Y-%m-%dT%H:%M:%SZ")
    entry["missed_cycles"] = 0
    entry["heartbeat_interval_seconds"] = _daemon_interval(repo, daemon_id)
    entry.pop("fracture_event_id", None)
    daemons[daemon_id] = entry


def _audit_running_daemon(
    repo: Path,
    state: dict[str, Any],
    daemon_id: str,
) -> dict[str, Any] | None:
    lock = _read_lock(repo, daemon_id)
    if not lock or not _pid_alive(int(lock.get("pid") or 0)):
        return None

    interval = _daemon_interval(repo, daemon_id)
    daemons = state.setdefault("daemons", {})
    entry = daemons.get(daemon_id) if isinstance(daemons.get(daemon_id), dict) else {}
    entry["heartbeat_interval_seconds"] = interval

    last_hb = _parse_iso(entry.get("last_heartbeat_at"))
    if last_hb is None:
        last_hb = _parse_iso(lock.get("started_at"))
    now = _now_utc()
    if last_hb is None:
        elapsed = interval * MISSED_CYCLES_THRESHOLD
    else:
        elapsed = max(0.0, (now - last_hb).total_seconds())

    missed = int(elapsed // interval)
    entry["missed_cycles"] = missed
    daemons[daemon_id] = entry

    if missed < MISSED_CYCLES_THRESHOLD:
        return None
    if entry.get("fracture_event_id"):
        return None

    seal = _emit_system_fracture(
        repo,
        daemon_id=daemon_id,
        daemon_uuid=_resolve_daemon_uuid(repo, daemon_id),
        missed_cycles=missed,
        last_heartbeat_at=entry.get("last_heartbeat_at"),
    )
    entry["fracture_event_id"] = seal["event_id"]
    daemons[daemon_id] = entry
    return seal


def audit_staleness(repo: Path) -> list[dict[str, Any]]:
    state = _load_state(repo)
    fractures: list[dict[str, Any]] = []
    for daemon_id in list_indexed_daemon_ids(repo):
        seal = _audit_running_daemon(repo, state, daemon_id)
        if seal:
            fractures.append(seal)
    _save_state(repo, state)
    return fractures


def audit_telemetry_file(repo: Path, rel_path: str) -> dict[str, Any]:
    event_path = (repo / rel_path.strip()).resolve()
    if not event_path.is_file():
        return {"ok": False, "error": f"no existe: {rel_path}"}
    try:
        body = json.loads(event_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        return {"ok": False, "error": str(exc)}

    if body.get("event_type") != "Daemon_Heartbeat":
        stamp_fractal_delivery_state(repo, event_path, SUBSCRIBER_KEY, "skipped")
        return {"ok": True, "status": "skipped", "reason": "wrong_event_type"}

    payload = body.get("payload") or {}
    if not isinstance(payload, dict):
        return {"ok": False, "error": "payload invalido"}

    state = _load_state(repo)
    _record_heartbeat(repo, state, payload)
    _save_state(repo, state)

    fractures = audit_staleness(repo)
    stamp_fractal_delivery_state(repo, event_path, SUBSCRIBER_KEY, "success")
    return {
        "ok": True,
        "status": "audited",
        "fractures_emitted": fractures,
        "daemon_name": payload.get("daemon_name"),
    }


def run_daemon_heartbeat_audit(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    rel = inputs.get("event_file_path")
    if isinstance(rel, str) and rel.strip():
        result = audit_telemetry_file(repo, rel.strip())
        ok = bool(result.get("ok"))
        return {
            "success": ok,
            "exitCode": 0 if ok else 1,
            "data": result,
            "error": None if ok else result.get("error"),
        }

    fractures = audit_staleness(repo)
    return {
        "success": True,
        "exitCode": 0,
        "data": {"status": "sweep", "fractures_emitted": fractures},
        "error": None,
    }
