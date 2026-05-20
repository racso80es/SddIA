# -*- coding: utf-8 -*-
"""REGISTRY de cápsulas físicas y orquestación de fases (laboratorio SddIA)."""

from __future__ import annotations

import json
import os
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from execute_process_core import (
    delegates_are_only_agents,
    load_process_def,
    parse_frontmatter,
    phase_invocations_index,
    validate_process_inputs,
)

from execute_process_forges import FORGE_BY_ENTITY_CLASS
from eda_bus_utils import find_existing_domain_event

try:
    import yaml
except ImportError:
    yaml = None  # type: ignore

SCRIPT = Path(__file__).resolve()
EXECUTE_PROCESS_CLI = SCRIPT.parent / "execute-process.py"
EXECUTE_ACTION_CLI = SCRIPT.parent / "execute-action.py"

CREATOR_BY_CLASS: dict[str, str] = {
    "skill": "skill-creator",
    "process": "process-creator",
    "agent": "agent-creator",
    "tool": "tool-creator",
    "action": "action-creator",
    "norm": "norm-creator",
    "codex": "codex-creator",
    "event": "event-creator",
}

DIR_BY_CLASS: dict[str, str] = {
    "skill": "SddIA/skills",
    "process": "SddIA/process",
    "agent": "SddIA/agents",
    "tool": "SddIA/tools",
    "action": "SddIA/actions",
    "norm": "SddIA/library/norms",
    "codex": "SddIA/library/codexes",
    "event": "SddIA/events",
}

PILOT_ENTITY_CLASSES = frozenset({
    "skill", "event", "process", "agent", "tool", "action", "norm", "codex",
})

# Cápsulas action:* con handler físico en execute-action.py
CAPSULE_ACTION_REGISTRY: dict[str, str] = {
    "action:emit-domain-mutation": "emit-domain-mutation",
    "action:emit-pr-merged-event": "emit-pr-merged-event",
    "action:emit-pr-presented-event": "emit-pr-presented-event",
}


def _load_cumulo(repo: Path) -> dict[str, Any]:
    return json.loads((repo / "SddIA" / "core" / "cumulo.paths.json").read_text(encoding="utf-8"))


def crypto(repo: Path, payload: dict[str, Any]) -> Any:
    crypto_script = repo / "scripts" / "skills" / "cryptography-manager.py"
    proc = subprocess.run(
        [sys.executable, str(crypto_script)],
        input=json.dumps(payload),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )
    out = json.loads(proc.stdout or "{}")
    if not out.get("success"):
        raise RuntimeError(out.get("error") or proc.stderr or "cryptography-manager failed")
    return out["data"]["result"]


def invoke_git_manager(repo: Path, operation_type: str, payload: dict[str, Any]) -> dict[str, Any]:
    git_script = repo / "scripts" / "skills" / "git-manager.py"
    if not git_script.is_file():
        raise FileNotFoundError(str(git_script))
    req = {
        "operation_type": operation_type,
        "repository_path": str(repo.resolve()),
        "operation_payload_json": payload,
    }
    proc = subprocess.run(
        [sys.executable, str(git_script)],
        input=json.dumps(req, ensure_ascii=False),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )
    stdout = (proc.stdout or "").strip()
    if not stdout:
        raise RuntimeError(proc.stderr or "git-manager sin salida")
    body = json.loads(stdout)
    if not body.get("success"):
        raise RuntimeError(body.get("error") or "git-manager failed")
    return body.get("data") or {}


