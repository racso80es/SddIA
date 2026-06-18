#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Bridge route-domain-event invocado desde binario Rust (P4 — core EDA en Python)."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

_QA = Path(__file__).resolve().parent
if str(_QA) not in sys.path:
    sys.path.insert(0, str(_QA))

from execute_process_core import repo_root  # noqa: E402
from route_domain_event_core import route_domain_event  # noqa: E402


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

    inputs = req.get("inputs")
    if not isinstance(inputs, dict):
        err = {"success": False, "status_code": 1, "error": "inputs debe ser objeto", "exitCode": 1}
        print(json.dumps(err, ensure_ascii=False))
        return 1

    rel = inputs.get("event_file_path")
    if not isinstance(rel, str) or not rel.strip():
        err = {
            "success": False,
            "status_code": 1,
            "error": "event_file_path requerido",
            "execution_report": {"process_name": "route-domain-event", "phases": []},
            "exitCode": 1,
        }
        print(json.dumps(err, ensure_ascii=False))
        return 1

    try:
        repo = repo_root()
        out = route_domain_event(repo, rel.strip())
        ok = bool(out.get("success")) and out.get("exitCode", 1) == 0
        result = {
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
        code = int(result.get("status_code", 0 if ok else 1))
        result.setdefault("exitCode", code)
        print(json.dumps(result, ensure_ascii=False))
        return code
    except Exception as exc:  # noqa: BLE001
        err = {"success": False, "status_code": 1, "error": str(exc), "exitCode": 1}
        print(json.dumps(err, ensure_ascii=False))
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
