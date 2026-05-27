# -*- coding: utf-8 -*-
"""Radamanto batch — acumulador telemetría y emisión Self-Healing (Fase 4)."""

from __future__ import annotations

import json
import os
import uuid
from pathlib import Path
from typing import Any

from eda_bus_utils import (
    _iso_now,
    _write_json_atomic,
    load_radamanto_config,
    write_fractal_event,
)


def _stats_path(repo: Path) -> Path:
    cfg = load_radamanto_config(repo)
    return repo / cfg["stats"]


def _consumed_path(repo: Path) -> Path:
    cfg = load_radamanto_config(repo)
    return repo / cfg["consumed"]


def _ensure_parent(path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)


def load_stats(repo: Path) -> dict[str, Any]:
    path = _stats_path(repo)
    if not path.is_file():
        return {"entities": {}}
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {"entities": {}}
    if not isinstance(data.get("entities"), dict):
        data["entities"] = {}
    return data


def save_stats(repo: Path, stats: dict[str, Any]) -> None:
    _ensure_parent(_stats_path(repo))
    _write_json_atomic(_stats_path(repo), stats)


def load_consumed(repo: Path) -> set[str]:
    path = _consumed_path(repo)
    if not path.is_file():
        return set()
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return set()
    ids = data.get("asset_ids") or []
    return {str(x) for x in ids if x}


def mark_consumed(repo: Path, asset_id: str) -> None:
    consumed = load_consumed(repo)
    consumed.add(asset_id)
    _ensure_parent(_consumed_path(repo))
    _write_json_atomic(_consumed_path(repo), {"asset_ids": sorted(consumed)})


def target_entity_from_payload(payload: dict[str, Any]) -> str:
    capsule = payload.get("capsule_id")
    if isinstance(capsule, str) and capsule.strip():
        return capsule.strip()
    process_name = payload.get("process_name")
    if isinstance(process_name, str) and process_name.strip():
        return process_name.strip()
    return "unknown-entity"


def _entity_bucket(stats: dict[str, Any], entity_id: str) -> dict[str, Any]:
    entities = stats.setdefault("entities", {})
    if entity_id not in entities or not isinstance(entities[entity_id], dict):
        entities[entity_id] = {
            "samples": [],
            "status": "healthy",
            "recovery_attempts": 0,
            "degraded_at": None,
            "structure_valid": False,
            "consecutive_success_count": 0,
        }
    return entities[entity_id]


def set_structure_valid(repo: Path, entity_id: str, valid: bool = True) -> None:
    stats = load_stats(repo)
    bucket = _entity_bucket(stats, entity_id)
    bucket["structure_valid"] = bool(valid)
    if valid and bucket.get("status") == "degraded":
        bucket["status"] = "pending_redemption"
        bucket["consecutive_success_count"] = 0
    save_stats(repo, stats)


def _success_rate(samples: list[dict[str, Any]]) -> float:
    if not samples:
        return 1.0
    ok = sum(1 for s in samples if int(s.get("exit_code", 1)) == 0)
    return ok / len(samples)


def _avg_duration(samples: list[dict[str, Any]]) -> float:
    if not samples:
        return 0.0
    return sum(int(s.get("duration_ms", 0)) for s in samples) / len(samples)


def build_domain_event(
    event_type: str,
    payload: dict[str, Any],
) -> dict[str, Any]:
    return {
        "event_id": str(uuid.uuid4()),
        "event_type": event_type,
        "event_family": "domain",
        "timestamp": _iso_now(),
        "emitter_agent": "radamanto",
        "payload": payload,
        "delivery_state": {},
    }


def emit_domain_and_route(repo: Path, event: dict[str, Any]) -> dict[str, Any]:
    seal = write_fractal_event(repo, event, "domain")
    route_out: dict[str, Any] | None = None
    if os.environ.get("SDDIA_LAB_ROUTE_SYNC", "").strip().lower() in ("1", "true", "yes"):
        from route_fractal_event_core import route_domain_fractal_event

        route_out = route_domain_fractal_event(repo, seal["target_path"])
    return {"seal": seal, "route": route_out}


