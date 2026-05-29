# -*- coding: utf-8 -*-
"""Utilidades compartidas bus EDA: topología V3+ simétrica, testigos y cabeceras."""

from __future__ import annotations

import json
import os
import re
import shutil
import tempfile
import time
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

_GH_PR_URL_RE = re.compile(r"github\.com/[^/]+/[^/]+/pull/(\d+)", re.I)

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

# Alias de claves de carpeta testigo (legacy V3 → V3+)
_WITNESS_KEY_ALIASES: dict[str, str] = {
    "subscriber_processing": "processing_subscribers",
    "subscriber_processed": "processed_subscribers",
    "subscriber_dead_letter": "dead_letter_subscribers",
    "processing": "processing_subscribers",
    "processed": "processed_subscribers",
    "dead_letter": "dead_letter_subscribers",
}

_HEADER_STATES = frozenset({"processing", "processed", "dead_letter"})


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


def _resolve_witness_key(state_key: str) -> str:
    return _WITNESS_KEY_ALIASES.get(state_key, state_key)


def _bus_defaults_from_root(event_bus: str) -> dict[str, str]:
    event_bus = _normalize_rel(event_bus.rstrip("/"))
    return {
        "event_bus": event_bus,
        "pending": f"{event_bus}/pending",
        "processing": f"{event_bus}/processing",
        "processing_subscribers": f"{event_bus}/processing/subscribers",
        "processed": f"{event_bus}/processed",
        "processed_subscribers": f"{event_bus}/processed/subscribers",
        "dead_letter": f"{event_bus}/dead-letter",
        "dead_letter_subscribers": f"{event_bus}/dead-letter/subscribers",
        "subscriptions": "SddIA/core/event-domain-subscriptions.json",
    }


def load_eda_bus(repo: Path) -> dict[str, str]:
    """Topología plana del bus V3+ (cabeceras por estado + subscribers anidados)."""
    env_bus = os.environ.get("EVENT_BUS_PATH", "").strip()
    if env_bus:
        defaults = _bus_defaults_from_root(env_bus)
    else:
        event_bus = _normalize_rel(_DEFAULT_EVENT_BUS)
        defaults = _bus_defaults_from_root(event_bus)
    try:
        cfg = _load_cumulo(repo)
        if not env_bus and isinstance(cfg.get("event_bus"), str) and cfg["event_bus"].strip():
            event_bus = _normalize_rel(cfg["event_bus"].strip())
            defaults = _bus_defaults_from_root(event_bus)

        bus = cfg.get("eda_bus") or {}
        if not env_bus:
            if isinstance(bus.get("pending"), str) and bus["pending"]:
                defaults["pending"] = _normalize_rel(bus["pending"])

            for key in ("processing", "processed", "dead_letter"):
                if isinstance(bus.get(key), str) and bus[key]:
                    defaults[key] = _normalize_rel(bus[key])
                    defaults[f"{key}_subscribers"] = _normalize_rel(
                        f"{bus[key].rstrip('/')}/subscribers"
                    )

            subs = bus.get("subscribers") or {}
            if isinstance(subs, dict):
                legacy_map = (
                    ("processing", "processing_subscribers"),
                    ("processed", "processed_subscribers"),
                    ("dead_letter", "dead_letter_subscribers"),
                )
                for legacy_key, flat_key in legacy_map:
                    if isinstance(subs.get(legacy_key), str) and subs[legacy_key]:
                        defaults[flat_key] = _normalize_rel(subs[legacy_key])

        if isinstance(bus.get("subscriptions"), str) and bus["subscriptions"]:
            defaults["subscriptions"] = bus["subscriptions"]
    except (OSError, ValueError):
        pass

    # Alias legacy para consumidores no migrados
    defaults["subscriber_processing"] = defaults["processing_subscribers"]
    defaults["subscriber_processed"] = defaults["processed_subscribers"]
    defaults["subscriber_dead_letter"] = defaults["dead_letter_subscribers"]
    return defaults


def ensure_event_bus_topology(repo: Path) -> dict[str, str]:
    """Crea idempotentemente pending/, estados con cabecera y subscribers/."""
    bus = load_eda_bus(repo)
    for key in (
        "pending",
        "processing",
        "processing_subscribers",
        "processed",
        "processed_subscribers",
        "dead_letter",
        "dead_letter_subscribers",
    ):
        (repo / bus[key]).mkdir(parents=True, exist_ok=True)
    return bus


