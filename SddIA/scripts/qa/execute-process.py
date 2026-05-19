#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""CLI de laboratorio: ejecuta procesos SddIA con handlers fisicos minimos.

Contrato stdin (execute-process):
  {"process_name": "entity-manager", "process_inputs": { ... }}

Atajo: payload plano de entity-manager sin process_name -> se envuelve automaticamente.

Uso:
  echo '{"process_name":"entity-manager","process_inputs":{...}}' | python SddIA/scripts/qa/execute-process.py
  python SddIA/scripts/qa/execute-process.py --input-file payload.json
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

try:
    import yaml
except ImportError:
    yaml = None  # type: ignore

SCRIPT = Path(__file__).resolve()
REPO = SCRIPT.parents[3]

CREATOR_BY_CLASS: dict[str, str] = {
    "skill": "skill-creator",
    "process": "process-creator",
    "agent": "agent-creator",
    "tool": "tool-creator",
    "action": "action-creator",
    "norm": "norm-creator",
    "codex": "codex-creator",
}

DIR_BY_CLASS: dict[str, str] = {
    "skill": "SddIA/skills",
    "process": "SddIA/process",
    "agent": "SddIA/agents",
    "tool": "SddIA/tools",
    "action": "SddIA/actions",
    "norm": "SddIA/library/norms",
    "codex": "SddIA/library/codexes",
}

PILOT_ENTITY_CLASSES = frozenset({"skill"})


def _repo_root() -> Path:
    for parent in SCRIPT.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontro raiz del workspace (SddIA/core/cumulo.paths.json)")


def _emit(envelope: dict[str, Any], code: int | None = None) -> None:
    if code is None:
        code = 0 if envelope.get("success") else 1
    envelope.setdefault("exitCode", code)
    sys.stdout.write(json.dumps(envelope, ensure_ascii=False) + "\n")
    sys.exit(code)


def _load_cumulo(repo: Path) -> dict[str, Any]:
    return json.loads((repo / "SddIA" / "core" / "cumulo.paths.json").read_text(encoding="utf-8"))