def invoke_subprocess_process(repo: Path, process_name: str, process_inputs: dict[str, Any]) -> dict[str, Any]:
    proc = subprocess.run(
        [
            sys.executable,
            str(EXECUTE_PROCESS_CLI),
            "--process",
            process_name,
            "--inputs",
            json.dumps(process_inputs, ensure_ascii=False),
        ],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if not line:
        raise RuntimeError(proc.stderr or f"subproceso {process_name} sin salida")
    body = json.loads(line)
    if not body.get("success"):
        raise RuntimeError(body.get("error") or f"subproceso {process_name} falló")
    return body.get("data") or {}


def is_workspace_init_phase(phase: dict[str, Any], inputs: dict[str, Any]) -> bool:
    delegates = phase.get("delegates_to") or []
    if not isinstance(delegates, list):
        return False
    has_git = any(isinstance(d, str) and d == "skill:git-manager" for d in delegates)
    feature_name = inputs.get("feature_name")
    return has_git and isinstance(feature_name, str) and bool(feature_name.strip())


def run_workspace_init(
    repo: Path,
    inputs: dict[str, Any],
) -> dict[str, Any]:
    """Handler genérico: fase con git-manager + feature_name → rama + objectives.md."""
    feature_name = str(inputs["feature_name"]).strip()
    branch_name = inputs.get("branch_name") or f"feat/{feature_name}"
    base_branch = inputs.get("base_branch") or "main"
    persist_ref = inputs.get("persist_ref") or f"docs/features/{feature_name}"
    refined = inputs.get("refined_requirements") or inputs.get("description") or ""

    if not isinstance(branch_name, str) or not branch_name.strip():
        raise ValueError("branch_name inválido")
    if not isinstance(base_branch, str) or not base_branch.strip():
        raise ValueError("base_branch inválido")
    if not isinstance(persist_ref, str) or not persist_ref.strip():
        raise ValueError("persist_ref inválido")

    git_steps: list[dict[str, Any]] = [
        {"op": "fetch", "result": invoke_git_manager(repo, "fetch", {"remote": "origin", "prune": True})},
        {
            "op": "checkout_base",
            "result": invoke_git_manager(
                repo,
                "checkout",
                {"branch_name": base_branch.strip(), "create_if_not_exists": False},
            ),
        },
        {
            "op": "pull_base",
            "result": invoke_git_manager(
                repo,
                "pull",
                {"remote": "origin", "branch": base_branch.strip()},
            ),
        },
    ]
    try:
        git_steps.append(
            {
                "op": "checkout_feature",
                "result": invoke_git_manager(
                    repo,
                    "checkout",
                    {"branch_name": branch_name.strip(), "create_if_not_exists": True},
                ),
            }
        )
    except RuntimeError:
        git_steps.append(
            {
                "op": "checkout_feature_existing",
                "result": invoke_git_manager(
                    repo,
                    "checkout",
                    {"branch_name": branch_name.strip(), "create_if_not_exists": False},
                ),
            }
        )

    persist_dir = repo / persist_ref
    persist_dir.mkdir(parents=True, exist_ok=True)
    objectives_path = persist_dir / "objectives.md"
    created = datetime.now(timezone.utc).strftime("%Y-%m-%d")
    if not objectives_path.is_file():
        summary = refined.strip() if isinstance(refined, str) else f"Feature {feature_name}"
        process_label = inputs.get("process_label") or "feature"
        objectives_path.write_text(
            f"""---
feature_name: {feature_name}
created: "{created}"
process: {process_label}
branch_name: {branch_name.strip()}
persist_ref: {persist_ref.strip()}
pbi_ref: {inputs.get("pbi_ref", "PBI-005")}
---

# Objetivos — {feature_name}

## Misión

{summary}

## Alcance (manifiesto)

Inicialización de contexto vía intérprete dinámico `execute-process.py` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
""",
            encoding="utf-8",
        )

    return {
        "feature_name": feature_name,
        "branch_name": branch_name.strip(),
        "persist_ref": persist_ref.strip(),
        "objectives_path": str(objectives_path.relative_to(repo)).replace("\\", "/"),
        "git_steps": git_steps,
    }


def run_skill_forge(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    name = inputs.get("skill_name") or inputs.get("entity_name")
    if not isinstance(name, str) or not name:
        raise ValueError("skill_name requerido")
    skill_path = repo / "SddIA" / "skills" / f"{name}.md"
    if skill_path.is_file() and inputs.get("lifecycle_operation", "create") == "create":
        raise FileExistsError(f"Ya existe {skill_path}")

    context = inputs.get("skill_context", "ecosystem-evolution")
    version = inputs.get("skill_version", "1.0.0")
    contract_ver = inputs.get("skills_contract_version", "1.1.0")
    desc = inputs.get("skill_description", f"Skill {name}")
    in_schema = inputs.get("skill_inputs_schema", [])
    out_schema = inputs.get("skill_outputs_schema", [])

    skill_uuid = crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    canon = {
        "skill_context": context,
        "skill_inputs_schema": in_schema,
        "skill_name": name,
        "skill_outputs_schema": out_schema,
        "skill_version": version,
    }
    hex_sig = crypto(
        repo,
        {
            "operation": "GENERATE_SHA256",
            "target_type": "STRING",
            "target_payload": json.dumps(
                canon, sort_keys=True, separators=(",", ":"), ensure_ascii=False
            ),
        },
    )
    hash_sig = f"sha256:{hex_sig}"
    cap = name.replace("-", "_")[:32] or "skill-cap"

    body = f"""---
uuid: "{skill_uuid}"
name: "{name}"
version: "{version}"
contract: "skills-contract v{contract_ver}"
context: "{context}"
capabilities:
  - "{cap}"
hash_signature: "{hash_sig}"
inputs:
  - "inputs_placeholder": "definir segun skill_inputs_schema en forja completa"
outputs:
  - "success": "boolean"
---

# Skill: {name}

{desc}
"""
    skill_path.parent.mkdir(parents=True, exist_ok=True)
    skill_path.write_text(body, encoding="utf-8")

    index_path = repo / "SddIA" / "skills" / "index.md"
    row = (
        f"| `{name}.md` | `{skill_uuid}` | {name} | {version} | "
        f"skills-contract v{contract_ver} | {context} | `{cap}` |"
    )
    idx = index_path.read_text(encoding="utf-8")
    if name not in idx:
        idx = idx.replace("| `shell-executor.md` |", row + "\n| `shell-executor.md` |", 1)
        if name not in idx:
            idx = idx.rstrip() + "\n" + row + "\n"
        index_path.write_text(idx, encoding="utf-8")

    return {
        "artifact_skill_md": str(skill_path.relative_to(repo)).replace("\\", "/"),
        "artifact_skills_index": "SddIA/skills/index.md",
        "handoff_entity_uuid": skill_uuid,
        "handoff_hash_signature_new": hash_sig,
        "handoff_hash_signature_old": None,
        "handoff_version": version,
    }


def run_event_forge(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    name = inputs.get("event_name") or inputs.get("entity_name")
    if not isinstance(name, str) or not name:
        raise ValueError("event_name requerido")
    event_path = repo / "SddIA" / "events" / f"{name}.md"
    if event_path.is_file() and inputs.get("lifecycle_operation", "create") == "create":
        raise FileExistsError(f"Ya existe {event_path}")

    event_type = inputs.get("event_type")
    if not isinstance(event_type, str) or not event_type.strip():
        raise ValueError("event_type requerido")
    context = inputs.get("event_context", "ecosystem-evolution")
    version = inputs.get("event_version", "1.0.0")
    contract_ver = inputs.get("events_contract_version", "1.0.0")
    desc = inputs.get("event_description", f"Clase de Evento {event_type}")
    payload_required = inputs.get("payload_required", [])
    payload_optional = inputs.get("payload_optional", [])
    payload_forbidden = inputs.get("payload_forbidden", [])
    emitters = inputs.get("emitter_agents", [])

    event_uuid = crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    canon = {
        "event_name": name,
        "event_type": event_type,
        "event_version": version,
        "event_context": context,
        "payload_required": payload_required,
        "payload_optional": payload_optional,
        "payload_forbidden": payload_forbidden,
    }
    hex_sig = crypto(
        repo,
        {
            "operation": "GENERATE_SHA256",
            "target_type": "STRING",
            "target_payload": json.dumps(
                canon, sort_keys=True, separators=(",", ":"), ensure_ascii=False
            ),
        },
    )
    hash_sig = f"sha256:{hex_sig}"
    cap = name.replace("-", "_")[:32] or "event-cap"

    req_lines = "\n".join(f"- `{f}`" for f in payload_required) or "- *(ninguno)*"
    opt_lines = "\n".join(f"- `{f}`" for f in payload_optional) or "- *(ninguno)*"
    forb_lines = "\n".join(f"- `{f}`" for f in payload_forbidden) or "- *(ninguno)*"
    emitter_lines = "\n".join(f"- `{e}`" for e in emitters) or "- *(definir en forja completa)*"

    body = f"""---
uuid: "{event_uuid}"
name: "{name}"
version: "{version}"
contract: "events-contract v{contract_ver}"
event_type: "{event_type}"
context: "{context}"
capabilities:
  - "{cap}"
hash_signature: "{hash_sig}"
---

# Event: {event_type}

{desc}

## Payload ECST

### REQUIRED
{req_lines}

### OPTIONAL
{opt_lines}

### FORBIDDEN
{forb_lines}

## Emisores autorizados

{emitter_lines}

## Suscripciones

Ver `SddIA/core/event-subscriptions.json` → clave `{event_type}`.
"""
    event_path.parent.mkdir(parents=True, exist_ok=True)
    event_path.write_text(body, encoding="utf-8")

    index_path = repo / "SddIA" / "events" / "index.md"
    row = (
        f"| `{name}.md` | `{event_uuid}` | {name} | {event_type} | {version} | "
        f"events-contract v{contract_ver} | {context} | `{cap}` |"
    )
    idx = index_path.read_text(encoding="utf-8")
    marker = "| Archivo fuente | uuid | name | event_type |"
    if name not in idx:
        if marker in idx:
            idx = idx.replace(
                "| Archivo fuente | uuid | name | event_type | version | contract | context | Capabilities |\n"
                "|----------------|------|------|------------|---------|----------|---------|--------------|\n",
                "| Archivo fuente | uuid | name | event_type | version | contract | context | Capabilities |\n"
                "|----------------|------|------|------------|---------|----------|---------|--------------|\n"
                + row + "\n",
                1,
            )
        else:
            idx = idx.rstrip() + "\n" + row + "\n"
        index_path.write_text(idx, encoding="utf-8")

    return {
        "artifact_event_md": str(event_path.relative_to(repo)).replace("\\", "/"),
        "artifact_events_index": "SddIA/events/index.md",
        "handoff_entity_uuid": event_uuid,
        "handoff_hash_signature_new": hash_sig,
        "handoff_hash_signature_old": None,
        "handoff_version": version,
    }


def materialize_forge_by_inputs(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    """Forja física según entity_class o forma del contrato de inputs."""
    entity_class = inputs.get("entity_class")
    if isinstance(entity_class, str) and entity_class in FORGE_BY_ENTITY_CLASS:
        return FORGE_BY_ENTITY_CLASS[entity_class](repo, inputs)
    if inputs.get("skill_name") is not None or (
        inputs.get("skill_inputs_schema") is not None and inputs.get("skill_context") is not None
    ):
        return run_skill_forge(repo, inputs)
    if inputs.get("event_type") is not None or inputs.get("event_name") is not None:
        return run_event_forge(repo, {**inputs, "lifecycle_operation": inputs.get("lifecycle_operation", "create")})
    if inputs.get("tool_name") is not None:
        return FORGE_BY_ENTITY_CLASS["tool"](repo, inputs)
    if inputs.get("action_name") is not None:
        return FORGE_BY_ENTITY_CLASS["action"](repo, inputs)
    if inputs.get("process_name") is not None:
        return FORGE_BY_ENTITY_CLASS["process"](repo, inputs)
    if inputs.get("agent_name") is not None:
        return FORGE_BY_ENTITY_CLASS["agent"](repo, inputs)
    if inputs.get("tactical_norm_name") is not None:
        return FORGE_BY_ENTITY_CLASS["norm"](repo, inputs)
    if inputs.get("domain_codex_slug") is not None:
        return FORGE_BY_ENTITY_CLASS["codex"](repo, inputs)
    raise NotImplementedError(
        "Forja física no disponible para esta forma de inputs"
    )


def _base_creator_inputs(
    entity_class: str, entity_name: str, lifecycle: str, seed: dict[str, Any]
) -> dict[str, Any]:
    scope = seed.get("scope", "core")
    origin = "local" if scope == "local" else "core"
    return {
        "entity_class": entity_class,
        "lifecycle_operation": lifecycle,
        "origin_topology": seed.get("origin_topology", origin),
    }


def creator_inputs_from_entity(
    entity_class: str, entity_name: str, lifecycle: str, seed: dict[str, Any]
) -> dict[str, Any]:
    base = _base_creator_inputs(entity_class, entity_name, lifecycle, seed)
    if entity_class == "skill":
        return {
            **base,
            "skill_name": seed.get("skill_name", entity_name),
            "skill_context": seed.get("skill_context", "ecosystem-evolution"),
            "skill_description": seed.get("skill_description", ""),
            "skill_inputs_schema": seed.get("skill_inputs_schema", []),
            "skill_outputs_schema": seed.get("skill_outputs_schema", []),
            "skill_version": seed.get("skill_version", "1.0.0"),
            "skills_contract_version": seed.get("skills_contract_version", "1.1.0"),
        }
    if entity_class == "event":
        return {
            **base,
            "event_name": seed.get("event_name", entity_name),
            "event_type": seed.get("event_type", ""),
            "event_context": seed.get("event_context", "ecosystem-evolution"),
            "event_description": seed.get("event_description", ""),
            "payload_required": seed.get("payload_required", []),
            "payload_optional": seed.get("payload_optional", []),
            "payload_forbidden": seed.get("payload_forbidden", []),
            "emitter_agents": seed.get("emitter_agents", []),
            "event_version": seed.get("event_version", "1.0.0"),
            "events_contract_version": seed.get("events_contract_version", "1.0.0"),
        }
    if entity_class == "tool":
        tname = seed.get("tool_name", entity_name)
        return {
            **base,
            "tool_name": tname,
            "tool_id": seed.get("tool_id", tname),
            "scope": seed.get("scope", "core"),
            "domain_origin": seed.get("domain_origin", "SddIA"),
            "tool_context": seed.get("tool_context", "ecosystem-evolution"),
            "required_secrets": seed.get("required_secrets", []),
            "dependencies": seed.get("dependencies", []),
            "tool_outputs": seed.get("tool_outputs", []),
            "execution_logic": seed.get("execution_logic", f"Tool {entity_name}"),
            "tools_contract_version": seed.get("tools_contract_version", "1.2.0"),
        }
    if entity_class == "action":
        return {
            **base,
            "action_name": seed.get("action_name", entity_name),
            "action_context": seed.get("action_context", "ecosystem-evolution"),
            "action_inputs": seed.get("action_inputs", []),
            "action_outputs": seed.get("action_outputs", []),
            "orchestration_logic": seed.get("orchestration_logic", f"Acción {entity_name}"),
            "actions_contract_version": seed.get("actions_contract_version", "1.2.0"),
        }
    if entity_class == "process":
        return {
            **base,
            "process_name": seed.get("process_name", entity_name),
            "process_description": seed.get("process_description", f"Proceso {entity_name}"),
            "process_context": seed.get("process_context", "ecosystem-evolution"),
            "process_phases": seed.get("process_phases", [{"name": "Fase inicial", "intent": "stub"}]),
            "process_contract_version": seed.get("process_contract_version", "1.3.0"),
            "process_aliases": seed.get("process_aliases", []),
        }
    if entity_class == "agent":
        return {
            **base,
            "agent_name": seed.get("agent_name", entity_name),
            "allowed_policies": seed.get("allowed_policies", ["ecosystem-evolution"]),
            "agent_inputs": seed.get("agent_inputs", []),
            "agent_outputs": seed.get("agent_outputs", []),
            "agent_purpose": seed.get("agent_purpose", f"Agente {entity_name}"),
            "agents_contract_version": seed.get("agents_contract_version", "1.0.0"),
        }
    if entity_class == "norm":
        return {
            **base,
            "tactical_norm_name": seed.get("tactical_norm_name", entity_name),
            "tactical_norm_version": seed.get("tactical_norm_version", "1.0.0"),
            "tactical_norm_friction": seed.get("tactical_norm_friction", f"Norma {entity_name}"),
            "tactical_norm_author": seed.get("tactical_norm_author", "laboratorio"),
            "tactical_norm_dependencies": seed.get("tactical_norm_dependencies", []),
            "norms_contract_version": seed.get("norms_contract_version", "1.0.0"),
            "norm_scope": seed.get("norm_scope", "agnostic"),
            "norm_category": seed.get("norm_category", "workflow"),
        }
    if entity_class == "codex":
        return {
            **base,
            "domain_codex_slug": seed.get("domain_codex_slug", entity_name),
            "domain_codex_name": seed.get("domain_codex_name", entity_name),
            "domain_codex_version": seed.get("domain_codex_version", "1.0.0"),
            "domain_codex_author": seed.get("domain_codex_author", "laboratorio"),
            "target_environment": seed.get("target_environment", ["dev"]),
            "tactical_norm_inventory": seed.get("tactical_norm_inventory", []),
            "codex_contract_version": seed.get("codex_contract_version", "1.0.0"),
            "domain_codex_certification_grade": seed.get("domain_codex_certification_grade", "Pendiente"),
        }
    raise NotImplementedError(f"mapeo semantic_seed no definido para entity_class={entity_class}")


def write_pending_event(repo: Path, event: dict[str, Any]) -> dict[str, str]:
    cumulo = _load_cumulo(repo)
    pending_rel = cumulo.get("eda_bus", {}).get("pending", "docs/events/pending")
    pending = repo / pending_rel
    pending.mkdir(parents=True, exist_ok=True)
    event_id = event["event_id"]
    target = pending / f"{event_id}.json"
    target.write_text(json.dumps(event, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return {
        "event_id": event_id,
        "target_path": str(target.relative_to(repo)).replace("\\", "/"),
    }


def emit_domain_mutation(repo: Path, payload: dict[str, Any]) -> dict[str, Any]:
    op = payload["lifecycle_operation"]
    event_type = {
        "create": "Domain_Entity_Created",
        "update": "Domain_Entity_Updated",
        "delete": "Domain_Entity_Deleted",
    }[op]

    entity_uuid = payload.get("entity_uuid")
    if entity_uuid and op != "delete":
        existing = find_existing_domain_event(repo, entity_uuid, op, event_type)
        if existing and existing.get("event_id"):
            return {"idempotent": True, **existing}

    origin_topology = payload.get("origin_topology", "core")
    if origin_topology not in ("core", "local"):
        origin_topology = "core"

    event_id = crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    event = {
        "event_id": event_id,
        "event_type": event_type,
        "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "emitter_agent": payload.get("emitter_agent", "entity-manager"),
        "payload": {
            "entity_class": payload["entity_class"],
            "lifecycle_operation": op,
            "entity_uuid": entity_uuid,
            "entity_name": payload["entity_name"],
            "version": payload.get("version"),
            "hash_signature_new": payload.get("hash_signature_new"),
            "hash_signature_old": payload.get("hash_signature_old"),
            "origin_topology": origin_topology,
            "changes_summary": payload.get(
                "changes_summary",
                f"{op} {payload['entity_class']} {payload['entity_name']}",
            ),
        },
        "delivery_state": {},
    }
    return write_pending_event(repo, event)


def run_phase_invocations(
    repo: Path,
    inv_block: dict[str, Any],
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> list[dict[str, Any]]:
    invocations = inv_block.get("invocations") or []
    log: list[dict[str, Any]] = []
    for inv in invocations:
        if not isinstance(inv, dict):
            continue
        capsule = inv.get("capsule", "")
        if capsule != "action:crypto-broker":
            log.append({"capsule": capsule, "status": "skipped", "note": "cápsula no ejecutada en lab v1"})
            continue
        stdin = inv.get("stdin_json")
        if not isinstance(stdin, dict):
            log.append({"capsule": capsule, "status": "skipped", "note": "stdin_json ausente"})
            continue
        result = crypto(repo, stdin)
        binds = inv.get("bind") or {}
        if isinstance(binds, dict):
            for path, var in binds.items():
                if path == "data.result":
                    state[var] = result
        log.append({"capsule": capsule, "status": "executed", "bind": binds})
    return log


def capsule_action_execute_process(
    repo: Path,
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any]:
    entity_class = inputs.get("entity_class")
    entity_name = inputs.get("entity_name")
    lifecycle = inputs.get("lifecycle_operation")
    if lifecycle == "delete":
        return {"skipped": True, "reason": "delete omite delegación al creator"}
    if not isinstance(entity_class, str) or entity_class not in CREATOR_BY_CLASS:
        raise ValueError(f"entity_class no resuelta: {entity_class}")
    if entity_class not in PILOT_ENTITY_CLASSES:
        raise NotImplementedError(
            f"entity_class '{entity_class}' fuera del piloto v1 ({', '.join(sorted(PILOT_ENTITY_CLASSES))})"
        )
    creator = CREATOR_BY_CLASS[entity_class]
    seed = dict(inputs.get("semantic_seed") or {})
    child_inputs = creator_inputs_from_entity(
        entity_class, str(entity_name), str(lifecycle), seed
    )
    forge: dict[str, Any] = {}
    try:
        forge = materialize_forge_by_inputs(repo, child_inputs)
        state["handoff"].update(forge)
    except NotImplementedError:
        pass
    if forge.get("handoff_entity_uuid"):
        return {"child_process": creator, "handoff": state["handoff"], "forge_only": True}
    data = invoke_subprocess_process(repo, creator, child_inputs)
    if data.get("handoff"):
        state["handoff"].update(data["handoff"])
    else:
        state["handoff"].update({k: v for k, v in data.items() if k.startswith("handoff_")})
    return {"child_process": creator, "handoff": state["handoff"]}


def capsule_filesystem_delete(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    entity_class = inputs.get("entity_class")
    entity_name = inputs.get("entity_name")
    if inputs.get("lifecycle_operation") != "delete":
        return {"skipped": True}
    rel_dir = DIR_BY_CLASS.get(str(entity_class))
    if not rel_dir:
        raise ValueError(f"entity_class desconocida: {entity_class}")
    artifact = repo / rel_dir / f"{entity_name}.md"
    if not artifact.is_file():
        raise FileNotFoundError(str(artifact))
    fm = parse_frontmatter(artifact)
    handoff = {
        "handoff_entity_uuid": fm.get("uuid"),
        "handoff_hash_signature_new": None,
        "handoff_hash_signature_old": fm.get("hash_signature"),
        "handoff_version": fm.get("version"),
    }
    artifact.unlink()
    state["handoff"].update(handoff)
    return {"deleted": str(artifact), **handoff}


def invoke_capsule_action(
    repo: Path, action_name: str, action_inputs: dict[str, Any]
) -> dict[str, Any]:
    body = shim_execute_action(repo, action_name, action_inputs)
    if not body.get("success"):
        raise RuntimeError(body.get("error") or f"acción {action_name} falló")
    return body.get("data") or {}


def capsule_emit_domain_mutation(repo: Path, inputs: dict[str, Any], state: dict[str, Any]) -> dict[str, Any]:
    handoff = state.get("handoff") or {}
    seed = dict(inputs.get("semantic_seed") or {})
    scope = seed.get("scope", "core")
    origin_topology = handoff.get("origin_topology") or seed.get("origin_topology")
    if not origin_topology:
        origin_topology = "local" if scope == "local" else "core"
    action_inputs = {
        "entity_class": inputs.get("entity_class"),
        "entity_name": inputs.get("entity_name"),
        "lifecycle_operation": inputs.get("lifecycle_operation"),
        "entity_uuid": handoff.get("handoff_entity_uuid"),
        "version": handoff.get("handoff_version"),
        "hash_signature_new": handoff.get("handoff_hash_signature_new"),
        "hash_signature_old": handoff.get("handoff_hash_signature_old"),
        "origin_topology": origin_topology,
        "emitter_agent": inputs.get("emitter_agent", "entity-manager"),
        "changes_summary": inputs.get(
            "changes_summary",
            f"{inputs.get('lifecycle_operation')} {inputs.get('entity_class')} {inputs.get('entity_name')}",
        ),
    }
    seal = invoke_capsule_action(repo, "emit-domain-mutation", action_inputs)
    state["handoff"].update(seal)
    return seal


def try_execute_registered_action_capsules(
    repo: Path,
    delegates: list[Any],
    inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any] | None:
    if not isinstance(delegates, list):
        return None
    for capsule in delegates:
        if not isinstance(capsule, str):
            continue
        action_name = CAPSULE_ACTION_REGISTRY.get(capsule)
        if not action_name:
            continue
        if action_name == "emit-domain-mutation" and inputs.get("entity_class"):
            return capsule_emit_domain_mutation(repo, inputs, state)
        if action_name == "emit-pr-presented-event":
            branch = inputs.get("branch") or inputs.get("branch_name")
            if isinstance(branch, str) and branch.strip():
                data = invoke_capsule_action(
                    repo,
                    action_name,
                    {
                        "branch": branch.strip(),
                        "status": inputs.get("status", "presented"),
                        "emitter_agent": inputs.get("emitter_agent", action_name),
                    },
                )
                return data
        if action_name == "emit-pr-merged-event":
            if inputs.get("merge_commit_hash") or inputs.get("hash_signature"):
                return invoke_capsule_action(repo, action_name, dict(inputs))
    return None


def execute_phase(
    repo: Path,
    phase: dict[str, Any],
    process_def: dict[str, Any],
    inputs: dict[str, Any],
    state: dict[str, Any],
    pi_index: dict[str, dict[str, Any]],
) -> dict[str, Any]:
    phase_name = phase.get("name")
    delegates = phase.get("delegates_to") or []
    entry: dict[str, Any] = {
        "phase_name": phase_name,
        "delegates_to": delegates,
    }

    if is_workspace_init_phase(phase, inputs):
        result = run_workspace_init(repo, inputs)
        entry["status"] = "executed"
        entry["handler"] = "workspace-init"
        entry.update({k: result[k] for k in ("git_steps", "objectives_path", "branch_name") if k in result})
        state["workspace"] = result
        return entry

    pi = pi_index.get(str(phase_name))
    if pi and pi.get("invocations"):
        inv_log = run_phase_invocations(repo, pi, inputs, state)
        entry["invocations"] = inv_log
        if any(i.get("status") == "executed" for i in inv_log):
            try:
                forge = materialize_forge_by_inputs(repo, inputs)
                state["handoff"].update(forge)
                entry["status"] = "executed"
                entry["forge"] = True
            except NotImplementedError:
                entry["status"] = "simulated"
                entry["note"] = "invocations parciales; forja no aplicable"
        else:
            entry["status"] = "simulated"
        return entry

    if isinstance(delegates, list):
        if "action:execute-process" in delegates and inputs.get("entity_class"):
            if inputs.get("lifecycle_operation") == "delete":
                entry["status"] = "skipped"
                entry["note"] = "fase omitida en delete"
                return entry
            child = capsule_action_execute_process(repo, inputs, state)
            entry["status"] = "executed"
            entry["child"] = child.get("child_process")
            return entry

        if "skill:filesystem-manager" in delegates and inputs.get("lifecycle_operation") == "delete":
            capsule_filesystem_delete(repo, inputs, state)
            entry["status"] = "executed"
            entry["handler"] = "filesystem-delete"
            return entry

        action_result = try_execute_registered_action_capsules(repo, delegates, inputs, state)
        if action_result is not None:
            entry["status"] = "executed"
            entry["action_capsule"] = action_result
            if isinstance(action_result, dict):
                entry.update({k: action_result[k] for k in ("event_id", "target_path", "event_type") if k in action_result})
            return entry

        if delegates_are_only_agents(delegates):
            entry["status"] = "simulated"
            entry["note"] = "agentes IDE; sin handler físico en laboratorio"
            return entry

        if any(isinstance(d, str) and d.startswith(("skill:", "tool:", "action:")) for d in delegates):
            entry["status"] = "simulated"
            entry["note"] = "cápsulas sin handler físico registrado"
            return entry

    entry["status"] = "simulated"
    return entry


def run_process(repo: Path, process_name: str, process_inputs: dict[str, Any]) -> dict[str, Any]:
    canonical, process_def, phases = load_process_def(repo, process_name)
    validate_process_inputs(process_def, process_inputs, canonical)

    state: dict[str, Any] = {"handoff": {}, "inputs": process_inputs}
    pi_index = phase_invocations_index(process_def)
    phase_reports: list[dict[str, Any]] = []

    for phase in phases:
        if not isinstance(phase, dict):
            continue
        phase_reports.append(
            execute_phase(repo, phase, process_def, process_inputs, state, pi_index)
        )

    data: dict[str, Any] = {"process_name": canonical, "handoff": state.get("handoff")}
    if state.get("workspace"):
        data.update(state["workspace"])

    return {
        "success": True,
        "status_code": 0,
        "data": data,
        "execution_report": {"process_name": canonical, "phases": phase_reports},
    }


def shim_execute_action(repo: Path, action_name: str, action_inputs: dict[str, Any]) -> dict[str, Any]:
    env = os.environ.copy()
    env["PYTHONIOENCODING"] = "utf-8"
    proc = subprocess.run(
        [
            sys.executable,
            str(EXECUTE_ACTION_CLI),
            "--action",
            action_name,
            "--inputs",
            json.dumps(action_inputs, ensure_ascii=False),
        ],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
        env=env,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if not line:
        raise RuntimeError(proc.stderr or "execute-action sin salida")
    return json.loads(line)