def header_path(bus: dict[str, str], state: str, event_uuid: str) -> Path:
    if state not in _HEADER_STATES:
        raise ValueError(f"estado cabecera inválido: {state}")
    return Path(bus[state]) / f"{event_uuid}.json"


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


def safe_remove_path(path: Path, *, retries: int = 3, delay_s: float = 0.05) -> bool:
    """Elimina un archivo con reintentos (absorción latencia E/S Windows)."""
    if not path.is_file():
        return True
    last_err: OSError | None = None
    for attempt in range(max(1, retries)):
        try:
            path.unlink()
            return True
        except (PermissionError, OSError) as exc:
            last_err = exc if isinstance(exc, OSError) else OSError(exc)
            if attempt + 1 < retries:
                time.sleep(delay_s)
    if last_err is not None:
        return False
    return not path.is_file()


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


def _copy_header_atomic(source: Path, dest: Path) -> None:
    if dest.is_file():
        return
    dest.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source, dest)


def ensure_state_header(
    repo: Path,
    bus: dict[str, str],
    state: str,
    event_uuid: str,
    source_path: Path,
) -> Path:
    """Réplica cabecera ECST en processing/processed/dead-letter si ausente."""
    dest = repo / header_path(bus, state, event_uuid)
    _copy_header_atomic(source_path, dest)
    return dest


def ensure_processing_header(
    repo: Path,
    bus: dict[str, str],
    event_uuid: str,
    pending_path: Path,
) -> Path:
    return ensure_state_header(repo, bus, "processing", event_uuid, pending_path)


def list_witnesses(
    repo: Path, bus: dict[str, str], state_key: str, event_uuid: str
) -> list[Path]:
    folder_key = _resolve_witness_key(state_key)
    folder = repo / bus[folder_key]
    if not folder.is_dir():
        return []
    return sorted(folder.glob(f"{event_uuid}.*.json"))


def witness_exists(
    repo: Path, bus: dict[str, str], state_key: str, event_uuid: str, subscriber_name: str
) -> bool:
    folder_key = _resolve_witness_key(state_key)
    path = repo / bus[folder_key] / witness_filename(event_uuid, subscriber_name)
    return path.is_file()


def terminal_witness_exists(
    repo: Path, bus: dict[str, str], event_uuid: str, subscriber_name: str
) -> bool:
    return witness_exists(
        repo, bus, "processed_subscribers", event_uuid, subscriber_name
    ) or witness_exists(repo, bus, "dead_letter_subscribers", event_uuid, subscriber_name)


def write_processing_witness(
    repo: Path,
    bus: dict[str, str],
    *,
    event_uuid: str,
    subscriber_name: str,
    event_type: str,
    dispatch_mode: str = "async",
) -> Path:
    dest = (
        repo
        / bus["processing_subscribers"]
        / witness_filename(event_uuid, subscriber_name)
    )
    _write_json_atomic(
        dest,
        {
            "event_uuid": event_uuid,
            "subscriber": subscriber_name,
            "state": "processing",
            "started_at": _iso_now(),
            "event_type": event_type,
            "dispatch_mode": dispatch_mode,
        },
    )
    return dest


def _delegation_meta(subscriber: dict[str, Any], exit_code: int) -> dict[str, Any]:
    kind = "unknown"
    target = "unknown"
    if isinstance(subscriber.get("process"), str) and subscriber["process"].strip():
        kind = "process"
        target = subscriber["process"].strip()
    elif isinstance(subscriber.get("action"), str) and subscriber["action"].strip():
        kind = "action"
        target = subscriber["action"].strip()
    elif isinstance(subscriber.get("tool"), str) and subscriber["tool"].strip():
        kind = "tool"
        target = subscriber["tool"].strip()
    return {"kind": kind, "target": target, "exit_code": exit_code}


