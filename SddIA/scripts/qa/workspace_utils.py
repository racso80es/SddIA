# -*- coding: utf-8 -*-
"""Resolución SSOT de workspaces dinámicos (Fase 2 telemetría EDA)."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any
from uuid import uuid4


def load_paths_config(repo: Path) -> dict[str, Any]:
    cfg_path = repo / "SddIA" / "core" / "cumulo.paths.json"
    data = json.loads(cfg_path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise ValueError("cumulo.paths.json inválido")
    local = repo / ".SddIA" / "local.paths.json"
    if local.is_file():
        try:
            overlay = json.loads(local.read_text(encoding="utf-8"))
            if isinstance(overlay, dict):
                for key, value in overlay.items():
                    if isinstance(value, dict) and isinstance(data.get(key), dict):
                        merged = dict(data[key])
                        merged.update(value)
                        data[key] = merged
                    else:
                        data[key] = value
        except (OSError, json.JSONDecodeError):
            pass
    return data


def _paths_block(cfg: dict[str, Any]) -> dict[str, Any]:
    paths = cfg.get("paths")
    return paths if isinstance(paths, dict) else {}


def _documentation_root(cfg: dict[str, Any]) -> str:
    directories = cfg.get("directories")
    if isinstance(directories, dict):
        doc = directories.get("documentation")
        if isinstance(doc, str) and doc.strip():
            return doc.strip().replace("\\", "/").rstrip("/")
    return "docs"


def resolve_workspaces_root(repo: Path, cfg: dict[str, Any] | None = None) -> Path:
    data = cfg if cfg is not None else load_paths_config(repo)
    paths = _paths_block(data)
    root = paths.get("workspacesRoot")
    if isinstance(root, str) and root.strip():
        rel = root.strip().replace("\\", "/")
        return (repo / rel).resolve()
    return (repo / ".SddIA" / "workspaces").resolve()


def resolve_documentation_features_path(repo: Path, cfg: dict[str, Any] | None = None) -> str:
    data = cfg if cfg is not None else load_paths_config(repo)
    paths = _paths_block(data)
    feature = paths.get("featurePath")
    if isinstance(feature, str) and feature.strip():
        return feature.strip().replace("\\", "/").rstrip("/")
    return f"{_documentation_root(data)}/features"


def resolve_documentation_fixes_path(repo: Path, cfg: dict[str, Any] | None = None) -> str:
    data = cfg if cfg is not None else load_paths_config(repo)
    paths = _paths_block(data)
    fix = paths.get("fixPath")
    if isinstance(fix, str) and fix.strip():
        return fix.strip().replace("\\", "/").rstrip("/")
    return f"{_documentation_root(data)}/fixes"


def load_workspace_template(process_def: dict[str, Any]) -> str:
    template = process_def.get("workspace_template")
    if not isinstance(template, str) or not template.strip():
        raise ValueError(
            "workspace_template ausente en definición del proceso (process-contract v1.4.0)"
        )
    return template.strip()


def materialize_workspace(
    repo: Path,
    process_name: str,
    template: str,
    execution_id: str,
    cfg: dict[str, Any] | None = None,
) -> Path:
    rel = template.format(process_name=process_name, execution_id=execution_id)
    rel_norm = rel.replace("\\", "/")
    if rel_norm.startswith(".SddIA/workspaces/") or rel_norm.startswith("SddIA/"):
        workspace_path = (repo / rel_norm).resolve()
    else:
        workspace_path = (resolve_workspaces_root(repo, cfg) / rel_norm).resolve()
    workspace_path.mkdir(parents=True, exist_ok=True)
    return workspace_path


def new_execution_id(process_inputs: dict[str, Any]) -> str:
    existing = process_inputs.get("execution_id")
    if isinstance(existing, str) and existing.strip():
        return existing.strip()
    return str(uuid4())


def sync_workspace_context(process_inputs: dict[str, Any], state: dict[str, Any]) -> None:
    for key in ("workspace_path", "execution_id", "persist_ref"):
        value = state.get(key)
        if value is not None and not process_inputs.get(key):
            process_inputs[key] = value


def materialize_child_workspace(
    orchestrator_workspace: Path,
    node_index: int,
    process_name: str,
    execution_id: str,
) -> Path:
    rel = orchestrator_workspace / "nodes" / f"{node_index:02d}-{process_name}" / execution_id
    rel.mkdir(parents=True, exist_ok=True)
    return rel.resolve()


def bootstrap_process_workspace(
    repo: Path,
    process_name: str,
    process_def: dict[str, Any],
    process_inputs: dict[str, Any],
    state: dict[str, Any],
) -> dict[str, Any]:
    execution_id = new_execution_id(process_inputs)
    template = load_workspace_template(process_def)
    existing_ws = process_inputs.get("workspace_path")
    if isinstance(existing_ws, str) and existing_ws.strip():
        ws_path = Path(existing_ws).resolve()
        ws_path.mkdir(parents=True, exist_ok=True)
        ws_str = str(ws_path)
    else:
        workspace_path = materialize_workspace(repo, process_name, template, execution_id)
        ws_str = str(workspace_path)
    state["execution_id"] = execution_id
    state["workspace_path"] = ws_str
    process_inputs["execution_id"] = execution_id
    process_inputs["workspace_path"] = ws_str
    sync_workspace_context(process_inputs, state)
    return {
        "execution_id": execution_id,
        "workspace_path": ws_str,
        "workspace_template": template,
    }
