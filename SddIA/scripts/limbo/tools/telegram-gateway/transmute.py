# -*- coding: utf-8 -*-
"""Transmutación texto Telegram → instancia ECST domain (SSOT lógica)."""

from __future__ import annotations

import re
import uuid
from datetime import datetime, timezone
from typing import Any

_TODO_RE = re.compile(r"^\s*TODO:\s*(.+)$", re.IGNORECASE)
_IDEA_RE = re.compile(r"^\s*IDEA:\s*(.+)$", re.IGNORECASE)
_EMITTER = "telegram-gateway"


def _iso_now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def build_telegram_message_received(text: str, chat_id: str) -> dict[str, Any]:
    raw = text if isinstance(text, str) else ""
    return {
        "event_id": str(uuid.uuid4()),
        "event_type": "TelegramMessage_Received",
        "event_family": "domain",
        "timestamp": _iso_now(),
        "emitter_agent": _EMITTER,
        "payload": {
            "text": raw,
            "chat_id": str(chat_id),
            "source": "telegram",
            "raw_text": raw,
        },
        "delivery_state": {},
    }


def transmute_text(text: str) -> dict[str, Any] | None:
    raw = text if isinstance(text, str) else ""
    stripped = raw.strip()
    if not stripped:
        return None
    match = _TODO_RE.match(stripped) or _IDEA_RE.match(stripped)
    if match:
        return {
            "event_id": str(uuid.uuid4()),
            "event_type": "Kaizen_Idea_Captured",
            "event_family": "domain",
            "timestamp": _iso_now(),
            "emitter_agent": _EMITTER,
            "payload": {
                "idea_text": match.group(1).strip(),
                "source": "telegram",
                "raw_text": raw,
            },
            "delivery_state": {},
        }
    return {
        "event_id": str(uuid.uuid4()),
        "event_type": "Manual_Task_Requested",
        "event_family": "domain",
        "timestamp": _iso_now(),
        "emitter_agent": _EMITTER,
        "payload": {
            "task_text": stripped,
            "source": "telegram",
            "raw_text": raw,
        },
        "delivery_state": {},
    }