def promote_witness(
    repo: Path,
    bus: dict[str, str],
    *,
    event_uuid: str,
    subscriber_name: str,
    to_state: str,
    extra: dict[str, Any] | None = None,
    pending_header: Path | None = None,
) -> Path:
    from_key = "processing_subscribers"
    to_key = (
        "processed_subscribers" if to_state == "processed" else "dead_letter_subscribers"
    )
    header_state = "processed" if to_state == "processed" else "dead_letter"
    src = repo / bus[from_key] / witness_filename(event_uuid, subscriber_name)
    dest = repo / bus[to_key] / witness_filename(event_uuid, subscriber_name)
    if not src.is_file():
        raise FileNotFoundError(f"testigo processing ausente: {src}")
    body = json.loads(src.read_text(encoding="utf-8"))
    body["state"] = "dead-letter" if to_state == "dead-letter" else "processed"
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
    if pending_header is not None and pending_header.is_file():
        ensure_state_header(repo, bus, header_state, event_uuid, pending_header)
    return dest


def terminal_subscriber_names(
    repo: Path, bus: dict[str, str], event_uuid: str
) -> set[str]:
    names: set[str] = set()
    for key in ("processed_subscribers", "dead_letter_subscribers"):
        for path in list_witnesses(repo, bus, key, event_uuid):
            suffix = path.name[len(event_uuid) + 1 : -5]
            if suffix:
                names.add(suffix)
    return names


def in_flight_subscriber_names(
    repo: Path, bus: dict[str, str], event_uuid: str
) -> set[str]:
    names: set[str] = set()
    for path in list_witnesses(repo, bus, "processing_subscribers", event_uuid):
        suffix = path.name[len(event_uuid) + 1 : -5]
        if suffix:
            names.add(suffix)
    return names


def maybe_purge_processing_header(
    repo: Path,
    bus: dict[str, str],
    event_uuid: str,
    registry: dict[str, Any],
    event_type: str,
    origin_topology: str,
) -> bool:
    """Elimina cabecera processing/ si todos los suscriptores aplicables están terminales."""
    required: list[str] = []
    for sub in registry.get(event_type) or []:
        if isinstance(sub, dict) and subscriber_applies_to_topology(sub, origin_topology):
            required.append(subscriber_id(sub))
    if not required:
        return False
    terminals = terminal_subscriber_names(repo, bus, event_uuid)
    in_flight = in_flight_subscriber_names(repo, bus, event_uuid)
    if not set(required).issubset(terminals):
        return False
    if in_flight & set(required):
        return False
    header = repo / header_path(bus, "processing", event_uuid)
    if header.is_file():
        header.unlink(missing_ok=True)
        return True
    return False


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
        "suite",
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
    """Instancias ECST padre: pending, cabeceras processing/processed V3+, legacy docs/events."""
    bus = load_eda_bus(repo)
    files: list[Path] = []
    seen: set[Path] = set()

    def _add_header_dir(dir_path: Path) -> None:
        if not dir_path.is_dir():
            return
        for path in sorted(dir_path.glob("*.json")):
            resolved = path.resolve()
            if resolved not in seen:
                seen.add(resolved)
                files.append(path)

    _add_header_dir(repo / bus["pending"])
    for state in ("processing", "processed"):
        _add_header_dir(repo / bus[state])

    pending_resolved = (repo / bus["pending"]).resolve()
    for legacy_dir in _legacy_bus_dirs(repo):
        if legacy_dir.resolve() == pending_resolved:
            continue
        _add_header_dir(legacy_dir)

    return sorted(files, key=lambda p: str(p.relative_to(repo)).replace("\\", "/"))


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
    from workspace_utils import resolve_documentation_features_path, resolve_documentation_fixes_path

    features_prefix = resolve_documentation_features_path(repo)
    fixes_prefix = resolve_documentation_fixes_path(repo)
    b = branch.strip()
    candidates: list[str] = []
    if b.startswith("feat/"):
        slug = b[5:]
        candidates.append(f"{features_prefix}/{slug}")
        m = _BRANCH_NUMERIC_SUFFIX_RE.match(slug)
        if m:
            candidates.append(f"{features_prefix}/{m.group('base')}")
    elif b.startswith("fix/"):
        slug = b[4:]
        candidates.append(f"{fixes_prefix}/{slug}")
        m = _BRANCH_NUMERIC_SUFFIX_RE.match(slug)
        if m:
            candidates.append(f"{fixes_prefix}/{m.group('base')}")
    seen: set[str] = set()
    for ref in candidates:
        if ref in seen:
            continue
        seen.add(ref)
        if (repo / ref).is_dir():
            return ref
    return None


def gh_executable() -> str | None:
    """Ruta a gh; override opcional vía SDDIA_GH_EXECUTABLE."""
    override = os.environ.get("SDDIA_GH_EXECUTABLE", "").strip()
    if override:
        p = Path(override)
        return str(p.resolve()) if p.is_file() else None
    return shutil.which("gh")


