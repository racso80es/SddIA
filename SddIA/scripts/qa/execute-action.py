#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""CLI de laboratorio: intérprete universal de Acciones de Dominio SddIA.

Carga el contrato SddIA/actions/{action}.md y delega en handlers físicos
y tools asignadas (p. ej. markdown-table-editor para Cúmulo).

Uso:
  python SddIA/scripts/qa/execute-action.py --action sync-entity-index --inputs '{"entity_class":"skill",...}'
  python SddIA/scripts/qa/execute-action.py --action sync-entity-index --input-file payload.json
  echo '{"entity_class":"skill",...}' | python SddIA/scripts/qa/execute-action.py --action sync-entity-index
"""

from __future__ import annotations

import argparse
import json
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
_QA_DIR = SCRIPT.parent
if str(_QA_DIR) not in sys.path:
    sys.path.insert(0, str(_QA_DIR))

from eda_bus_utils import (  # noqa: E402
    find_existing_domain_event,
    resolve_origin_topology,
)

INDEX_MAP: dict[str, str] = {
    "process": "SddIA/process/index.md",
    "agent": "SddIA/agents/index.md",
    "skill": "SddIA/skills/index.md",
    "tool": "SddIA/tools/index.md",
    "action": "SddIA/actions/index.md",
    "codex": "SddIA/library/codexes/index.md",
}

ACTION_AGENT: dict[str, str] = {
    "sync-entity-index": "cumulo",
    "emit-pr-merged-event": "eda-bus",
    "emit-pr-presented-event": "eda-bus",
    "emit-domain-mutation": "eda-bus",
}


def _crypto(repo: Path, payload: dict[str, Any]) -> Any:
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


def _load_cumulo(repo: Path) -> dict[str, Any]:
    return json.loads((repo / "SddIA" / "core" / "cumulo.paths.json").read_text(encoding="utf-8"))


def _write_pending_event(repo: Path, event: dict[str, Any]) -> dict[str, str]:
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


def _run_emit_pr_merged(repo: Path, inputs: dict[str, Any], action_def: dict[str, Any]) -> dict[str, Any]:
    _ = action_def
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


def _run_emit_pr_presented(repo: Path, inputs: dict[str, Any], action_def: dict[str, Any]) -> dict[str, Any]:
    _ = action_def
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
        "payload": {"branch": branch.strip(), "status": status},
        "delivery_state": {},
    }
    seal = _write_pending_event(repo, event)
    return {
        "success": True,
        "event_id": seal["event_id"],
        "target_path": seal["target_path"],
        "event_type": "PullRequest_Presented",
    }


def _repo_root() -> Path:
    for parent in SCRIPT.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace (SddIA/core/cumulo.paths.json)")


def _emit(envelope: dict[str, Any], code: int | None = None) -> None:
    if code is None:
        code = 0 if envelope.get("success") else 1
    envelope.setdefault("exitCode", code)
    sys.stdout.write(json.dumps(envelope, ensure_ascii=False) + "\n")
    sys.exit(code)


def _resolve_action_path(repo: Path, action_name: str) -> Path:
    actions_dir = repo / "SddIA" / "actions"
    direct = actions_dir / f"{action_name}.md"
    if direct.is_file():
        return direct
    if yaml is None:
        raise RuntimeError("PyYAML requerido para resolver aliases de acción")
    for md in actions_dir.glob("*.md"):
        if md.stem in ("index", "actions-contract"):
            continue
        try:
            fm = yaml.safe_load(md.read_text(encoding="utf-8").split("---", 2)[1])
        except (IndexError, yaml.YAMLError):
            continue
        if isinstance(fm, dict) and fm.get("name") == action_name:
            return md
    raise FileNotFoundError(f"Acción no encontrada: {action_name}")


def _load_action_def(repo: Path, action_name: str) -> dict[str, Any]:
    path = _resolve_action_path(repo, action_name)
    if yaml is None:
        return {"name": path.stem}
    fm = yaml.safe_load(path.read_text(encoding="utf-8").split("---", 2)[1])
    return fm if isinstance(fm, dict) else {"name": path.stem}


def _invoke_bus_operator(repo: Path, operation: str, operation_payload: dict[str, Any]) -> dict[str, Any]:
    skill_script = repo / "scripts" / "skills" / "bus-operator.py"
    if not skill_script.is_file():
        raise FileNotFoundError(str(skill_script))
    req = {"operation": operation, "operation_payload": operation_payload}
    proc = subprocess.run(
        [sys.executable, str(skill_script)],
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
        raise RuntimeError(proc.stderr or "bus-operator sin salida")
    body = json.loads(stdout.splitlines()[-1])
    if not body.get("success"):
        raise RuntimeError(body.get("error") or "bus-operator failed")
    return body.get("result") or {}


def _run_emit_domain_mutation(repo: Path, inputs: dict[str, Any], action_def: dict[str, Any]) -> dict[str, Any]:
    _ = action_def
    entity_class = inputs.get("entity_class")
    entity_name = inputs.get("entity_name")
    lifecycle = inputs.get("lifecycle_operation")
    entity_uuid = inputs.get("entity_uuid")
    if not all(isinstance(x, str) for x in (entity_class, entity_name, lifecycle)):
        raise ValueError("entity_class, entity_name y lifecycle_operation son obligatorios")
    if lifecycle != "delete" and not entity_uuid:
        raise ValueError("entity_uuid obligatorio salvo delete")

    op = lifecycle
    event_type = {
        "create": "Domain_Entity_Created",
        "update": "Domain_Entity_Updated",
        "delete": "Domain_Entity_Deleted",
    }.get(op)
    if not event_type:
        raise ValueError(f"lifecycle_operation no soportada: {lifecycle}")

    origin_topology = inputs.get("origin_topology", "core")
    if origin_topology not in ("core", "local"):
        origin_topology = "core"

    if entity_uuid and lifecycle != "delete":
        existing = find_existing_domain_event(repo, entity_uuid, op, event_type)
        if existing and existing.get("event_id"):
            return {
                "success": True,
                "idempotent": True,
                "event_type": event_type,
                "event_id": existing["event_id"],
                "target_path": existing.get("target_path"),
            }

    event_id = _crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    event = {
        "event_id": event_id,
        "event_type": event_type,
        "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "emitter_agent": inputs.get("emitter_agent", "entity-manager"),
        "payload": {
            "entity_class": entity_class,
            "lifecycle_operation": op,
            "entity_uuid": entity_uuid,
            "entity_name": entity_name,
            "version": inputs.get("version"),
            "hash_signature_new": inputs.get("hash_signature_new"),
            "hash_signature_old": inputs.get("hash_signature_old"),
            "origin_topology": origin_topology,
            "changes_summary": inputs.get(
                "changes_summary",
                f"{op} {entity_class} {entity_name}",
            ),
        },
        "delivery_state": {},
    }
    seal = _write_pending_event(repo, event)
    return {"success": True, "event_type": event_type, **seal}


def _run_sync_entity_index(repo: Path, inputs: dict[str, Any], action_def: dict[str, Any]) -> dict[str, Any]:
    _ = action_def
    entity_class = inputs.get("entity_class", "")
    entity_name = inputs.get("entity_name", "")
    lifecycle_operation = inputs.get("lifecycle_operation", "")

    if entity_class == "norm":
        return {
            "success": True,
            "target_index_path": None,
            "message": "Indexación ignorada para norm.",
        }

    origin_topology = resolve_origin_topology(dict(inputs))
    if origin_topology == "local":
        return {
            "success": True,
            "target_index_path": None,
            "message": "Indexación canónica omitida (origin_topology=local).",
        }

    rel_path = INDEX_MAP.get(str(entity_class))
    if not rel_path:
        return {
            "success": True,
            "target_index_path": None,
            "message": f"entity_class desconocida ({entity_class}): no-op.",
        }

    if not entity_name:
        raise ValueError("entity_name requerido")

    target_file = repo / rel_path
    if not target_file.is_file():
        return {
            "success": False,
            "target_index_path": rel_path,
            "message": f"Índice inexistente: {rel_path}",
        }

    if lifecycle_operation == "delete":
        result = _invoke_bus_operator(
            repo,
            "sync_entity_index",
            {
                "entity_class": entity_class,
                "entity_name": entity_name,
                "lifecycle_operation": lifecycle_operation,
            },
        )
        return {
            "success": True,
            "target_index_path": rel_path,
            "message": f"Fila purgada para {entity_name}."
            if result.get("rows_removed", 0) or result.get("modified")
            else f"Sin fila para purgar (idempotente): {entity_name}.",
        }

    if lifecycle_operation in ("create", "update"):
        result = _invoke_bus_operator(
            repo,
            "sync_entity_index",
            {
                "entity_class": entity_class,
                "entity_name": entity_name,
                "lifecycle_operation": lifecycle_operation,
            },
        )
        exists = bool(result.get("exists"))
        return {
            "success": exists,
            "target_index_path": rel_path,
            "message": (
                f"Auditoría OK: fila presente para {entity_name}."
                if exists
                else f"ALERTA: Fila no encontrada para {entity_name}."
            ),
        }

    raise ValueError(f"lifecycle_operation no soportada: {lifecycle_operation}")


PHYSICAL_HANDLERS: dict[str, Any] = {
    "sync-entity-index": _run_sync_entity_index,
    "emit-pr-merged-event": _run_emit_pr_merged,
    "emit-pr-presented-event": _run_emit_pr_presented,
    "emit-domain-mutation": _run_emit_domain_mutation,
}


def run_action(repo: Path, action_name: str, action_inputs: dict[str, Any]) -> dict[str, Any]:
    action_def = _load_action_def(repo, action_name)
    canonical = action_def.get("name") or action_name
    handler = PHYSICAL_HANDLERS.get(canonical)
    if handler is None:
        return {
            "success": True,
            "status_code": 0,
            "data": {
                "action_name": canonical,
                "agent": ACTION_AGENT.get(canonical),
                "note": "contrato cargado; handler físico no implementado (simulado)",
            },
        }

    agent = ACTION_AGENT.get(canonical, "cumulo")
    data = handler(repo, action_inputs, action_def)
    inner_ok = data.get("success", True)
    return {
        "success": True,
        "status_code": 0,
        "data": {
            **data,
            "action_name": canonical,
            "delegated_agent": agent,
            "delegated_skill": "bus-operator" if canonical == "sync-entity-index" else None,
            "delegated_tool": "markdown-table-editor" if canonical == "sync-entity-index" else None,
        },
        "execution_report": {
            "action_name": canonical,
            "agent": agent,
            "business_success": inner_ok,
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser(description="execute-action (laboratorio SddIA)")
    parser.add_argument("--action", required=True, help="Nombre canónico de la acción")
    parser.add_argument("--inputs", help="JSON de inputs de la acción")
    parser.add_argument("--input-file", help="Ruta a JSON de inputs")
    args = parser.parse_args()

    try:
        if args.input_file:
            action_inputs = json.loads(Path(args.input_file).read_text(encoding="utf-8-sig"))
        elif args.inputs:
            action_inputs = json.loads(args.inputs)
        else:
            stdin = sys.stdin.read()
            action_inputs = json.loads(stdin) if stdin.strip() else {}
        if not isinstance(action_inputs, dict):
            raise ValueError("inputs deben ser objeto JSON")
    except json.JSONDecodeError as e:
        _emit({"success": False, "error": f"JSON inválido: {e}"}, 1)

    try:
        repo = _repo_root()
        result = run_action(repo, args.action.strip(), action_inputs)
        business = (result.get("data") or {}).get("success", True)
        if result.get("success") and business is False:
            result["status_code"] = 0
        _emit(result, result.get("status_code", 0))
    except Exception as e:
        _emit({"success": False, "status_code": 1, "error": str(e)}, 1)


if __name__ == "__main__":
    main()
