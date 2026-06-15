#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Runtime Centinela: lock, Daemon_Heartbeat, shutdown limpio."""

from __future__ import annotations

import atexit
import json
import os
import signal
import sys
import time
import uuid
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from eda_bus_utils import ensure_fractal_bus_topology, write_fractal_event
from execute_process_core import parse_frontmatter

_REGISTRY: dict[str, "CentinelaRuntime"] = {}


def _load_cumulo(repo: Path) -> dict[str, Any]:
    path = repo / "SddIA" / "core" / "cumulo.paths.json"
    return json.loads(path.read_text(encoding="utf-8"))


def _status_dir(repo: Path) -> Path:
    cfg = _load_cumulo(repo)
    rel = (cfg.get("daemons_instance") or {}).get("status", ".SddIA/daemons/status")
    return repo / rel


def _daemons_dir(repo: Path) -> Path:
    cfg = _load_cumulo(repo)
    rel = (cfg.get("directories") or {}).get("daemons", "SddIA/daemons")
    return repo / rel


def _pid_alive(pid: int) -> bool:
    if pid <= 0:
        return False
    try:
        os.kill(pid, 0)
    except OSError:
        return False
    return True


def _load_daemon_spec(repo: Path, daemon_name: str) -> dict[str, Any]:
    md = _daemons_dir(repo) / f"{daemon_name}.md"
    if not md.is_file():
        return {
            "uuid": "",
            "heartbeat_interval_seconds": 30,
        }
    fm = parse_frontmatter(md)
    execution = fm.get("execution") or {}
    try:
        interval = max(5, int(execution.get("heartbeat_interval_seconds", 30)))
    except (TypeError, ValueError):
        interval = 30
    return {
        "uuid": str(fm.get("uuid") or ""),
        "heartbeat_interval_seconds": interval,
    }


class CentinelaRuntime:
    def __init__(self, repo: Path, daemon_name: str) -> None:
        self.repo = repo
        self.daemon_name = daemon_name
        self._spec = _load_daemon_spec(repo, daemon_name)
        self._started_at = time.monotonic()
        self._started_iso = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
        self._last_emit_monotonic = 0.0
        self._last_stimulus_at: str | None = None
        self._bootstrapped = False

    @property
    def lock_path(self) -> Path:
        return _status_dir(self.repo) / f"{self.daemon_name}.lock"

    def bootstrap(self) -> None:
        if self._bootstrapped:
            return
        existing = self._read_lock()
        if existing:
            other_pid = int(existing.get("pid") or 0)
            if other_pid != os.getpid() and _pid_alive(other_pid):
                print(
                    f"[{self.daemon_name}] lock activo pid={other_pid}; abortando duplicado",
                    file=sys.stderr,
                    flush=True,
                )
                sys.exit(1)
        self._write_lock()
        self._register_hooks()
        self.emit_heartbeat(force=True)
        self._bootstrapped = True
        _REGISTRY[self.daemon_name] = self

    def note_stimulus(self) -> None:
        self._last_stimulus_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    def emit_heartbeat(self, *, force: bool = False) -> dict[str, str] | None:
        interval = self._spec["heartbeat_interval_seconds"]
        now = time.monotonic()
        if not force and (now - self._last_emit_monotonic) < interval:
            return None
        ensure_fractal_bus_topology(self.repo)
        payload: dict[str, Any] = {
            "daemon_name": self.daemon_name,
            "daemon_uuid": self._spec["uuid"],
            "pid": os.getpid(),
            "uptime_seconds": max(0, int(now - self._started_at)),
            "status": "alive",
        }
        if self._last_stimulus_at:
            payload["last_stimulus_at"] = self._last_stimulus_at
        event = {
            "event_id": str(uuid.uuid4()),
            "event_type": "Daemon_Heartbeat",
            "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            "emitter_agent": self.daemon_name,
            "payload": payload,
        }
        seal = write_fractal_event(self.repo, event, "telemetry")
        self._last_emit_monotonic = now
        return seal

    def tick(self) -> dict[str, str] | None:
        if not self._bootstrapped:
            self.bootstrap()
        return self.emit_heartbeat()

    def shutdown(self) -> None:
        if self.lock_path.is_file():
            try:
                self.lock_path.unlink()
            except OSError:
                pass
        _REGISTRY.pop(self.daemon_name, None)

    def _read_lock(self) -> dict[str, Any] | None:
        if not self.lock_path.is_file():
            return None
        try:
            body = json.loads(self.lock_path.read_text(encoding="utf-8"))
            return body if isinstance(body, dict) else None
        except (OSError, json.JSONDecodeError):
            return None

    def _write_lock(self) -> None:
        status = _status_dir(self.repo)
        status.mkdir(parents=True, exist_ok=True)
        payload = {
            "daemon_name": self.daemon_name,
            "pid": os.getpid(),
            "started_at": self._started_iso,
            "heartbeat_interval_seconds": self._spec["heartbeat_interval_seconds"],
        }
        self.lock_path.write_text(
            json.dumps(payload, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )

    def _register_hooks(self) -> None:
        atexit.register(self.shutdown)

        def _handler(signum: int, _frame: Any) -> None:
            try:
                payload: dict[str, Any] = {
                    "daemon_name": self.daemon_name,
                    "daemon_uuid": self._spec["uuid"],
                    "pid": os.getpid(),
                    "uptime_seconds": max(0, int(time.monotonic() - self._started_at)),
                    "status": "shutting_down",
                }
                event = {
                    "event_id": str(uuid.uuid4()),
                    "event_type": "Daemon_Heartbeat",
                    "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
                    "emitter_agent": self.daemon_name,
                    "payload": payload,
                }
                write_fractal_event(self.repo, event, "telemetry")
            except Exception:
                pass
            self.shutdown()
            signal.signal(signum, signal.SIG_DFL)
            signal.raise_signal(signum)

        for sig in (signal.SIGTERM, signal.SIGINT):
            try:
                signal.signal(sig, _handler)
            except (ValueError, OSError):
                pass


def centinela_runtime(repo: Path, daemon_name: str) -> CentinelaRuntime:
    return CentinelaRuntime(repo, daemon_name)
