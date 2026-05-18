#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Reconciliación determinista de index.md tras eventos Domain_Entity_* (cápsula watcher)."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Any

INDEX_MAP: dict[str, str] = {
    "process": "SddIA/process/index.md",
    "agent": "SddIA/agents/index.md",
    "skill": "SddIA/skills/index.md",
    "tool": "SddIA/tools/index.md",
    "action": "SddIA/actions/index.md",
    "codex": "SddIA/library/codexes/index.md",
}


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace (SddIA/core/cumulo.paths.json)")


def _emit(envelope: dict[str, Any]) -> None:
    sys.stdout.write(json.dumps(envelope, ensure_ascii=False) + "\n")


def _row_matches_entity(line: str, entity_name: str) -> bool:
    if not line.strip().startswith("|"):
        return False
    if re.match(r"^\|\s*[-:]+", line):
        return False
    return entity_name in line


def _find_entity_row(lines: list[str], entity_name: str) -> bool:
    for line in lines:
        if _row_matches_entity(line, entity_name):
            return True
    return False


def _delete_entity_rows(lines: list[str], entity_name: str) -> list[str]:
    out: list[str] = []
    for line in lines:
        if _row_matches_entity(line, entity_name):
            continue
        out.append(line)
    return out


def main() -> None:
    try:
        raw = sys.stdin.read()
        payload = json.loads(raw) if raw.strip() else {}
    except json.JSONDecodeError as e:
        _emit({
            "success": False,
            "exitCode": 1,
            "data": None,
            "error": f"stdin JSON inválido: {e}",
        })
        return

    entity_class = payload.get("entity_class", "")
    entity_name = payload.get("entity_name", "")
    lifecycle_operation = payload.get("lifecycle_operation", "")

    if entity_class == "norm":
        _emit({
            "success": True,
            "exitCode": 0,
            "data": {
                "success": True,
                "message": "Indexación ignorada para norm.",
            },
        })
        return

    rel_path = INDEX_MAP.get(entity_class)
    if not rel_path:
        _emit({
            "success": True,
            "exitCode": 0,
            "data": {
                "success": True,
                "message": f"entity_class desconocida ({entity_class}): no-op.",
            },
        })
        return

    if not entity_name:
        _emit({
            "success": False,
            "exitCode": 1,
            "data": None,
            "error": "entity_name requerido",
        })
        return

    repo = _repo_root()
    target = repo / rel_path

    if not target.is_file():
        _emit({
            "success": True,
            "exitCode": 0,
            "data": {
                "success": False,
                "target_index_path": rel_path,
                "message": f"Índice inexistente: {rel_path}",
            },
        })
        return

    text = target.read_text(encoding="utf-8")
    lines = text.splitlines(keepends=True)

    if lifecycle_operation == "delete":
        new_lines = _delete_entity_rows(lines, entity_name)
        if new_lines != lines:
            target.write_text("".join(new_lines), encoding="utf-8")
        _emit({
            "success": True,
            "exitCode": 0,
            "data": {
                "success": True,
                "target_index_path": rel_path,
                "message": f"Fila purgada para {entity_name}.",
            },
        })
        return

    if lifecycle_operation in ("create", "update"):
        exists = _find_entity_row(lines, entity_name)
        if exists:
            _emit({
                "success": True,
                "exitCode": 0,
                "data": {
                    "success": True,
                    "target_index_path": rel_path,
                    "message": f"Auditoría OK: fila presente para {entity_name}.",
                },
            })
        else:
            _emit({
                "success": True,
                "exitCode": 0,
                "data": {
                    "success": False,
                    "target_index_path": rel_path,
                    "message": f"ALERTA: Fila no encontrada para {entity_name}.",
                },
            })
        return

    _emit({
        "success": False,
        "exitCode": 1,
        "data": None,
        "error": f"lifecycle_operation no soportada: {lifecycle_operation}",
    })


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        _emit({
            "success": False,
            "exitCode": 1,
            "data": None,
            "error": str(e),
        })
