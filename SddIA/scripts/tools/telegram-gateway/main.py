#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Cápsula telegram-gateway — stdin text → stdout event JSON."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

_TOOL_DIR = Path(__file__).resolve().parent
if str(_TOOL_DIR) not in sys.path:
    sys.path.insert(0, str(_TOOL_DIR))

from transmute import transmute_text

TOOL_NAME = "telegram-gateway"


def _emit(body: dict[str, Any], *, code: int = 0) -> None:
    body.setdefault("name", TOOL_NAME)
    body.setdefault("exitCode", code)
    body.setdefault("success", code == 0)
    sys.stdout.write(json.dumps(body, ensure_ascii=False) + "\n")
    sys.exit(code)


def _load_request() -> dict[str, Any]:
    raw = sys.stdin.read()
    if not raw.strip():
        return {}
    data = json.loads(raw)
    return data if isinstance(data, dict) else {}


def main() -> None:
    req = _load_request()
    text = req.get("text")
    if not isinstance(text, str):
        _emit({"success": False, "error": "text obligatorio", "emitted": False}, code=1)

    event = transmute_text(text)
    if event is None:
        _emit({"success": True, "emitted": False, "event": None})
    _emit({"success": True, "emitted": True, "event": event, "event_type": event.get("event_type")})


if __name__ == "__main__":
    main()
