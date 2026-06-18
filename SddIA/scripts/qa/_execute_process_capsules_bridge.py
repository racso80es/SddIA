#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Bridge interno residual: procesos no portados a Rust (creators, telemetry, accept-pr, …).

Lee JSON stdin: {"process": "<name>", "inputs": {...}}
Emite envelope JSON (última línea stdout).
"""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

_QA = Path(__file__).resolve().parent
if str(_QA) not in sys.path:
    sys.path.insert(0, str(_QA))

from execute_process_capsules import run_process  # noqa: E402
from execute_process_core import repo_root  # noqa: E402


def main() -> int:
    raw = sys.stdin.read()
    if not raw.strip():
        out = {"success": False, "status_code": 1, "error": "stdin vacío", "exitCode": 1}
        sys.stdout.write(json.dumps(out, ensure_ascii=False) + "\n")
        return 1
    try:
        req: dict[str, Any] = json.loads(raw)
    except json.JSONDecodeError as exc:
        out = {"success": False, "status_code": 1, "error": f"JSON inválido: {exc}", "exitCode": 1}
        sys.stdout.write(json.dumps(out, ensure_ascii=False) + "\n")
        return 1

    process_name = req.get("process")
    process_inputs = req.get("inputs")
    if not isinstance(process_name, str) or not process_name.strip():
        out = {"success": False, "status_code": 1, "error": "process requerido", "exitCode": 1}
        sys.stdout.write(json.dumps(out, ensure_ascii=False) + "\n")
        return 1
    if not isinstance(process_inputs, dict):
        out = {"success": False, "status_code": 1, "error": "inputs debe ser objeto", "exitCode": 1}
        sys.stdout.write(json.dumps(out, ensure_ascii=False) + "\n")
        return 1

    try:
        repo = repo_root()
        result = run_process(repo, process_name.strip(), process_inputs)
        code = int(result.get("status_code", 0 if result.get("success") else 1))
        result.setdefault("exitCode", code)
        sys.stdout.write(json.dumps(result, ensure_ascii=False) + "\n")
        return code
    except Exception as exc:  # noqa: BLE001 — bridge: envelope JSON obligatorio
        out = {"success": False, "status_code": 1, "error": str(exc), "exitCode": 1}
        sys.stdout.write(json.dumps(out, ensure_ascii=False) + "\n")
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
