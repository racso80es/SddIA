# -*- coding: utf-8 -*-
"""Auditoría cumplimiento termodinámico — Fase 5 (fan-out telemetría)."""

from __future__ import annotations

import json
import os
from pathlib import Path
from typing import Any

from eda_bus_utils import (
    build_telemetry_compliance_breached_event,
    load_telemetry_compliance_config,
    receipt_satisfies_schema,
    resolve_ed_telemetry_contract,
    stamp_fractal_delivery_state,
    write_fractal_event,
)

COMPLIANCE_SUBSCRIBER_KEY = "argos.telemetry-compliance-audit"


def _emitted_path(repo: Path) -> Path:
    cfg = load_telemetry_compliance_config(repo)
    return repo / cfg["emitted_registry"]


def load_emitted_breaches(repo: Path) -> set[str]:
    path = _emitted_path(repo)
    if not path.is_file():
        return set()
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return set()
    ids = data.get("breach_asset_ids") or []
    return {str(x) for x in ids if x}


def mark_breach_emitted(repo: Path, asset_id: str) -> None:
    emitted = load_emitted_breaches(repo)
    emitted.add(asset_id)
    path = _emitted_path(repo)
    path.parent.mkdir(parents=True, exist_ok=True)
    from eda_bus_utils import _write_json_atomic

    _write_json_atomic(path, {"breach_asset_ids": sorted(emitted)})


def emit_breach_if_needed(
    repo: Path,
    *,
    asset_id: str,
    capsule_id: str,
    process_name: str,
    breach_reason: str,
    expected_schema: list[str] | None,
) -> dict[str, Any] | None:
    if asset_id in load_emitted_breaches(repo):
        return None
    event = build_telemetry_compliance_breached_event(
        asset_id=asset_id,
        capsule_id=capsule_id,
        process_name=process_name,
        breach_reason=breach_reason,
        expected_schema=expected_schema,
    )
    seal = write_fractal_event(repo, event, "domain")
    mark_breach_emitted(repo, asset_id)
    route_out: dict[str, Any] | None = None
    if os.environ.get("SDDIA_LAB_ROUTE_SYNC", "").strip().lower() in ("1", "true", "yes"):
        from route_fractal_event_core import route_domain_fractal_event

        route_out = route_domain_fractal_event(repo, seal["target_path"])
    return {"seal": seal, "route": route_out}


def audit_telemetry_compliance(repo: Path, rel_path: str) -> dict[str, Any]:
    event_path = (repo / rel_path.strip()).resolve()
    if not event_path.is_file():
        return {"ok": False, "error": f"no existe: {rel_path}"}
    try:
        body = json.loads(event_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        return {"ok": False, "error": str(exc)}

    if body.get("event_type") != "Raw_Execution_Finished":
        stamp_fractal_delivery_state(
            repo, event_path, COMPLIANCE_SUBSCRIBER_KEY, "skipped"
        )
        return {"ok": True, "status": "skipped", "reason": "wrong_event_type"}

    payload = body.get("payload") or {}
    if not isinstance(payload, dict):
        return {"ok": False, "error": "payload invalido"}

    asset_id = payload.get("asset_id")
    if not isinstance(asset_id, str) or not asset_id.strip():
        return {"ok": False, "error": "asset_id requerido"}

    capsule_id = payload.get("capsule_id")
    capsule_str = capsule_id.strip() if isinstance(capsule_id, str) else ""
    process_name = str(payload.get("process_name") or "unknown")
    contract = resolve_ed_telemetry_contract(repo, capsule_str or None)

    if not contract.get("telemetry_provided"):
        stamp_fractal_delivery_state(
            repo, event_path, COMPLIANCE_SUBSCRIBER_KEY, "success"
        )
        return {"ok": True, "status": "skipped", "reason": "not_required"}

    receipt = payload.get("telemetry_receipt")
    schema = contract.get("telemetry_schema")
    schema_list = schema if isinstance(schema, list) else None

    breach: dict[str, Any] | None = None
    if receipt is None or not isinstance(receipt, dict) or not receipt:
        breach = emit_breach_if_needed(
            repo,
            asset_id=asset_id,
            capsule_id=capsule_str or process_name,
            process_name=process_name,
            breach_reason="missing_receipt",
            expected_schema=schema_list,
        )
    elif schema_list and not receipt_satisfies_schema(receipt, schema_list):
        breach = emit_breach_if_needed(
            repo,
            asset_id=asset_id,
            capsule_id=capsule_str or process_name,
            process_name=process_name,
            breach_reason="schema_mismatch",
            expected_schema=schema_list,
        )

    stamp_fractal_delivery_state(
        repo, event_path, COMPLIANCE_SUBSCRIBER_KEY, "success"
    )
    return {
        "ok": True,
        "status": "breach" if breach else "pass",
        "breach": breach,
        "asset_id": asset_id,
    }
