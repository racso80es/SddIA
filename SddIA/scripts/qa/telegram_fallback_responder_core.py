# -*- coding: utf-8 -*-
"""Proceso telegram-fallback-responder — Filtro C → Síntesis → Materialización."""

from __future__ import annotations

import os
import re
from typing import Any

from telegram_notify_core import invoke_send_telegram_notification

_COMMAND_PREFIXES = ("/", "!")
_RESERVED_RE = re.compile(r"^\s*(TODO|IDEA)\s*:", re.IGNORECASE)

MAYEUTA_PROMPT_TEMPLATE = (
    '[HARD OVERRIDE] Has recibido este estímulo externo: "{text}". '
    "Genera una respuesta orgánica de máximo 2 líneas. "
    "Habla desde tu identidad arquitectónica (Tormentosa/Aiúa). "
    "Acusa recibo, asimila o cuestiona el estímulo. "
    "PROHIBIDO: Ser verboso, ofrecer asistencia genérica o actuar como herramienta esclava."
)


def filter_c_should_abort(text: str) -> bool:
    raw = text if isinstance(text, str) else ""
    stripped = raw.strip()
    if not stripped:
        return True
    if stripped.startswith(_COMMAND_PREFIXES):
        return True
    if _RESERVED_RE.match(stripped):
        return True
    return False


def synthesize_mayeuta_response(text: str) -> str:
    """Síntesis determinista de laboratorio alineada al prompt Mayeuta (≤2 líneas)."""
    snippet = text.strip()[:120]
    _ = MAYEUTA_PROMPT_TEMPLATE.format(text=snippet)
    line1 = f"[Tormentosa/Aiúa] Recibo el estímulo: «{snippet}»."
    line2 = "Lo asimilo como fricción arquitectónica — ¿es señal o ruido?"
    return f"{line1}\n{line2}"


def run_telegram_fallback_responder(
    repo: Any,
    text: str,
    chat_id: str | None = None,
) -> dict[str, Any]:
    if filter_c_should_abort(text):
        return {
            "ok": True,
            "filtered": True,
            "synthesized": False,
            "notified": False,
            "reason": "filtro_c_abort",
        }

    synthesized = synthesize_mayeuta_response(text)
    resolved_chat = (chat_id or os.environ.get("TELEGRAM_ALLOWED_CHAT_ID", "")).strip()
    if not resolved_chat:
        return {
            "ok": False,
            "filtered": False,
            "synthesized": True,
            "notified": False,
            "error": "chat_id ausente",
        }

    ok, body = invoke_send_telegram_notification(
        repo,
        synthesized,
        parse_mode=None,
    )
    return {
        "ok": ok,
        "filtered": False,
        "synthesized": True,
        "notified": ok,
        "chat_id": resolved_chat,
        "message_preview": synthesized.splitlines()[0][:80],
        "tool_result": body,
        "error": None if ok else body.get("error"),
    }
