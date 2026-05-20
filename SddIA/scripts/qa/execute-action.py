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
from pathlib import Path
from typing import Any

try:
    import yaml
except ImportError:
    yaml = None  # type: ignore

SCRIPT = Path(__file__).resolve()

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
