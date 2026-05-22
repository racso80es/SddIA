# -*- coding: utf-8 -*-
"""Utilidades compartidas bus EDA: topología V3, testigos suscriptor, idempotencia."""

from __future__ import annotations

import json
import re
import tempfile
from datetime import datetime, timezone
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

ECST_GATE_SUBSCRIBER = "ecst-gate"

_DEFAULT_EVENT_BUS = ".events"


def _iso_now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def _normalize_rel(path: str) -> str:
    p = path.replace("\\", "/")
    if p.startswith("./"):
        return p[2:]
    return p


def _load_cumulo(repo: Path) -> dict[str, Any]:
    cfg_path = repo / "SddIA" / "core" / "cumulo.paths.json"
    return json.loads(cfg_path.read_text(encoding="utf-8"))


def load_eda_bus(repo: Path) -> dict[str, str]:
    """Topología plana del bus V3 (padre + testigos suscriptor)."""
    event_bus = _normalize_rel(_DEFAULT_EVENT_BUS)
    defaults = {
        "event_bus": event_bus,
        "pending": f"{event_bus}/pending",
        "subscriber_processing": f"{event_bus}/subscribers/processing",
        "subscriber_processed": f"{event_bus}/subscribers/processed",
        "subscriber_dead_letter": f"{event_bus}/subscribers/dead-letter",
        "subscriptions": "SddIA/core/event-subscriptions.json",
    }
    try:
        cfg = _load_cumulo(repo)
        if isinstance(cfg.get("event_bus"), str) and cfg["event_bus"].strip():
            event_bus = _normalize_rel(cfg["event_bus"].strip())
            defaults["event_bus"] = event_bus
        bus = cfg.get("eda_bus") or {}
        if isinstance(bus.get("pending"), str) and bus["pending"]:
            defaults["pending"] = _normalize_rel(bus["pending"])
        subs = bus.get("subscribers") or {}
        if isinstance(subs, dict):
            for key, flat_key in (
                ("processing", "subscriber_processing"),
                ("processed", "subscriber_processed"),
                ("dead_letter", "subscriber_dead_letter"),
            ):
                if isinstance(subs.get(key), str) and subs[key]:
                    defaults[flat_key] = _normalize_rel(subs[key])
        if isinstance(bus.get("subscriptions"), str) and bus["subscriptions"]:
            defaults["subscriptions"] = bus["subscriptions"]
    except (OSError, ValueError):
        pass

    # Alias planos para consumidores legacy que usan processing/processed/dead_letter
    defaults["processing"] = defaults["subscriber_processing"]
    defaults["processed"] = defaults["subscriber_processed"]
    defaults["dead_letter"] = defaults["subscriber_dead_letter"]
    return defaults


def ensure_event_bus_topology(repo: Path) -> dict[str, str]:
    """Crea idempotentemente pending/ y subscribers/{processing,processed,dead-letter}/."""
    bus = load_eda_bus(repo)
    for key in (
        "pending",
        "subscriber_processing",
        "subscriber_processed",
        "subscriber_dead_letter",
    ):
        (repo / bus[key]).mkdir(parents=True, exist_ok=True)
    return bus


def subscriber_id(subscriber: dict[str, Any]) -> str:
    """Identificador único del suscriptor para nombre de testigo."""
    agent = subscriber.get("agent")
    if not isinstance(agent, str) or not agent.strip():
        return "unknown"
    agent = agent.strip()
    for key in ("process", "action", "tool"):
        value = subscriber.get(key)
        if isinstance(value, str) and value.strip():
            return f"{agent}.{value.strip()}"
    return agent


def witness_filename(event_uuid: str, subscriber_name: str) -> str:
    return f"{event_uuid}.{subscriber_name}.json"


def _write_json_atomic(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    text = json.dumps(payload, indent=2, ensure_ascii=False) + "\n"
    fd, tmp_name = tempfile.mkstemp(
        dir=str(path.parent), suffix=".tmp", prefix=f".{path.stem}-"
    )
    tmp = Path(tmp_name)
    try:
        with open(fd, "w", encoding="utf-8") as fh:
            fh.write(text)
        tmp.replace(path)
    finally:
        if tmp.is_file():
            tmp.unlink(missing_ok=True)


def write_processing_witness(
    repo: Path,
    bus: dict[str, str],
    *,
    event_uuid: str,
    subscriber_name: str,
    event_type: str,
) -> Path:
    dest = repo / bus["subscriber_processing"] / witness_filename(event_uuid, subscriber_name)
    _write_json_atomic(
        dest,
        {
            "event_uuid": event_uuid,
            "subscriber": subscriber_name,
            "state": "processing",
            "started_at": _iso_now(),
            "event_type": event_type,
        },
    )
    return dest


def promote_witness(
    repo: Path,
    bus: dict[str, str],
    *,
    event_uuid: str,
    subscriber_name: str,
    to_state: str,
    extra: dict[str, Any] | None = None,
) -> Path:
    from_key = "subscriber_processing"
    to_key = "subscriber_processed" if to_state == "processed" else "subscriber_dead_letter"
    src = repo / bus[from_key] / witness_filename(event_uuid, subscriber_name)
    dest = repo / bus[to_key] / witness_filename(event_uuid, subscriber_name)
    if not src.is_file():
        raise FileNotFoundError(f"testigo processing ausente: {src}")
    body = json.loads(src.read_text(encoding="utf-8"))
    body["state"] = to_state
    now = _iso_now()
    if to_state == "processed":
        body["completed_at"] = now
    else:
        body["failed_at"] = now
        body.setdefault("error_trace", "unknown failure")
    if extra:
        body.update(extra)
    dest.parent.mkdir(parents=True, exist_ok=True)
    _write_json_atomic(dest, body)
    src.unlink(missing_ok=True)
    return dest


def list_witnesses(repo: Path, bus: dict[str, str], state_key: str, event_uuid: str) -> list[Path]:
    folder = repo / bus[state_key]
    if not folder.is_dir():
        return []
    return sorted(folder.glob(f"{event_uuid}.*.json"))


def required_subscriber_ids(registry: dict[str, Any], event_type: str) -> list[str]:
    subscribers = registry.get(event_type) or []
    if not isinstance(subscribers, list):
        return []
    ids: list[str] = []
    for sub in subscribers:
        if isinstance(sub, dict):
            ids.append(subscriber_id(sub))
    return ids


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


def _legacy_bus_dirs(repo: Path) -> list[Path]:
    legacy = [
        repo / "docs/events/pending",
        repo / "docs/events/processing",
        repo / "docs/events/processed",
        repo / "docs/events/dead-letter",
    ]
    return [d for d in legacy if d.is_dir()]


def iter_bus_event_files(repo: Path) -> list[Path]:
    """Instancias ECST padre: pending V3 + legacy docs/events."""
    bus = load_eda_bus(repo)
    files: list[Path] = []
    pending = repo / bus["pending"]
    if pending.is_dir():
        files.extend(sorted(pending.glob("*.json")))
    for legacy_dir in _legacy_bus_dirs(repo):
        if legacy_dir == pending:
            continue
        files.extend(sorted(legacy_dir.glob("*.json")))
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


def archive_processed_witnesses(repo: Path, bus: dict[str, str], event_uuid: str) -> int:
    """Elimina testigos processed/ del evento tras purga del padre."""
    removed = 0
    for path in list_witnesses(repo, bus, "subscriber_processed", event_uuid):
        path.unlink(missing_ok=True)
        removed += 1
    return removed
