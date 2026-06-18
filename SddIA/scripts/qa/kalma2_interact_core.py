# -*- coding: utf-8 -*-
"""Proceso kalma2-interact — síntesis Mayeuta lab para cliente Kalma2."""

from __future__ import annotations

from typing import Any

from telegram_fallback_responder_core import synthesize_mayeuta_response


def run_kalma2_interact(_repo: Any, prompt: str) -> dict[str, Any]:
    raw = prompt if isinstance(prompt, str) else ""
    stripped = raw.strip()
    if not stripped:
        return {"ok": False, "error": "prompt vacío", "response": None}

    response = synthesize_mayeuta_response(stripped)
    return {"ok": True, "response": response, "error": None}