def parse_pr_number(pr_url: str | None) -> int | None:
    if not isinstance(pr_url, str) or not pr_url.strip():
        return None
    m = _GH_PR_URL_RE.search(pr_url.strip())
    return int(m.group(1)) if m else None


def _run_git(repo: Path, args: list[str]) -> tuple[int, str, str]:
    import subprocess

    proc = subprocess.run(
        ["git", *args],
        cwd=str(repo.resolve()),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    return proc.returncode, (proc.stdout or "").strip(), (proc.stderr or "").strip()


def _gh_pr_state(pr_url: str) -> str | None:
    """Estado GitHub del PR (MERGED|OPEN|CLOSED|…) o None si gh indisponible."""
    import subprocess

    gh = gh_executable()
    url = pr_url.strip()
    if not gh or not url:
        return None
    try:
        proc = subprocess.run(
            [gh, "pr", "view", url, "--json", "state"],
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            check=False,
        )
    except OSError:
        return None
    if proc.returncode != 0 or not (proc.stdout or "").strip():
        return None
    try:
        data = json.loads(proc.stdout)
    except json.JSONDecodeError:
        return None
    state = data.get("state")
    return state if isinstance(state, str) else None


def github_pr_merged(pr_url: str) -> bool:
    """True si gh reporta el PR en estado MERGED (retroactivo / handoff)."""
    return _gh_pr_state(pr_url) == "MERGED"


def _branch_exists_on_remote(repo: Path, branch: str, *, fetch: bool = True) -> bool:
    if fetch:
        _run_git(repo, ["fetch", "origin", "--prune"])
    code, _, _ = _run_git(repo, ["rev-parse", "--verify", f"origin/{branch}"])
    return code == 0


def _merged_via_pull_ref(repo: Path, pr_number: int, target_branch: str = "main") -> bool:
    remote_ref = f"refs/remotes/origin/.sddia/pr-{pr_number}-head"
    fetch_spec = f"pull/{pr_number}/head:{remote_ref}"
    code, _, _ = _run_git(repo, ["fetch", "origin", fetch_spec])
    if code != 0:
        return False
    local_ref = f"origin/.sddia/pr-{pr_number}-head"
    target = f"origin/{target_branch.strip() or 'main'}"
    code_tgt, _, _ = _run_git(repo, ["rev-parse", "--verify", target])
    if code_tgt != 0:
        return False
    code_anc, _, _ = _run_git(repo, ["merge-base", "--is-ancestor", local_ref, target])
    return code_anc == 0


def resolve_pull_request_lifecycle(
    repo: Path,
    *,
    branch: str,
    pr_url: str | None = None,
    target_branch: str = "main",
) -> dict[str, Any]:
    """Resuelve merge y presencia remota antes de invocar pull-request-review."""
    branch = branch.strip()
    target = target_branch.strip() or "main"
    diagnostics: list[str] = []
    pr_number = parse_pr_number(pr_url)

    if isinstance(pr_url, str) and pr_url.strip():
        gh_state = _gh_pr_state(pr_url)
        if gh_state == "MERGED":
            diagnostics.append("gh:MERGED")
            return {
                "merged": True,
                "source": "gh",
                "branch_on_remote": _branch_exists_on_remote(repo, branch, fetch=False),
                "pr_number": pr_number,
                "diagnostics": diagnostics,
            }
        if gh_state == "OPEN":
            diagnostics.append("gh:OPEN")
        elif gh_state == "CLOSED":
            diagnostics.append("gh:CLOSED")
            return {
                "merged": False,
                "source": "gh",
                "branch_on_remote": _branch_exists_on_remote(repo, branch, fetch=True),
                "pr_number": pr_number,
                "diagnostics": diagnostics,
            }
        elif gh_state is None:
            diagnostics.append("gh:UNAVAILABLE" if not gh_executable() else "gh:ERROR")
    else:
        diagnostics.append("pr_url:absent")

    branch_on_remote = _branch_exists_on_remote(repo, branch, fetch=True)
    if branch_on_remote:
        diagnostics.append("branch:remote-present")
        return {
            "merged": False,
            "source": "branch-remote",
            "branch_on_remote": True,
            "pr_number": pr_number,
            "diagnostics": diagnostics,
        }

    diagnostics.append("branch:remote-absent")
    if pr_number is not None and _merged_via_pull_ref(repo, pr_number, target):
        diagnostics.append("git-pull-ref:ancestor")
        return {
            "merged": True,
            "source": "git-pull-ref",
            "branch_on_remote": False,
            "pr_number": pr_number,
            "diagnostics": diagnostics,
        }

    return {
        "merged": None,
        "source": "unknown",
        "branch_on_remote": False,
        "pr_number": pr_number,
        "diagnostics": diagnostics,
    }


def processed_subscriber_names(
    repo: Path, bus: dict[str, str], event_uuid: str
) -> set[str]:
    names: set[str] = set()
    for path in list_witnesses(repo, bus, "processed_subscribers", event_uuid):
        suffix = path.name[len(event_uuid) + 1 : -5]
        if suffix:
            names.add(suffix)
    return names


def applicable_subscriber_ids_for_event(
    registry: dict[str, Any], event_type: str, payload: dict[str, Any]
) -> list[str]:
    """Suscriptores que aplican a origin_topology, sin fallback global."""
    origin = resolve_origin_topology(payload)
    return [
        subscriber_id(sub)
        for sub in registry.get(event_type) or []
        if isinstance(sub, dict) and subscriber_applies_to_topology(sub, origin)
    ]


def required_subscriber_ids_for_event(
    registry: dict[str, Any], event_type: str, payload: dict[str, Any]
) -> list[str]:
    origin = resolve_origin_topology(payload)
    ids: list[str] = []
    for sub in registry.get(event_type) or []:
        if isinstance(sub, dict) and subscriber_applies_to_topology(sub, origin):
            ids.append(subscriber_id(sub))
    if ids:
        return ids
    return required_subscriber_ids(registry, event_type)


def finalize_kaizen_terminal(
    repo: Path,
    bus: dict[str, str],
    event_uuid: str,
    pending_path: Path,
    registry: dict[str, Any],
    event_type: str,
    origin_topology: str,
) -> dict[str, int]:
    """Retira padre de pending/ cuando Kaizen está terminal (DL + suscriptores cerrados)."""
    counts = {"pending": 0, "headers": 0}
    dead_header = repo / header_path(bus, "dead_letter", event_uuid)
    if not dead_header.is_file() and pending_path.is_file():
        ensure_state_header(repo, bus, "dead_letter", event_uuid, pending_path)
        counts["headers"] += 1
    if pending_path.is_file() and safe_remove_path(pending_path):
        counts["pending"] = 1
    if maybe_purge_processing_header(
        repo, bus, event_uuid, registry, event_type, origin_topology
    ):
        counts["headers"] += 1
    return counts


def try_sweep_event(
    repo: Path,
    bus: dict[str, str],
    event_uuid: str,
    registry: dict[str, Any] | None = None,
) -> dict[str, Any]:
    """Intenta purgar el padre en pending/ cuando hay consenso de suscriptores."""
    base: dict[str, Any] = {"event_uuid": event_uuid, "purged": False}
    pending_path = repo / bus["pending"] / f"{event_uuid}.json"
    if not pending_path.is_file():
        return {**base, "status": "absent"}

    try:
        event = json.loads(pending_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {**base, "status": "invalid-json"}

    event_type = event.get("event_type")
    if not isinstance(event_type, str) or not event_type:
        return {**base, "status": "missing-event_type"}

    payload = event.get("payload") if isinstance(event.get("payload"), dict) else {}

    if registry is None:
        subs_path = repo / bus["subscriptions"]
        try:
            registry = json.loads(subs_path.read_text(encoding="utf-8-sig"))
        except (OSError, json.JSONDecodeError):
            return {**base, "status": "invalid-registry", "event_type": event_type}

    applicable = applicable_subscriber_ids_for_event(registry, event_type, payload)

    dead = list_witnesses(repo, bus, "dead_letter_subscribers", event_uuid)
    if dead:
        origin = resolve_origin_topology(payload)
        in_flight = in_flight_subscriber_names(repo, bus, event_uuid)
        terminals = terminal_subscriber_names(repo, bus, event_uuid)
        if (
            applicable
            and set(applicable).issubset(terminals)
            and not (in_flight & set(applicable))
        ):
            finalized = finalize_kaizen_terminal(
                repo, bus, event_uuid, pending_path, registry, event_type, origin
            )
            return {
                **base,
                "status": "kaizen-finalized",
                "purged": True,
                "finalized": True,
                "event_type": event_type,
                "dead_letter_witnesses": [p.name for p in dead],
                **finalized,
            }
        return {
            **base,
            "status": "kaizen",
            "event_type": event_type,
            "dead_letter_witnesses": [p.name for p in dead],
        }
    if not applicable:
        archived = archive_event_after_sweep(repo, bus, event_uuid, event_type=event_type)
        return {
            **base,
            "status": "purged",
            "purged": True,
            "event_type": event_type,
            **archived,
        }

    in_flight = in_flight_subscriber_names(repo, bus, event_uuid)
    overlap = in_flight & set(applicable)
    if overlap:
        return {
            **base,
            "status": "in-flight",
            "event_type": event_type,
            "in_flight": sorted(overlap),
        }

    done = processed_subscriber_names(repo, bus, event_uuid)
    if set(applicable).issubset(done):
        archived = archive_event_after_sweep(repo, bus, event_uuid, event_type=event_type)
        return {
            **base,
            "status": "purged",
            "purged": True,
            "event_type": event_type,
            **archived,
        }

    return {
        **base,
        "status": "awaiting",
        "event_type": event_type,
        "pending_subscribers": sorted(set(applicable) - done),
    }


def archive_event_after_sweep(
    repo: Path,
    bus: dict[str, str],
    event_uuid: str,
    *,
    event_type: str | None = None,
) -> dict[str, int]:
    """Purgar pending, cabeceras processing/processed y testigos (sweep vacío)."""
    counts: dict[str, int] = {"witnesses": 0, "headers": 0, "pending": 0}

    if event_type is None:
        for state in ("processed", "processing"):
            header = repo / header_path(bus, state, event_uuid)
            if not header.is_file():
                continue
            try:
                body = json.loads(header.read_text(encoding="utf-8"))
            except (OSError, json.JSONDecodeError):
                continue
            if isinstance(body.get("event_type"), str):
                event_type = body["event_type"]
                break

    pending = repo / bus["pending"] / f"{event_uuid}.json"
    if pending.is_file():
        if safe_remove_path(pending):
            counts["pending"] = 1
    for state in ("processing", "processed"):
        header = repo / header_path(bus, state, event_uuid)
        if header.is_file() and safe_remove_path(header):
            counts["headers"] += 1
    for path in list_witnesses(repo, bus, "processed_subscribers", event_uuid):
        if safe_remove_path(path):
            counts["witnesses"] += 1
    return counts


def archive_processed_witnesses(repo: Path, bus: dict[str, str], event_uuid: str) -> int:
    """Compat legacy: delega en archive_event_after_sweep."""
    return archive_event_after_sweep(repo, bus, event_uuid)["witnesses"]


_FRACTAL_FAMILIES = frozenset({"telemetry", "orchestration", "domain"})


def load_eda_fractal(repo: Path) -> dict[str, str]:
    """Rutas runtime fractales (Simetría Fractal genoma ↔ ./.events/{family}/)."""
    defaults = {
        "telemetry": "./.events/telemetry",
        "orchestration": "./.events/orchestration",
        "domain": "./.events/domain",
        "telemetry_subscriptions": "SddIA/core/event-telemetry-subscriptions.json",
        "orchestration_subscriptions": "SddIA/core/event-orchestration-subscriptions.json",
        "domain_subscriptions": "SddIA/core/event-domain-subscriptions.json",
    }
    try:
        cfg = _load_cumulo(repo)
        fractal = cfg.get("eda_fractal") or {}
        if isinstance(fractal, dict):
            for key, value in fractal.items():
                if isinstance(value, str) and value.strip():
                    defaults[key] = _normalize_rel(value.strip())
    except (OSError, ValueError):
        pass
    return defaults


def ensure_fractal_bus_topology(repo: Path) -> dict[str, str]:
    """Crea idempotentemente telemetry/, orchestration/, domain/ bajo .events/."""
    fractal = load_eda_fractal(repo)
    for key in ("telemetry", "orchestration", "domain"):
        (repo / fractal[key]).mkdir(parents=True, exist_ok=True)
    return fractal


def write_fractal_event(repo: Path, event: dict[str, Any], family: str) -> dict[str, str]:
    """Escribe instancia ECST en la ruta fractal de la familia indicada."""
    if family not in _FRACTAL_FAMILIES:
        raise ValueError(f"invalid event family: {family}")
    fractal = ensure_fractal_bus_topology(repo)
    event_id = event.get("event_id")
    if not isinstance(event_id, str) or not event_id.strip():
        raise ValueError("event_id required")
    target = repo / fractal[family] / f"{event_id.strip()}.json"
    _write_json_atomic(target, event)
    return {
        "event_id": event_id.strip(),
        "target_path": str(target.relative_to(repo)).replace("\\", "/"),
        "family": family,
    }


def _telemetry_timestamp() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def build_raw_execution_finished_event(
    *,
    event_id: str,
    asset_id: str,
    exit_code: int,
    duration_ms: int,
    process_name: str,
    execution_id: str | None = None,
    workspace_path: str | None = None,
    capsule_id: str | None = None,
    telemetry_receipt: dict[str, Any] | None = None,
) -> dict[str, Any]:
    payload: dict[str, Any] = {
        "asset_id": asset_id,
        "exit_code": int(exit_code),
        "duration_ms": int(duration_ms),
        "process_name": process_name,
    }
    if execution_id:
        payload["execution_id"] = execution_id
    if workspace_path:
        payload["workspace_path"] = workspace_path
    if isinstance(capsule_id, str) and capsule_id.strip():
        payload["capsule_id"] = capsule_id.strip()
    if isinstance(telemetry_receipt, dict) and telemetry_receipt:
        payload["telemetry_receipt"] = telemetry_receipt
    return {
        "event_id": event_id,
        "event_type": "Raw_Execution_Finished",
        "event_family": "telemetry",
        "timestamp": _telemetry_timestamp(),
        "emitter_agent": "execute-process",
        "payload": payload,
        "delivery_state": {},
    }


def build_process_execution_completed_event(
    *,
    event_id: str,
    asset_id: str,
    process_name: str,
    status: str,
    workspace_path: str | None = None,
    execution_id: str | None = None,
    phase_count: int | None = None,
    persist_ref: str | None = None,
) -> dict[str, Any]:
    payload: dict[str, Any] = {
        "asset_id": asset_id,
        "process_name": process_name,
        "status": status,
    }
    if workspace_path:
        payload["workspace_path"] = workspace_path
    if execution_id:
        payload["execution_id"] = execution_id
    if phase_count is not None:
        payload["phase_count"] = int(phase_count)
    if persist_ref:
        payload["persist_ref"] = persist_ref
    return {
        "event_id": event_id,
        "event_type": "Process_Execution_Completed",
        "event_family": "orchestration",
        "timestamp": _telemetry_timestamp(),
        "emitter_agent": "execute-process",
        "payload": payload,
        "delivery_state": {},
    }


def load_radamanto_config(repo: Path) -> dict[str, Any]:
    """SSOT Radamanto: rutas locales + umbrales."""
    defaults: dict[str, Any] = {
        "stats": ".SddIA/radamanto/stats.json",
        "consumed": ".SddIA/radamanto/consumed.json",
        "thresholds": "SddIA/agents/radamanto.thresholds.json",
        "sandbox_root": ".SddIA/sandbox/",
        "revoked_entities": ".SddIA/cerbero/revoked_entities.json",
    }
    try:
        cfg = _load_cumulo(repo)
        block = cfg.get("radamanto") or {}
        if isinstance(block, dict):
            for key, value in block.items():
                if isinstance(value, str) and value.strip():
                    defaults[key] = _normalize_rel(value.strip())
    except (OSError, ValueError):
        pass
    thresh_rel = defaults["thresholds"]
    thresh_path = repo / thresh_rel if not Path(thresh_rel).is_absolute() else Path(thresh_rel)
    thresholds: dict[str, Any] = {
        "success_rate_min": 0.85,
        "batch_min_events": 10,
        "latency_ms_p95_threshold": 30000,
        "redemption_success_count": 3,
        "max_recovery_attempts": 3,
        "abrupt_drop_min_samples": 3,
    }
    if thresh_path.is_file():
        try:
            loaded = json.loads(thresh_path.read_text(encoding="utf-8"))
            if isinstance(loaded, dict):
                thresholds.update(loaded)
        except (OSError, json.JSONDecodeError):
            pass
    defaults["thresholds"] = thresholds
    return defaults


DEFAULT_TELEMETRY_SCHEMA = ["prompt_tokens", "completion_tokens"]

RADAMANTO_BATCH_SUBSCRIBER_KEY = "radamanto.radamanto-batch"
COMPLIANCE_SUBSCRIBER_KEY = "argos.telemetry-compliance-audit"


def _parse_ed_frontmatter(repo: Path, rel_path: str) -> dict[str, Any]:
    path = repo / rel_path
    if not path.is_file():
        return {}
    try:
        from execute_process_core import parse_frontmatter

        return parse_frontmatter(path)
    except (ImportError, OSError):
        return {}


def resolve_ed_telemetry_contract(
    repo: Path, capsule_id: str | None
) -> dict[str, Any]:
    if not isinstance(capsule_id, str) or not capsule_id.strip():
        return {"telemetry_provided": False, "telemetry_schema": None, "entity_kind": None}
    cid = capsule_id.strip()
    for kind, subdir in (("skill", "skills"), ("action", "actions"), ("tool", "tools")):
        rel = f"SddIA/{subdir}/{cid}.md"
        fm = _parse_ed_frontmatter(repo, rel)
        if not fm:
            continue
        provided = bool(fm.get("telemetry_provided", False))
        schema = fm.get("telemetry_schema")
        if provided and not schema:
            schema = list(DEFAULT_TELEMETRY_SCHEMA)
        elif isinstance(schema, list):
            schema = [str(x) for x in schema]
        else:
            schema = None
        return {
            "telemetry_provided": provided,
            "telemetry_schema": schema,
            "entity_kind": kind,
        }
    return {"telemetry_provided": False, "telemetry_schema": None, "entity_kind": None}


def receipt_satisfies_schema(receipt: dict[str, Any], schema: list[str]) -> bool:
    if not isinstance(receipt, dict):
        return False
    for key in schema:
        val = receipt.get(key)
        if not isinstance(val, (int, float)) or val < 0:
            return False
    return True


def stamp_fractal_delivery_state(
    repo: Path,
    event_path: Path,
    subscriber_key: str,
    status: str,
) -> None:
    if not event_path.is_file():
        return
    try:
        body = json.loads(event_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return
    ds = body.get("delivery_state")
    if not isinstance(ds, dict):
        ds = {}
        body["delivery_state"] = ds
    ds[subscriber_key] = status
    _write_json_atomic(event_path, body)


def delivery_stamp_terminal_ok(status: str) -> bool:
    return status == "success" or status == "skipped" or status.startswith("skipped")


def maybe_purge_fractal_telemetry_when_terminal(
    repo: Path,
    event_path: Path,
    registry: dict[str, Any],
    event_type: str,
) -> bool:
    required = required_subscriber_ids(registry, event_type)
    if not required or not event_path.is_file():
        return False
    try:
        body = json.loads(event_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return False
    ds = body.get("delivery_state")
    if not isinstance(ds, dict):
        return False
    for sid in required:
        st = ds.get(sid)
        if not isinstance(st, str) or not delivery_stamp_terminal_ok(st):
            return False
    return safe_remove_path(event_path)


def load_telemetry_compliance_config(repo: Path) -> dict[str, str]:
    defaults: dict[str, str] = {
        "emitted_registry": ".SddIA/telemetry-compliance/emitted.json",
    }
    try:
        cfg = _load_cumulo(repo)
        block = cfg.get("telemetry_compliance") or {}
        if isinstance(block, dict):
            for key, value in block.items():
                if isinstance(value, str) and value.strip():
                    defaults[key] = _normalize_rel(value.strip())
    except (OSError, ValueError):
        pass
    return defaults


def build_telemetry_compliance_breached_event(
    *,
    asset_id: str,
    capsule_id: str,
    process_name: str,
    breach_reason: str,
    expected_schema: list[str] | None = None,
) -> dict[str, Any]:
    import uuid

    payload: dict[str, Any] = {
        "asset_id": asset_id,
        "capsule_id": capsule_id,
        "breach_reason": breach_reason,
        "process_name": process_name,
    }
    if expected_schema:
        payload["expected_schema"] = expected_schema
    return {
        "event_id": str(uuid.uuid4()),
        "event_type": "Telemetry_Compliance_Breached",
        "event_family": "domain",
        "timestamp": _telemetry_timestamp(),
        "emitter_agent": "telemetry-compliance-audit",
        "payload": payload,
        "delivery_state": {},
    }
