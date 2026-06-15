# -*- coding: utf-8 -*-
"""Proceso telegram-gateway — delega cápsula tool e inyecta en bus domain."""

from __future__ import annotations

import json
import os
import sys
from pathlib import Path
from typing import Any

from capsule_resolve import invoke_tool_capsule_json

_TOOL_DIR = Path(__file__).resolve().parents[1] / "limbo" / "tools" / "telegram-gateway"
if str(_TOOL_DIR) not in sys.path:
    sys.path.insert(0, str(_TOOL_DIR))

from eda_bus_utils import write_fractal_event
from transmute import build_telegram_message_received, transmute_text


def transmute_telegram_text(text: str) -> tuple[str, dict[str, Any]] | None:
    """Compat tests: retorna (event_type, payload)."""
    event = transmute_text(text)
    if event is None:
        return None
    payload = event.get("payload")
    if not isinstance(payload, dict):
        return None
    et = event.get("event_type")
    if not isinstance(et, str):
        return None
    return et, payload


def _invoke_tool_capsule(repo: Path, text: str) -> dict[str, Any]:
    _rc, body = invoke_tool_capsule_json(repo, "telegram-gateway", {"text": text})
    if not isinstance(body, dict):
        return {"success": False, "error": "envelope no objeto"}
    return body


def run_telegram_gateway(repo: Path, text: str) -> dict[str, Any]:
    capsule = _invoke_tool_capsule(repo, text)
    if not capsule.get("success"):
        return {"ok": False, "emitted": False, "error": capsule.get("error")}
    if not capsule.get("emitted"):
        return {"ok": True, "emitted": False, "reason": "empty_text"}
    event = capsule.get("event")
    if not isinstance(event, dict):
        event = transmute_text(text)
    if event is None:
        return {"ok": True, "emitted": False, "reason": "empty_text"}
    sensorial_id: str | None = None
    chat_id = os.environ.get("TELEGRAM_ALLOWED_CHAT_ID", "").strip()
    if chat_id:
        sensorial = build_telegram_message_received(text, chat_id)
        sensorial_seal = write_fractal_event(repo, sensorial, "domain")
        sensorial_id = sensorial.get("event_id") if isinstance(sensorial.get("event_id"), str) else None
    else:
        sensorial_seal = None
    seal = write_fractal_event(repo, event, "domain")
    return {
        "ok": True,
        "emitted": True,
        "event_type": event.get("event_type"),
        "event_id": event.get("event_id"),
        "telegram_message_received_id": sensorial_id,
        "seal": seal,
        "sensorial_seal": sensorial_seal,
    }
