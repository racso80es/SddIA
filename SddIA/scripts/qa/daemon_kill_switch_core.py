#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Kill-Switch global Centinelas (CEN-03)."""

from __future__ import annotations

import atexit
import json
import signal
import uuid
from pathlib import Path
from typing import Any

from eda_bus_utils import build_process_execution_completed_event, write_fractal_event
from governance_daemon_manager_core import (
    _daemons_dir,
    _load_cumulo,
    _lock_path,
    _pid_alive,
    _read_lock,
    _remove_lock,
    run_governance_daemon_manager,
)

_HOOK_REGISTERED = False
_REPO_FOR_HOOK: Path | None = None


def list_indexed_daemon_ids(repo: Path) -> list[str]:
    daemons = _daemons_dir(repo)
    skip = frozenset({"index", "daemons-contract"})
    ids: list[str] = []
    for md in sorted(daemons.glob("*.md")):
        if md.stem in skip:
            continue
        ids.append(md.stem)
    return ids


def run_daemon_kill_switch(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    grace = inputs.get("kill_grace_seconds", 10)
    try:
        grace = int(grace)
    except (TypeError, ValueError):
        grace = 10

    asset_id = str(inputs.get("asset_id") or uuid.uuid4())
    workspace_path = inputs.get("repository_path") or inputs.get("workspace_path")
    daemon_ids = list_indexed_daemon_ids(repo)
    purge_report: list[dict[str, Any]] = []
    all_ok = True

    for daemon_id in daemon_ids:
        entry: dict[str, Any] = {"daemon_id": daemon_id}
        lock = _read_lock(repo, daemon_id)
        if lock and _pid_alive(int(lock.get("pid") or 0)):
            out = run_governance_daemon_manager(
                repo,
                {
                    "operation": "kill",
                    "daemon_id": daemon_id,
                    "kill_grace_seconds": grace,
                    "asset_id": asset_id,
                    "repository_path": workspace_path,
                },
            )
            entry["kill"] = out.get("data") or {}
            if not out.get("success"):
                all_ok = False
        else:
            if lock:
                _remove_lock(repo, daemon_id)
            entry["kill"] = {"operation_status": "noop", "reason": "no live lock"}
        purge_report.append(entry)

    stale_locks: list[str] = []
    status_root = repo / (_load_cumulo(repo).get("daemons_instance") or {}).get(
        "status", ".SddIA/daemons/status"
    )
    if status_root.is_dir():
        for lock_file in status_root.glob("*.lock"):
            body = _read_lock(repo, lock_file.stem)
            if body and not _pid_alive(int(body.get("pid") or 0)):
                _remove_lock(repo, lock_file.stem)
                stale_locks.append(lock_file.stem)

    event_id = str(uuid.uuid4())
    status = "success" if all_ok else "failed"
    event = build_process_execution_completed_event(
        event_id=event_id,
        asset_id=asset_id,
        process_name="daemon-kill-switch",
        status=status,
        workspace_path=str(workspace_path) if workspace_path else None,
    )
    payload = event.get("payload") or {}
    payload["purge_report"] = purge_report
    payload["stale_locks_removed"] = stale_locks
    payload["daemon_count"] = len(daemon_ids)
    event["payload"] = payload
    event["emitter_agent"] = "daemon-kill-switch"
    try:
        orch = write_fractal_event(repo, event, "orchestration")
    except Exception as exc:
        orch = {"error": str(exc)}

    return {
        "success": all_ok,
        "exitCode": 0 if all_ok else 1,
        "data": {
            "purge_report": purge_report,
            "stale_locks_removed": stale_locks,
            "orchestration_event_id": orch.get("event_id"),
            "orchestration_event_path": orch.get("target_path"),
        },
        "error": None if all_ok else "purga parcial fallida",
    }


def _shutdown_purge() -> None:
    if _REPO_FOR_HOOK is None:
        return
    try:
        run_daemon_kill_switch(_REPO_FOR_HOOK, {"repository_path": str(_REPO_FOR_HOOK)})
    except Exception:
        pass


def register_kill_switch_hooks(repo: Path) -> None:
    global _HOOK_REGISTERED, _REPO_FOR_HOOK
    if _HOOK_REGISTERED:
        return
    _REPO_FOR_HOOK = repo
    atexit.register(_shutdown_purge)

    def _handler(signum: int, _frame: Any) -> None:
        _shutdown_purge()
        signal.signal(signum, signal.SIG_DFL)
        signal.raise_signal(signum)

    for sig in (signal.SIGTERM, signal.SIGINT):
        try:
            signal.signal(sig, _handler)
        except (ValueError, OSError):
            pass
    _HOOK_REGISTERED = True
