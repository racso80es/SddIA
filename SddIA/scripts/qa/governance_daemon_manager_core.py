#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Handler laboratorio: governance-daemon-manager (CEN-02)."""

from __future__ import annotations

import json
import os
import signal
import subprocess
import sys
import time
import uuid
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from eda_bus_utils import build_process_execution_completed_event, write_fractal_event
from execute_process_core import parse_frontmatter


def _load_cumulo(repo: Path) -> dict[str, Any]:
    path = repo / "SddIA" / "core" / "cumulo.paths.json"
    return json.loads(path.read_text(encoding="utf-8"))


def _daemons_dir(repo: Path) -> Path:
    cfg = _load_cumulo(repo)
    rel = (cfg.get("directories") or {}).get("daemons", "SddIA/daemons")
    return repo / rel


def _status_dir(repo: Path) -> Path:
    cfg = _load_cumulo(repo)
    rel = (cfg.get("daemons_instance") or {}).get("status", ".SddIA/daemons/status")
    return repo / rel


def _lock_path(repo: Path, daemon_id: str) -> Path:
    return _status_dir(repo) / f"{daemon_id}.lock"


def _pid_alive(pid: int) -> bool:
    if pid <= 0:
        return False
    try:
        os.kill(pid, 0)
    except OSError:
        return False
    return True


def _read_lock(repo: Path, daemon_id: str) -> dict[str, Any] | None:
    path = _lock_path(repo, daemon_id)
    if not path.is_file():
        return None
    try:
        body = json.loads(path.read_text(encoding="utf-8"))
        return body if isinstance(body, dict) else None
    except (OSError, json.JSONDecodeError):
        return None


def _write_lock(
    repo: Path,
    daemon_id: str,
    pid: int,
    heartbeat_interval_seconds: int,
) -> Path:
    status = _status_dir(repo)
    status.mkdir(parents=True, exist_ok=True)
    path = _lock_path(repo, daemon_id)
    payload = {
        "daemon_name": daemon_id,
        "pid": pid,
        "started_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "heartbeat_interval_seconds": heartbeat_interval_seconds,
    }
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8")
    return path


def _remove_lock(repo: Path, daemon_id: str) -> bool:
    path = _lock_path(repo, daemon_id)
    if path.is_file():
        path.unlink()
        return True
    return False


def _resolve_daemon(repo: Path, daemon_id: str) -> dict[str, Any]:
    daemons = _daemons_dir(repo)
    md_path = daemons / f"{daemon_id}.md"
    if not md_path.is_file():
        raise FileNotFoundError(f"Centinela no definido: {daemon_id}")
    fm = parse_frontmatter(md_path)
    execution = fm.get("execution") or {}
    if not isinstance(execution, dict):
        execution = {}
    entrypoint = execution.get("entrypoint")
    runtime = execution.get("runtime")
    if not isinstance(entrypoint, str) or not entrypoint.strip():
        raise ValueError(f"execution.entrypoint inválido en {daemon_id}")
    if not isinstance(runtime, str) or not runtime.strip():
        raise ValueError(f"execution.runtime inválido en {daemon_id}")
    heartbeat = execution.get("heartbeat_interval_seconds", 30)
    try:
        heartbeat = int(heartbeat)
    except (TypeError, ValueError):
        heartbeat = 30
    return {
        "daemon_id": daemon_id,
        "daemon_uuid": str(fm.get("uuid") or ""),
        "entrypoint": entrypoint.strip(),
        "runtime": runtime.strip(),
        "heartbeat_interval_seconds": max(5, heartbeat),
        "md_path": str(md_path.relative_to(repo)).replace("\\", "/"),
    }


def _emit_orchestration(
    repo: Path,
    *,
    asset_id: str,
    workspace_path: str | None,
    status: str,
    operation: str,
    daemon_id: str,
    daemon_uuid: str,
    operation_status: str,
    os_result: dict[str, Any],
) -> dict[str, str]:
    event_id = str(uuid.uuid4())
    base = build_process_execution_completed_event(
        event_id=event_id,
        asset_id=asset_id,
        process_name="governance-daemon-manager",
        status=status,
        workspace_path=workspace_path,
    )
    payload = base.get("payload") or {}
    payload["operation"] = operation
    payload["daemon_id"] = daemon_id
    payload["daemon_uuid"] = daemon_uuid
    payload["operation_status"] = operation_status
    payload["os_result"] = os_result
    base["payload"] = payload
    base["emitter_agent"] = "governance-daemon-manager"
    return write_fractal_event(repo, base, "orchestration")


