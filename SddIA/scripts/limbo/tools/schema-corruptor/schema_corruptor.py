#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Cápsula schema-corruptor — recibo telemetría inválido (Caos S+)."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

TOOL_NAME = "schema-corruptor"


def _emit(envelope: dict[str, Any]) -> None:
    code = envelope.get("exitCode", 0 if envelope.get("success") else 1)
    envelope.setdefault("name", TOOL_NAME)
    envelope.setdefault("exitCode", code)
    sys.stdout.write(json.dumps(envelope, ensure_ascii=False) + "\n")
    sys.exit(code)


def _fail(message: str, *, code: int = 1) -> None:
    _emit({
        "name": TOOL_NAME,
        "success": False,
        "exitCode": code,
        "message": message,
        "error": message,
        "result": None,
    })


def _load_request() -> dict[str, Any]:
    raw = sys.stdin.read()
    if not raw.strip():
        return {}
    data = json.loads(raw)
    return data if isinstance(data, dict) else {}


def main() -> None:
    req = _load_request()
    mode = req.get("corruption_mode", "empty")
    if not isinstance(mode, str):
        mode = "empty"
    mode = mode.strip().lower()
    if mode not in ("empty", "invalid_json", "partial"):
        _fail("corruption_mode invalido: empty | invalid_json | partial")

    envelope: dict[str, Any] = {
        "name": TOOL_NAME,
        "success": True,
        "exitCode": 0,
        "message": f"recibo corrupto modo={mode}",
        "result": {"corruption_mode": mode},
    }

    if mode == "empty":
        pass
    elif mode == "invalid_json":
        envelope["telemetry_receipt"] = "not-valid-json{{"
    elif mode == "partial":
        envelope["telemetry_receipt"] = {"prompt_tokens": 1}

    _emit(envelope)


if __name__ == "__main__":
    main()