def _crypto(repo: Path, payload: dict[str, Any]) -> Any:
    crypto = repo / "scripts" / "skills" / "cryptography-manager.py"
    proc = subprocess.run(
        [sys.executable, str(crypto)],
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


def _parse_frontmatter(md_path: Path) -> dict[str, Any]:
    text = md_path.read_text(encoding="utf-8")
    parts = text.split("---", 2)
    if len(parts) < 3 or yaml is None:
        return {}
    return yaml.safe_load(parts[1]) or {}


def _resolve_process_path(repo: Path, process_name: str) -> Path:
    process_dir = repo / "SddIA" / "process"
    canonical = process_name
    direct = process_dir / f"{canonical}.md"
    if direct.is_file():
        return direct
    if yaml is None:
        raise RuntimeError("PyYAML requerido para resolver aliases de proceso")
    for md in process_dir.glob("*.md"):
        if md.stem in ("index", "process-contract"):
            continue
        try:
            fm = yaml.safe_load(md.read_text(encoding="utf-8").split("---", 2)[1])
        except (IndexError, yaml.YAMLError):
            continue
        if not isinstance(fm, dict):
            continue
        if fm.get("name") == process_name:
            return md
        aliases = fm.get("aliases") or []
        if isinstance(aliases, list) and process_name in aliases:
            return md
    raise FileNotFoundError(f"Proceso no encontrado: {process_name}")


def _load_process_def(repo: Path, process_name: str) -> tuple[str, dict[str, Any], list[dict[str, Any]]]:
    path = _resolve_process_path(repo, process_name)
    if yaml is None:
        raise RuntimeError("PyYAML requerido")
    fm = yaml.safe_load(path.read_text(encoding="utf-8").split("---", 2)[1])
    phases = fm.get("phases") or []
    if not isinstance(phases, list):
        phases = []
    return fm.get("name") or path.stem, fm, phases


def _simulate_phase_log(phases: list[dict[str, Any]]) -> list[dict[str, Any]]:
    report: list[dict[str, Any]] = []
    for ph in phases:
        if not isinstance(ph, dict):
            continue
        report.append(
            {
                "phase_name": ph.get("name"),
                "status": "simulated",
                "delegates_to": ph.get("delegates_to") or [],
            }
        )
    return report


def _run_skill_creator(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
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

    skill_uuid = _crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    canon = {
        "skill_context": context,
        "skill_inputs_schema": in_schema,
        "skill_name": name,
        "skill_outputs_schema": out_schema,
        "skill_version": version,
    }
    hex_sig = _crypto(
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


def _write_pending_event(repo: Path, event: dict[str, Any]) -> dict[str, str]:
    cumulo = _load_cumulo(repo)
    pending_rel = cumulo.get("eda_bus", {}).get("pending", ".docs/events/pending")
    pending = repo / pending_rel
    pending.mkdir(parents=True, exist_ok=True)
    event_id = event["event_id"]
    target = pending / f"{event_id}.json"
    target.write_text(json.dumps(event, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return {
        "event_id": event_id,
        "target_path": str(target.relative_to(repo)).replace("\\", "/"),
    }


def _emit_pr_merged(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    merge_hash = inputs.get("merge_commit_hash") or inputs.get("hash_signature")
    if not isinstance(merge_hash, str) or not merge_hash.strip():
        raise ValueError("merge_commit_hash o hash_signature es obligatorio")

    source_branch = inputs.get("source_branch")
    if not isinstance(source_branch, str) or not source_branch.strip():
        pr_url = inputs.get("pr_url", "")
        if isinstance(pr_url, str) and "feature/" in pr_url:
            source_branch = pr_url.rsplit("/", 1)[-1]
        else:
            source_branch = "feature/eda-bus-v1"

    correlation_id = inputs.get("correlation_id")
    if not isinstance(correlation_id, str) or not correlation_id.strip():
        correlation_id = _crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})

    event_id = _crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    payload: dict[str, Any] = {
        "source_branch": source_branch.strip(),
        "target_branch": "main",
        "merge_commit_hash": merge_hash.strip(),
        "author": inputs.get("author", "integration-operator"),
        "security_clearance": {
            "auditor": "Argos",
            "audit_event_reference": "TODO: pending_argos_eda_emission",
            "policy_applied": "pr-acceptance-protocol",
        },
    }
    if inputs.get("pr_url"):
        payload["pr_url"] = inputs["pr_url"]
    if inputs.get("repository_name"):
        payload["repository_name"] = inputs["repository_name"]
    if inputs.get("hash_signature"):
        payload["hash_signature"] = inputs["hash_signature"]

    event = {
        "event_id": event_id,
        "event_type": "PullRequest_Merged",
        "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "emitter_agent": inputs.get("emitter_agent", "emit-pr-merged-event"),
        "correlation_id": correlation_id,
        "payload": payload,
        "delivery_state": {},
    }
    seal = _write_pending_event(repo, event)
    return {
        "success": True,
        "event_id": seal["event_id"],
        "target_path": seal["target_path"],
        "event_type": "PullRequest_Merged",
        "merge_commit_hash": merge_hash.strip(),
    }


def _emit_pr_presented(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    branch = inputs.get("branch")
    status = inputs.get("status", "presented")
    if not isinstance(branch, str) or not branch.strip():
        raise ValueError("branch es obligatorio (string)")

    event_id = _crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    event = {
        "event_id": event_id,
        "event_type": "PullRequest_Presented",
        "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "emitter_agent": inputs.get("emitter_agent", "emit-pr-presented-event"),
        "payload": {
            "branch": branch.strip(),
            "status": status,
        },
        "delivery_state": {},
    }
    seal = _write_pending_event(repo, event)
    return {
        "success": True,
        "event_id": seal["event_id"],
        "target_path": seal["target_path"],
        "event_type": "PullRequest_Presented",
    }


def _emit_domain_mutation(repo: Path, payload: dict[str, Any]) -> dict[str, str]:
    op = payload["lifecycle_operation"]
    event_type = {
        "create": "Domain_Entity_Created",
        "update": "Domain_Entity_Updated",
        "delete": "Domain_Entity_Deleted",
    }[op]

    event_id = _crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    event = {
        "event_id": event_id,
        "event_type": event_type,
        "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "emitter_agent": payload.get("emitter_agent", "entity-manager"),
        "payload": {
            "entity_class": payload["entity_class"],
            "lifecycle_operation": op,
            "entity_uuid": payload["entity_uuid"],
            "entity_name": payload["entity_name"],
            "version": payload.get("version"),
            "hash_signature_new": payload.get("hash_signature_new"),
            "hash_signature_old": payload.get("hash_signature_old"),
            "changes_summary": payload.get(
                "changes_summary",
                f"{op} {payload['entity_class']} {payload['entity_name']}",
            ),
        },
        "delivery_state": {},
    }
    return _write_pending_event(repo, event)


def _run_entity_manager(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    entity_class = inputs.get("entity_class")
    entity_name = inputs.get("entity_name")
    lifecycle = inputs.get("lifecycle_operation")
    if not all(isinstance(x, str) for x in (entity_class, entity_name, lifecycle)):
        raise ValueError("entity_class, entity_name y lifecycle_operation son obligatorios")

    report: list[dict[str, Any]] = []
    handoff: dict[str, Any] = {}

    if lifecycle in ("create", "update"):
        if entity_class not in PILOT_ENTITY_CLASSES:
            raise NotImplementedError(
                f"entity_class '{entity_class}' fuera del piloto v1 ({', '.join(sorted(PILOT_ENTITY_CLASSES))})"
            )
        creator = CREATOR_BY_CLASS[entity_class]
        seed = dict(inputs.get("semantic_seed") or {})
        creator_inputs = {
            "skill_name": seed.get("skill_name", entity_name),
            "skill_context": seed.get("skill_context", "ecosystem-evolution"),
            "skill_description": seed.get("skill_description", ""),
            "skill_inputs_schema": seed.get("skill_inputs_schema", []),
            "skill_outputs_schema": seed.get("skill_outputs_schema", []),
            "skill_version": seed.get("skill_version", "1.0.0"),
            "skills_contract_version": seed.get("skills_contract_version", "1.1.0"),
            "lifecycle_operation": lifecycle,
        }
        report.append({"phase_name": "Delegacion al creator", "status": "executed", "child": creator})
        handoff = _run_skill_creator(repo, creator_inputs)

    elif lifecycle == "delete":
        rel_dir = DIR_BY_CLASS.get(entity_class)
        if not rel_dir:
            raise ValueError(f"entity_class desconocida: {entity_class}")
        artifact = repo / rel_dir / f"{entity_name}.md"
        if not artifact.is_file():
            raise FileNotFoundError(str(artifact))
        fm = _parse_frontmatter(artifact)
        handoff = {
            "handoff_entity_uuid": fm.get("uuid"),
            "handoff_hash_signature_new": None,
            "handoff_hash_signature_old": fm.get("hash_signature"),
            "handoff_version": fm.get("version"),
        }
        artifact.unlink()
        report.append({"phase_name": "Delete fisico", "status": "executed", "deleted": str(artifact)})
    else:
        raise ValueError(f"lifecycle_operation no soportada: {lifecycle}")

    seal = _emit_domain_mutation(
        repo,
        {
            "entity_class": entity_class,
            "entity_name": entity_name,
            "lifecycle_operation": lifecycle,
            "entity_uuid": handoff.get("handoff_entity_uuid"),
            "version": handoff.get("handoff_version"),
            "hash_signature_new": handoff.get("handoff_hash_signature_new"),
            "hash_signature_old": handoff.get("handoff_hash_signature_old"),
            "emitter_agent": "entity-manager",
        },
    )
    report.append({"phase_name": "Sello universal", "status": "executed", **seal})

    return {**handoff, **seal, "execution_report": {"process_name": "entity-manager", "phases": report}}


def run_action(repo: Path, action_name: str, action_inputs: dict[str, Any]) -> dict[str, Any]:
    if action_name == "emit-pr-presented-event":
        data = _emit_pr_presented(repo, action_inputs)
        return {"success": True, "status_code": 0, "data": data}
    if action_name == "emit-pr-merged-event":
        data = _emit_pr_merged(repo, action_inputs)
        return {"success": True, "status_code": 0, "data": data}
    raise NotImplementedError(f"accion sin handler fisico: {action_name}")


def run_process(repo: Path, process_name: str, process_inputs: dict[str, Any]) -> dict[str, Any]:
    canonical, fm, phases = _load_process_def(repo, process_name)

    if canonical == "entity-manager":
        data = _run_entity_manager(repo, process_inputs)
        return {"success": True, "status_code": 0, "data": data, "execution_report": data.get("execution_report")}

    if canonical == "skill-creator":
        data = _run_skill_creator(repo, process_inputs)
        return {
            "success": True,
            "status_code": 0,
            "data": data,
            "execution_report": {"process_name": canonical, "phases": _simulate_phase_log(phases)},
        }

    return {
        "success": True,
        "status_code": 0,
        "data": {"process_name": canonical, "note": "fases simuladas (sin handler fisico)"},
        "execution_report": {"process_name": canonical, "phases": _simulate_phase_log(phases)},
    }


def _normalize_request(raw: dict[str, Any]) -> tuple[str, dict[str, Any]]:
    if "process_name" in raw:
        name = raw["process_name"]
        inputs = raw.get("process_inputs") or raw.get("inputs") or {}
        if not isinstance(name, str):
            raise ValueError("process_name debe ser string")
        if not isinstance(inputs, dict):
            raise ValueError("process_inputs debe ser objeto")
        return name, inputs
    if "entity_class" in raw and "entity_name" in raw:
        return "entity-manager", raw
    raise ValueError("stdin debe incluir process_name o payload entity-manager")


def main() -> None:
    parser = argparse.ArgumentParser(description="execute-process (laboratorio SddIA)")
    parser.add_argument("--input-file", help="Ruta a JSON de entrada")
    parser.add_argument("--action", help="Nombre de accion de dominio (cápsula fisica)")
    parser.add_argument("--inputs", help="JSON de inputs de la accion")
    args = parser.parse_args()

    try:
        if args.action:
            if not args.inputs:
                raise ValueError("--inputs requerido con --action")
            action_inputs = json.loads(args.inputs)
            if not isinstance(action_inputs, dict):
                raise ValueError("--inputs debe ser objeto JSON")
            raw: dict[str, Any] = {"__action__": args.action, "__action_inputs__": action_inputs}
        elif args.input_file:
            raw = json.loads(Path(args.input_file).read_text(encoding="utf-8-sig"))
        else:
            stdin = sys.stdin.read()
            raw = json.loads(stdin) if stdin.strip() else {}
    except json.JSONDecodeError as e:
        _emit({"success": False, "error": f"JSON invalido: {e}"}, 1)

    try:
        repo = _repo_root()
        if raw.get("__action__"):
            result = run_action(repo, str(raw["__action__"]), raw["__action_inputs__"])
        else:
            process_name, process_inputs = _normalize_request(raw)
            result = run_process(repo, process_name, process_inputs)
        _emit(result, result.get("status_code", 0))
    except Exception as e:
        _emit({"success": False, "status_code": 1, "error": str(e)}, 1)


if __name__ == "__main__":
    main()
