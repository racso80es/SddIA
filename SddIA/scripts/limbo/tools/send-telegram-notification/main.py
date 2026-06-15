#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Cápsula send-telegram-notification — POST ciego + Táctica del Refugio."""

from __future__ import annotations

import json
import os
import sys
from pathlib import Path
from typing import Any

_TOOL_DIR = Path(__file__).resolve().parent
if str(_TOOL_DIR) not in sys.path:
    sys.path.insert(0, str(_TOOL_DIR))

from telegram_api import send_message_with_refugio

TOOL_NAME = "send-telegram-notification"


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace")


def _load_env(repo: Path) -> None:
    qa = repo / "SddIA" / "scripts" / "qa"
    if str(qa) not in sys.path:
        sys.path.insert(0, str(qa))
    from env_loader import load_hierarchical_env

    load_hierarchical_env(repo)


def _emit(envelope: dict[str, Any]) -> None:
    code = 0 if envelope.get("success") else 1
    envelope.setdefault("name", TOOL_NAME)
    envelope.setdefault("exitCode", code)
    sys.stdout.write(json.dumps(envelope, ensure_ascii=False) + "\n")
    sys.exit(code)


def _fail(message: str, *, code: int = 2) -> None:
    _emit({
        "name": TOOL_NAME,
        "success": False,
        "exitCode": code,
        "error": message,
        "message": message,
    })


def _load_request() -> dict[str, Any]:
    raw = sys.stdin.read()
    if not raw.strip():
        return {}
    data = json.loads(raw)
    return data if isinstance(data, dict) else {}


def main() -> None:
    repo = _repo_root()
    _load_env(repo)
    token = os.environ.get("TELEGRAM_BOT_TOKEN", "").strip()
    chat_id = os.environ.get("TELEGRAM_ALLOWED_CHAT_ID", "").strip()
    if not token or not chat_id:
        _fail("TELEGRAM_BOT_TOKEN y TELEGRAM_ALLOWED_CHAT_ID son obligatorios")

    req = _load_request()
    message = req.get("message")
    if not isinstance(message, str) or not message.strip():
        _fail("message obligatorio")
    parse_mode = req.get("parse_mode", "MarkdownV2")
    if parse_mode is not None and not isinstance(parse_mode, str):
        _fail("parse_mode debe ser string o null")
    if parse_mode == "":
        parse_mode = None

    result = send_message_with_refugio(token, chat_id, message.strip(), parse_mode)
    envelope = {
        "name": TOOL_NAME,
        "success": result["success"],
        "exitCode": 0 if result["success"] else 1,
        "message_id": result.get("message_id"),
        "attempt": result.get("attempt"),
        "degraded_plain_fallback": result.get("degraded_plain_fallback"),
        "parse_mode_requested": result.get("parse_mode_requested"),
        "error": result.get("error"),
        "result": result,
    }
    _emit(envelope)


if __name__ == "__main__":
    main()