def _kill_pid(pid: int, grace_seconds: int) -> dict[str, Any]:
    signals: list[str] = []
    if not _pid_alive(pid):
        return {"pid": pid, "alive": False, "signal_sequence": signals, "exit_code": None}
    try:
        os.kill(pid, signal.SIGTERM)
        signals.append("SIGTERM")
    except OSError as exc:
        return {"pid": pid, "alive": False, "error": str(exc), "signal_sequence": signals}
    deadline = time.monotonic() + max(1, grace_seconds)
    while time.monotonic() < deadline:
        if not _pid_alive(pid):
            return {"pid": pid, "alive": False, "signal_sequence": signals, "exit_code": 0}
        time.sleep(0.2)
    if _pid_alive(pid):
        try:
            os.kill(pid, signal.SIGKILL)
            signals.append("SIGKILL")
        except OSError as exc:
            return {"pid": pid, "alive": _pid_alive(pid), "error": str(exc), "signal_sequence": signals}
        time.sleep(0.3)
    return {
        "pid": pid,
        "alive": _pid_alive(pid),
        "signal_sequence": signals,
        "exit_code": 0 if not _pid_alive(pid) else None,
    }


def run_governance_daemon_manager(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    operation = inputs.get("operation")
    daemon_id = inputs.get("daemon_id")
    if operation not in ("start", "status", "kill"):
        return {"success": False, "exitCode": 1, "error": "operation debe ser start|status|kill"}
    if not isinstance(daemon_id, str) or not daemon_id.strip():
        return {"success": False, "exitCode": 1, "error": "daemon_id requerido"}
    daemon_id = daemon_id.strip()
    grace = inputs.get("kill_grace_seconds", 10)
    try:
        grace = int(grace)
    except (TypeError, ValueError):
        grace = 10

    asset_id = str(inputs.get("asset_id") or uuid.uuid4())
    workspace_path = inputs.get("repository_path") or inputs.get("workspace_path")
    if isinstance(workspace_path, Path):
        workspace_path = str(workspace_path)

    try:
        spec = _resolve_daemon(repo, daemon_id)
    except (FileNotFoundError, ValueError) as exc:
        return {"success": False, "exitCode": 1, "error": str(exc)}

    entry = repo / spec["entrypoint"]
    if not entry.is_file():
        return {
            "success": False,
            "exitCode": 1,
            "error": f"entrypoint no encontrado: {spec['entrypoint']}",
        }

    lock_rel = str(_lock_path(repo, daemon_id).relative_to(repo)).replace("\\", "/")
    os_result: dict[str, Any] = {
        "entrypoint_resolved": spec["entrypoint"],
        "runtime": spec["runtime"],
        "lock_path": lock_rel,
    }
    operation_status = "failed"
    success = False

    if operation == "start":
        existing = _read_lock(repo, daemon_id)
        if existing and _pid_alive(int(existing.get("pid") or 0)):
            os_result.update({"pid": existing["pid"], "alive": True})
            operation_status = "noop"
            success = True
        else:
            if existing:
                _remove_lock(repo, daemon_id)
            proc = subprocess.Popen(
                [spec["runtime"], str(entry)],
                cwd=str(repo),
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                start_new_session=True,
            )
            _write_lock(repo, daemon_id, proc.pid, spec["heartbeat_interval_seconds"])
            os_result.update({"pid": proc.pid, "alive": True})
            operation_status = "succeeded"
            success = True

    elif operation == "status":
        lock = _read_lock(repo, daemon_id)
        if not lock:
            os_result.update({"alive": False, "pid": None})
        else:
            pid = int(lock.get("pid") or 0)
            alive = _pid_alive(pid)
            os_result.update(
                {
                    "pid": pid,
                    "alive": alive,
                    "started_at": lock.get("started_at"),
                    "heartbeat_interval_seconds": lock.get("heartbeat_interval_seconds"),
                }
            )
            if not alive:
                _remove_lock(repo, daemon_id)
        operation_status = "succeeded"
        success = True

    elif operation == "kill":
        lock = _read_lock(repo, daemon_id)
        if not lock or not _pid_alive(int(lock.get("pid") or 0)):
            _remove_lock(repo, daemon_id)
            os_result.update({"pid": lock.get("pid") if lock else None, "alive": False})
            operation_status = "noop"
            success = True
        else:
            pid = int(lock["pid"])
            kill_out = _kill_pid(pid, grace)
            os_result.update(kill_out)
            if not kill_out.get("alive", True):
                _remove_lock(repo, daemon_id)
                os_result["lock_removed"] = True
            operation_status = "succeeded" if not kill_out.get("alive", True) else "failed"
            success = operation_status != "failed"

    orch_status = "success" if success else "failed"
    try:
        orch = _emit_orchestration(
            repo,
            asset_id=asset_id,
            workspace_path=str(workspace_path) if workspace_path else None,
            status=orch_status,
            operation=operation,
            daemon_id=daemon_id,
            daemon_uuid=spec["daemon_uuid"],
            operation_status=operation_status,
            os_result=os_result,
        )
    except Exception as exc:
        orch = {"error": str(exc)}

    return {
        "success": success,
        "exitCode": 0 if success else 1,
        "data": {
            "operation": operation,
            "daemon_id": daemon_id,
            "daemon_uuid": spec["daemon_uuid"],
            "operation_status": operation_status,
            "os_result": os_result,
            "orchestration_event_id": orch.get("event_id"),
            "orchestration_event_path": orch.get("target_path"),
        },
        "error": None if success else f"operación {operation} falló",
    }
