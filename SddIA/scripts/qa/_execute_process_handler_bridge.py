#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Bridge: handlers satélite invocados desde binario Rust (P4)."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any, Callable

_QA = Path(__file__).resolve().parent
if str(_QA) not in sys.path:
    sys.path.insert(0, str(_QA))

from execute_process_core import repo_root  # noqa: E402


def _route_domain(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    from route_domain_event_core import route_domain_event

    rel = inputs.get("event_file_path")
    if not isinstance(rel, str) or not rel.strip():
        return {
            "success": False,
            "status_code": 1,
            "error": "event_file_path requerido",
            "execution_report": {"process_name": "route-domain-event", "phases": []},
        }
    out = route_domain_event(repo, rel.strip())
    ok = bool(out.get("success")) and out.get("exitCode", 1) == 0
    return {
        "success": ok,
        "status_code": out.get("exitCode", 0 if ok else 1),
        "data": out.get("data"),
        "error": out.get("error"),
        "execution_report": {
            "process_name": "route-domain-event",
            "phases": [
                {
                    "phase_name": "Orquestación route-domain-event",
                    "status": "executed" if ok else "failed",
                    "handler": "route-domain-event-core",
                    "dispatch_mode": (out.get("data") or {}).get("dispatch_mode"),
                }
            ],
        },
    }


def _telegram_fallback(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    from telegram_fallback_responder_core import run_telegram_fallback_responder

    text = inputs.get("text")
    if not isinstance(text, str):
        return {
            "success": False,
            "status_code": 1,
            "error": "text requerido",
            "execution_report": {"process_name": "telegram-fallback-responder", "phases": []},
        }
    chat_id = inputs.get("chat_id")
    out = run_telegram_fallback_responder(
        repo, text, chat_id if isinstance(chat_id, str) else None
    )
    ok = bool(out.get("ok"))
    return {
        "success": ok,
        "status_code": 0 if ok else 1,
        "data": out,
        "execution_report": {
            "process_name": "telegram-fallback-responder",
            "phases": [
                {
                    "phase_name": "Filtro C",
                    "status": "executed",
                    "handler": "telegram-fallback-responder-core",
                    "filtered": out.get("filtered"),
                },
                {
                    "phase_name": "Notificación",
                    "status": "executed" if ok else "failed",
                    "handler": "telegram-fallback-responder-core",
                    "notified": out.get("notified"),
                },
            ],
        },
    }


def _telegram_gateway(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    from telegram_gateway_core import run_telegram_gateway

    text = inputs.get("text")
    if not isinstance(text, str):
        return {
            "success": False,
            "status_code": 1,
            "error": "text requerido",
            "execution_report": {"process_name": "telegram-gateway", "phases": []},
        }
    out = run_telegram_gateway(repo, text)
    ok = bool(out.get("ok"))
    return {
        "success": ok,
        "status_code": 0 if ok else 1,
        "data": out,
        "execution_report": {
            "process_name": "telegram-gateway",
            "phases": [
                {
                    "phase_name": "Transmutación e inyección",
                    "status": "executed",
                    "handler": "telegram-gateway-core",
                    "emitted": out.get("emitted"),
                    "event_type": out.get("event_type"),
                }
            ],
        },
    }


def _daemon_kill_switch(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    from daemon_kill_switch_core import run_daemon_kill_switch

    if not isinstance(inputs.get("repository_path"), str):
        inputs = {**inputs, "repository_path": str(repo)}
    out = run_daemon_kill_switch(repo, inputs)
    ok = bool(out.get("success"))
    return {
        "success": ok,
        "status_code": out.get("exitCode", 0 if ok else 1),
        "data": out.get("data"),
        "error": out.get("error"),
        "execution_report": {
            "process_name": "daemon-kill-switch",
            "phases": [
                {
                    "phase_name": "Purga global",
                    "status": "executed" if ok else "failed",
                    "handler": "daemon-kill-switch-core",
                }
            ],
        },
    }


def _governance_daemon(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    from governance_daemon_manager_core import run_governance_daemon_manager

    if not isinstance(inputs.get("repository_path"), str):
        inputs = {**inputs, "repository_path": str(repo)}
    out = run_governance_daemon_manager(repo, inputs)
    ok = bool(out.get("success"))
    return {
        "success": ok,
        "status_code": out.get("exitCode", 0 if ok else 1),
        "data": out.get("data"),
        "error": out.get("error"),
        "execution_report": {
            "process_name": "governance-daemon-manager",
            "phases": [
                {
                    "phase_name": "Actuación OS",
                    "status": "executed" if ok else "failed",
                    "handler": "governance-daemon-manager-core",
                }
            ],
        },
    }


def _daemon_heartbeat(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    from daemon_heartbeat_audit_core import run_daemon_heartbeat_audit

    out = run_daemon_heartbeat_audit(repo, inputs)
    ok = bool(out.get("success"))
    return {
        "success": ok,
        "status_code": out.get("exitCode", 0 if ok else 1),
        "data": out.get("data"),
        "error": out.get("error"),
        "execution_report": {
            "process_name": "daemon-heartbeat-audit",
            "phases": [
                {
                    "phase_name": "Auditoría staleness",
                    "status": "executed" if ok else "failed",
                    "handler": "daemon-heartbeat-audit-core",
                }
            ],
        },
    }


HANDLERS: dict[str, Callable[[Path, dict[str, Any]], dict[str, Any]]] = {
    "route-domain-event": _route_domain,
    "telegram-fallback-responder": _telegram_fallback,
    "telegram-gateway": _telegram_gateway,
    "daemon-kill-switch": _daemon_kill_switch,
    "governance-daemon-manager": _governance_daemon,
    "daemon-heartbeat-audit": _daemon_heartbeat,
}


def main() -> int:
    raw = sys.stdin.read()
    if not raw.strip():
        err = {"success": False, "status_code": 1, "error": "stdin vacío", "exitCode": 1}
        print(json.dumps(err, ensure_ascii=False))
        return 1
    try:
        req = json.loads(raw)
    except json.JSONDecodeError as exc:
        err = {"success": False, "status_code": 1, "error": f"JSON inválido: {exc}", "exitCode": 1}
        print(json.dumps(err, ensure_ascii=False))
        return 1
    process_name = req.get("process")
    inputs = req.get("inputs")
    if not isinstance(process_name, str) or not process_name.strip():
        err = {"success": False, "status_code": 1, "error": "process requerido", "exitCode": 1}
        print(json.dumps(err, ensure_ascii=False))
        return 1
    if not isinstance(inputs, dict):
        err = {"success": False, "status_code": 1, "error": "inputs debe ser objeto", "exitCode": 1}
        print(json.dumps(err, ensure_ascii=False))
        return 1
    handler = HANDLERS.get(process_name.strip())
    if handler is None:
        err = {
            "success": False,
            "status_code": 1,
            "error": f"handler no registrado: {process_name}",
            "exitCode": 1,
        }
        print(json.dumps(err, ensure_ascii=False))
        return 1
    try:
        repo = repo_root()
        result = handler(repo, inputs)
        code = int(result.get("status_code", 0 if result.get("success") else 1))
        result.setdefault("exitCode", code)
        print(json.dumps(result, ensure_ascii=False))
        return code
    except Exception as exc:  # noqa: BLE001
        err = {"success": False, "status_code": 1, "error": str(exc), "exitCode": 1}
        print(json.dumps(err, ensure_ascii=False))
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
