# -*- coding: utf-8 -*-
"""Proceso telegram-gateway — delega cápsula tool e inyecta en bus domain."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Any

_TOOL_DIR = Path(__file__).resolve().parents[1] / "tools" / "telegram-gateway"
if str(_TOOL_DIR) not in sys.path:
    sys.path.insert(0, str(_TOOL_DIR))

from eda_bus_utils import write_fractal_event
from transmute import transmute_text


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
    script = repo / "SddIA" / "scripts" / "tools" / "telegram-gateway" / "main.py"
    proc = subprocess.run(
        [sys.executable, str(script)],
        input=json.dumps({"text": text}, ensure_ascii=False),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if not line:
        return {"success": False, "error": proc.stderr or "sin salida cápsula"}
    try:
        body = json.loads(line)
    except json.JSONDecodeError:
        return {"success": False, "error": "JSON inválido cápsula"}
    if not isinstance(body, dict):
        return {"success": False, "error": "envelope no objeto"}
    if proc.returncode != 0 and not body.get("success"):
        return body
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
    seal = write_fractal_event(repo, event, "domain")
    return {
        "ok": True,
        "emitted": True,
        "event_type": event.get("event_type"),
        "event_id": event.get("event_id"),
        "seal": seal,
    }
