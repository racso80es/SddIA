# -*- coding: utf-8 -*-
"""Cliente HTTP mínimo Telegram sendMessage + Táctica del Refugio."""

from __future__ import annotations

import json
import re
import urllib.error
import urllib.request
from typing import Any

_PARSE_ERROR_MARKERS = (
    "can't parse entities",
    "can't parse message text",
    "wrong html entity",
    "character",
    "is reserved",
    "can't parse",
)


def escape_markdown_v2(text: str) -> str:
    """Escapa caracteres reservados MarkdownV2 (primera línea de defensa)."""
    reserved = r"_*[]()~`>#+-=|{}.!"
    return re.sub(f"([{re.escape(reserved)}])", r"\\\1", text)


def is_telegram_parse_error(http_code: int, body: dict[str, Any]) -> bool:
    if http_code != 400:
        return False
    desc = str(body.get("description") or "").lower()
    return any(marker in desc for marker in _PARSE_ERROR_MARKERS)


def _post_send_message(
    token: str,
    chat_id: str,
    message: str,
    parse_mode: str | None,
) -> tuple[int, dict[str, Any]]:
    url = f"https://api.telegram.org/bot{token}/sendMessage"
    payload: dict[str, Any] = {"chat_id": chat_id, "text": message[:4096]}
    if parse_mode:
        payload["parse_mode"] = parse_mode
    data = json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(
        url,
        data=data,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            raw = resp.read().decode("utf-8", errors="replace")
            code = resp.getcode()
    except urllib.error.HTTPError as exc:
        raw = exc.read().decode("utf-8", errors="replace")
        code = exc.code
    except urllib.error.URLError as exc:
        return 0, {"ok": False, "description": str(exc.reason)}
    try:
        body = json.loads(raw) if raw.strip() else {}
    except json.JSONDecodeError:
        body = {"ok": False, "description": raw[:500]}
    if not isinstance(body, dict):
        body = {"ok": False, "description": str(body)}
    return code, body


def send_message_with_refugio(
    token: str,
    chat_id: str,
    message: str,
    parse_mode: str | None = "MarkdownV2",
) -> dict[str, Any]:
    """Máximo 2 POST: formato solicitado → refugio plain si parsing falla."""
    requested = parse_mode
    text_attempt1 = escape_markdown_v2(message) if parse_mode == "MarkdownV2" else message
    code1, body1 = _post_send_message(token, chat_id, text_attempt1, parse_mode)
    if body1.get("ok"):
        result = body1.get("result") if isinstance(body1.get("result"), dict) else {}
        mid = result.get("message_id")
        return {
            "success": True,
            "message_id": mid,
            "attempt": 1,
            "degraded_plain_fallback": False,
            "parse_mode_requested": requested,
            "error": None,
        }

    if parse_mode and is_telegram_parse_error(code1, body1):
        code2, body2 = _post_send_message(token, chat_id, message, None)
        if body2.get("ok"):
            result = body2.get("result") if isinstance(body2.get("result"), dict) else {}
            mid = result.get("message_id")
            return {
                "success": True,
                "message_id": mid,
                "attempt": 2,
                "degraded_plain_fallback": True,
                "parse_mode_requested": requested,
                "error": None,
            }
        err2 = body2.get("description") or body2.get("error") or f"HTTP {code2}"
        return {
            "success": False,
            "message_id": None,
            "attempt": 2,
            "degraded_plain_fallback": True,
            "parse_mode_requested": requested,
            "error": str(err2),
        }

    err1 = body1.get("description") or body1.get("error") or f"HTTP {code1}"
    return {
        "success": False,
        "message_id": None,
        "attempt": 1,
        "degraded_plain_fallback": False,
        "parse_mode_requested": requested,
        "error": str(err1),
    }
