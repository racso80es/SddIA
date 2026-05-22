# -*- coding: utf-8 -*-
"""Utilidades compartidas bus EDA: topología fractal, idempotencia, umbral DLT."""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any

DOMAIN_ENTITY_TYPES = frozenset(
    {
        "Domain_Entity_Created",
        "Domain_Entity_Updated",
        "Domain_Entity_Deleted",
    }
)

UUID4_RE = re.compile(
    r"^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$",
    re.I,
)

DLT_PLACEHOLDER_HASHES = frozenset(
    {
        "sha256:pending-forge",
        "sha256:pending",
        "",
    }
)

BACKFILL_EMITTERS = frozenset({"cumulo-eda-backfill"})


def load_eda_bus(repo: Path) -> dict[str, str]:
    defaults = {
        "pending": "docs/events/pending",
        "processing": "docs/events/processing",
        "processed": "docs/events/processed",
        "dead_letter": "docs/events/dead-letter",
        "subscriptions": "SddIA/core/event-subscriptions.json",
    }
    cfg_path = repo / "SddIA" / "core" / "cumulo.paths.json"
    try:
        cfg = json.loads(cfg_path.read_text(encoding="utf-8"))
        bus = cfg.get("eda_bus") or {}
        out = dict(defaults)
        for k in defaults:
            if isinstance(bus.get(k), str) and bus[k]:
                out[k] = bus[k]
        return out
    except (OSError, ValueError):
        return defaults


def resolve_origin_topology(payload: dict[str, Any]) -> str:
    topo = payload.get("origin_topology")
    if isinstance(topo, str) and topo in ("core", "local"):
        return topo
    return "core"


def subscriber_applies_to_topology(subscriber: dict[str, Any], origin_topology: str) -> bool:
    applies = subscriber.get("applies_to_origin_topology")
    if not isinstance(applies, list) or not applies:
        applies = ["core"]
    return origin_topology in applies


def is_backfill_emitter(emitter_agent: str | None) -> bool:
    return isinstance(emitter_agent, str) and emitter_agent in BACKFILL_EMITTERS


def dlt_threshold_ok(event: dict[str, Any]) -> tuple[bool, str]:
    """Umbral DLT para Domain_Entity_Created core."""
    if event.get("event_type") != "Domain_Entity_Created":
        return True, "not-create"
    payload = event.get("payload")
    if not isinstance(payload, dict):
        return False, "payload-missing"
    if resolve_origin_topology(payload) != "core":
        return False, "topology-local"
    entity_uuid = payload.get("entity_uuid")
    if not isinstance(entity_uuid, str) or not UUID4_RE.match(entity_uuid):
        return False, "invalid-uuid"
    hnew = payload.get("hash_signature_new")
    if not isinstance(hnew, str) or not hnew.startswith("sha256:"):
        return False, "invalid-hash-prefix"
    if hnew.lower() in DLT_PLACEHOLDER_HASHES:
        return False, "placeholder-hash"
    entity_class = payload.get("entity_class")
    allowed = {
        "process",
        "agent",
        "skill",
        "tool",
        "action",
        "norm",
        "codex",
        "event",
    }
    if entity_class not in allowed:
        return False, "invalid-entity-class"
    return True, "ok"


def iter_bus_event_files(repo: Path) -> list[Path]:
    bus = load_eda_bus(repo)
    files: list[Path] = []
    for key in ("pending", "processing", "processed", "dead_letter"):
        d = repo / bus[key]
        if d.is_dir():
            files.extend(sorted(d.glob("*.json")))
    return files


def find_existing_domain_event(
    repo: Path,
    entity_uuid: str,
    lifecycle_operation: str,
    event_type: str | None = None,
) -> dict[str, Any] | None:
    for path in iter_bus_event_files(repo):
        try:
            body = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            continue
        if event_type and body.get("event_type") != event_type:
            continue
        payload = body.get("payload") or {}
        if not isinstance(payload, dict):
            continue
        if (
            payload.get("entity_uuid") == entity_uuid
            and payload.get("lifecycle_operation") == lifecycle_operation
        ):
            return {
                "event_id": body.get("event_id"),
                "target_path": str(path.relative_to(repo)).replace("\\", "/"),
                "event_type": body.get("event_type"),
            }
    return None


def inject_domain_entity_topology_defaults(event: dict[str, Any]) -> None:
    if event.get("event_type") not in DOMAIN_ENTITY_TYPES:
        return
    payload = event.get("payload")
    if isinstance(payload, dict) and "origin_topology" not in payload:
        payload["origin_topology"] = "core"


_BRANCH_NUMERIC_SUFFIX_RE = re.compile(r"^(?P<base>.+)-\d{10,}$")


def infer_persist_ref_from_branch(repo: Path, branch: str) -> str | None:
    """Resuelve persist_ref existente; ignora sufijo numérico tipo Jules en la rama."""
    b = branch.strip()
    candidates: list[str] = []
    if b.startswith("feat/"):
        slug = b[5:]
        candidates.append(f"docs/features/{slug}")
        m = _BRANCH_NUMERIC_SUFFIX_RE.match(slug)
        if m:
            candidates.append(f"docs/features/{m.group('base')}")
    elif b.startswith("fix/"):
        slug = b[4:]
        candidates.append(f"docs/fixes/{slug}")
        m = _BRANCH_NUMERIC_SUFFIX_RE.match(slug)
        if m:
            candidates.append(f"docs/fixes/{m.group('base')}")
    seen: set[str] = set()
    for ref in candidates:
        if ref in seen:
            continue
        seen.add(ref)
        if (repo / ref).is_dir():
            return ref
    return None


def github_pr_merged(pr_url: str) -> bool:
    """True si gh reporta el PR en estado MERGED (retroactivo / handoff)."""
    import subprocess

    url = pr_url.strip()
    if not url:
        return False
    try:
        proc = subprocess.run(
            ["gh", "pr", "view", url, "--json", "state"],
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            check=False,
        )
    except OSError:
        return False
    if proc.returncode != 0 or not (proc.stdout or "").strip():
        return False
    try:
        data = json.loads(proc.stdout)
    except json.JSONDecodeError:
        return False
    return data.get("state") == "MERGED"