def process_telemetry_file(repo: Path, rel_path: str) -> dict[str, Any]:
    cfg = load_radamanto_config(repo)
    thresholds = cfg["thresholds"]
    event_path = (repo / rel_path.strip()).resolve()
    if not event_path.is_file():
        return {"ok": False, "error": f"no existe: {rel_path}"}
    try:
        body = json.loads(event_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        return {"ok": False, "error": str(exc)}

    payload = body.get("payload") or {}
    if not isinstance(payload, dict):
        return {"ok": False, "error": "payload invalido"}

    asset_id = payload.get("asset_id")
    if not isinstance(asset_id, str) or not asset_id.strip():
        return {"ok": False, "error": "asset_id requerido"}

    if asset_id in load_consumed(repo):
        event_path.unlink(missing_ok=True)
        return {"ok": True, "skipped": "duplicate_asset_id", "asset_id": asset_id}

    entity_id = target_entity_from_payload(payload)
    stats = load_stats(repo)
    bucket = _entity_bucket(stats, entity_id)
    sample = {
        "asset_id": asset_id,
        "exit_code": int(payload.get("exit_code", 1)),
        "duration_ms": int(payload.get("duration_ms", 0)),
    }
    samples: list[dict[str, Any]] = list(bucket.get("samples") or [])
    samples.append(sample)
    max_keep = max(int(thresholds.get("batch_min_events", 10)), 20)
    bucket["samples"] = samples[-max_keep:]

    actions: list[dict[str, Any]] = []
    status = str(bucket.get("status") or "healthy")
    exit_code = sample["exit_code"]

    def _reload_bucket() -> None:
        nonlocal stats, bucket, status
        stats = load_stats(repo)
        bucket = _entity_bucket(stats, entity_id)
        bucket["samples"] = samples
        status = str(bucket.get("status") or "healthy")

    if status in ("degraded", "pending_redemption"):
        if exit_code == 0:
            bucket["consecutive_success_count"] = int(bucket.get("consecutive_success_count") or 0) + 1
            save_stats(repo, stats)
        else:
            bucket["consecutive_success_count"] = 0
            attempts = int(bucket.get("recovery_attempts") or 0)
            if attempts >= int(thresholds.get("max_recovery_attempts", 3)):
                bucket["status"] = "deprecated"
                save_stats(repo, stats)
                ev = build_domain_event(
                    "Tool_Deprecated",
                    {
                        "target_entity_id": entity_id,
                        "recovery_attempts": attempts,
                        "reason": "max_recovery_attempts_exceeded",
                    },
                )
                actions.append({"type": "Tool_Deprecated", "result": emit_domain_and_route(repo, ev)})
                _reload_bucket()

    if status == "pending_redemption" and bucket.get("structure_valid"):
        need = int(thresholds.get("redemption_success_count", 3))
        if int(bucket.get("consecutive_success_count") or 0) >= need:
            rate = _success_rate(bucket["samples"])
            bucket["status"] = "healthy"
            bucket["structure_valid"] = False
            bucket["consecutive_success_count"] = 0
            bucket["degraded_at"] = None
            save_stats(repo, stats)
            ev = build_domain_event(
                "Status_Restored",
                {
                    "target_entity_id": entity_id,
                    "success_rate": round(rate, 4),
                    "consecutive_success_count": need,
                },
            )
            actions.append({"type": "Status_Restored", "result": emit_domain_and_route(repo, ev)})
            _reload_bucket()
            stats = load_stats(repo)
            bucket = _entity_bucket(stats, entity_id)
            bucket["samples"] = samples[-int(thresholds.get("redemption_success_count", 3)) :]
            save_stats(repo, stats)
            mark_consumed(repo, asset_id)
            event_path.unlink(missing_ok=True)
            return {
                "ok": True,
                "asset_id": asset_id,
                "entity_id": entity_id,
                "status": bucket.get("status"),
                "actions": actions,
                "purged": True,
            }

    if status == "healthy":
        min_batch = int(thresholds.get("batch_min_events", 10))
        abrupt_min = int(thresholds.get("abrupt_drop_min_samples", 3))
        rate = _success_rate(bucket["samples"])
        avg_ms = _avg_duration(bucket["samples"])
        degraded = False
        reason = ""
        if len(bucket["samples"]) >= min_batch and rate < float(thresholds.get("success_rate_min", 0.85)):
            degraded = True
            reason = "success_rate_below_threshold"
        elif len(bucket["samples"]) >= abrupt_min and rate < float(thresholds.get("success_rate_min", 0.85)):
            degraded = True
            reason = "abrupt_success_rate_drop"
        elif len(bucket["samples"]) >= 5 and avg_ms > float(thresholds.get("latency_ms_p95_threshold", 30000)):
            degraded = True
            reason = "latency_threshold"

        if degraded:
            attempts = int(bucket.get("recovery_attempts") or 0) + 1
            bucket["recovery_attempts"] = attempts
            bucket["status"] = "degraded"
            bucket["degraded_at"] = _iso_now()
            bucket["structure_valid"] = False
            bucket["consecutive_success_count"] = 0
            save_stats(repo, stats)
            ev = build_domain_event(
                "Tool_Degraded",
                {
                    "target_entity_id": entity_id,
                    "reason": reason,
                    "success_rate": round(rate, 4),
                    "recovery_attempt": attempts,
                    "avg_duration_ms": round(avg_ms, 2),
                },
            )
            actions.append({"type": "Tool_Degraded", "result": emit_domain_and_route(repo, ev)})
            _reload_bucket()

    stats = load_stats(repo)
    bucket = _entity_bucket(stats, entity_id)
    bucket["samples"] = samples
    if status in ("degraded", "pending_redemption", "healthy", "deprecated"):
        pass  # disk already has latest status from incremental saves
    save_stats(repo, stats)
    mark_consumed(repo, asset_id)
    event_path.unlink(missing_ok=True)

    return {
        "ok": True,
        "asset_id": asset_id,
        "entity_id": entity_id,
        "status": bucket.get("status"),
        "actions": actions,
        "purged": True,
    }
