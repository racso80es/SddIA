#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""tool:read-event-subscriptions — lectura estricta del SSOT de suscripciones EDA."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

TOOL_NAME = "read-event-subscriptions"


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace")


def _emit(envelope: dict[str, Any]) -> None:
    code = envelope.get("exitCode", 0 if envelope.get("success") else 1)
    envelope.setdefault("name", TOOL_NAME)
    envelope.setdefault("exitCode", code)
    sys.stdout.write(json.dumps(envelope, ensure_ascii=False) + "\n")
    sys.exit(code)


def _fail(message: str) -> None:
    _emit({"name": TOOL_NAME, "success": False, "exitCode": 1, "message": message, "error": message})


def main() -> None:
    try:
        raw = sys.stdin.read()
        req = json.loads(raw) if raw.strip() else {}
    except json.JSONDecodeError as e:
        _fail(f"JSON inválido: {e}")

    event_type = req.get("event_type")
    repo = _repo_root()
    cumulo = json.loads((repo / "SddIA" / "core" / "cumulo.paths.json").read_text(encoding="utf-8"))
    rel = (cumulo.get("eda_bus") or {}).get("subscriptions") or cumulo.get("normative_documents", {}).get(
        "event_subscriptions", "SddIA/core/event-subscriptions.json"
    )
    subs_path = repo / rel
    if not subs_path.is_file():
        _fail(f"SSOT inexistente: {rel}")

    try:
        registry = json.loads(subs_path.read_text(encoding="utf-8-sig"))
    except (OSError, json.JSONDecodeError) as e:
        _fail(f"No se pudo parsear suscripciones: {e}")

    if event_type is None:
        _emit(
            {
                "name": TOOL_NAME,
                "success": True,
                "exitCode": 0,
                "message": "Registro completo cargado.",
                "result": {
                    "subscriptions_path": rel.replace("\\", "/"),
                    "registry": registry,
                },
            }
        )

    if not isinstance(event_type, str) or not event_type.strip():
        _fail("event_type debe ser string no vacío si se proporciona")

    subscribers = registry.get(event_type.strip())
    if subscribers is None:
        subscribers = []
    if not isinstance(subscribers, list):
        _fail(f"Entrada para {event_type} no es array")

    _emit(
        {
            "name": TOOL_NAME,
            "success": True,
            "exitCode": 0,
            "message": f"Suscriptores para {event_type}.",
            "result": {
                "subscriptions_path": rel.replace("\\", "/"),
                "event_type": event_type.strip(),
                "subscribers": subscribers,
            },
        }
    )


if __name__ == "__main__":
    main()
