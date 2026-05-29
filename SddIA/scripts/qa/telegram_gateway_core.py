# -*- coding: utf-8 -*-
"""Proceso telegram-gateway — transmutación texto → evento domain."""

from __future__ import annotations

import re
import uuid
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from eda_bus_utils import write_fractal_event

_TODO_RE = re.compile(r"^\s*TODO:\s*(.+)$", re.IGNORECASE)


def _iso_now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def transmute_telegram_text(text: str) -> tuple[str, dict[str, Any]] | None:
    raw = text if isinstance(text, str) else ""
    stripped = raw.strip()
    if not stripped:
        return None
    match = _TODO_RE.match(stripped)
    if match:
        idea = match.group(1).strip()
        return (
            "Kaizen_Idea_Captured",
            {
                "idea_text": idea,
                "source": "telegram",
                "raw_text": raw,
            },
        )
    return (
        "Manual_Task_Requested",
        {
            "task_text": stripped,
            "source": "telegram",
            "raw_text": raw,
        },
    )


def run_telegram_gateway(repo: Path, text: str) -> dict[str, Any]:
    transmuted = transmute_telegram_text(text)
    if transmuted is None:
        return {"ok": True, "emitted": False, "reason": "empty_text"}
    event_type, payload = transmuted
    event = {
        "event_id": str(uuid.uuid4()),
        "event_type": event_type,
        "event_family": "domain",
        "timestamp": _iso_now(),
        "emitter_agent": "telegram-gateway",
        "payload": payload,
        "delivery_state": {},
    }
    seal = write_fractal_event(repo, event, "domain")
    return {
        "ok": True,
        "emitted": True,
        "event_type": event_type,
        "event_id": event["event_id"],
        "seal": seal,
    }
