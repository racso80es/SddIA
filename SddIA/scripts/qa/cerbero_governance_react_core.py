# -*- coding: utf-8 -*-
"""Cerbero reacción RBAC ante eventos Self-Healing (Fase 4+ agnóstico)."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from eda_bus_utils import _write_json_atomic, load_radamanto_config


def revoked_path(repo: Path) -> Path:
    cfg = load_radamanto_config(repo)
    rel = cfg.get("revoked_entities") or ".SddIA/cerbero/revoked_entities.json"
    return repo / rel


def load_revoked(repo: Path) -> dict[str, Any]:
    path = revoked_path(repo)
    if not path.is_file():
        return {"revoked": {}, "permanent": {}}
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {"revoked": {}, "permanent": {}}
    data.setdefault("revoked", {})
    data.setdefault("permanent", {})
    return data


def save_revoked(repo: Path, data: dict[str, Any]) -> None:
    path = revoked_path(repo)
    path.parent.mkdir(parents=True, exist_ok=True)
    _write_json_atomic(path, data)


def is_entity_revoked(repo: Path, entity_id: str) -> bool:
    data = load_revoked(repo)
    if entity_id in data.get("permanent", {}):
        return True
    return entity_id in data.get("revoked", {})


def _resolve_entity_id(payload: dict[str, Any]) -> str | None:
    eid = payload.get("entity_id") or payload.get("target_entity_id")
    if isinstance(eid, str) and eid.strip():
        return eid.strip()
    return None


def react_to_domain_event(repo: Path, event: dict[str, Any]) -> dict[str, Any]:
    event_type = event.get("event_type")
    payload = event.get("payload") or {}
    if not isinstance(payload, dict):
        return {"ok": False, "error": "payload invalido"}
    entity_id = _resolve_entity_id(payload)
    if not entity_id:
        return {"ok": False, "error": "entity_id requerido"}
    data = load_revoked(repo)

    if event_type == "Domain_Entity_Degraded":
        data["revoked"][entity_id] = {
            "since": event.get("timestamp"),
            "reason": payload.get("reason"),
            "entity_type": payload.get("entity_type"),
        }
        save_revoked(repo, data)
        return {"ok": True, "action": "revoked", "entity_id": entity_id}

    if event_type == "Domain_Entity_Restored":
        if event.get("emitter_agent") != "radamanto":
            return {"ok": False, "error": "Domain_Entity_Restored solo desde radamanto"}
        data["revoked"].pop(entity_id, None)
        save_revoked(repo, data)
        return {"ok": True, "action": "restored", "entity_id": entity_id}

    if event_type == "Domain_Entity_Deprecated":
        data["revoked"].pop(entity_id, None)
        data["permanent"][entity_id] = {
            "since": event.get("timestamp"),
            "reason": payload.get("reason"),
            "entity_type": payload.get("entity_type"),
        }
        save_revoked(repo, data)
        return {"ok": True, "action": "permanent_block", "entity_id": entity_id}

    return {"ok": False, "error": f"event_type no soportado: {event_type}"}
