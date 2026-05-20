#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""tool:manage-event-receipt — mutación de sufijos .notificado | .procesado | .error."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

TOOL_NAME = "manage-event-receipt"
ALLOWED_SUFFIXES = frozenset({".notificado", ".procesado", ".error"})


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace")


def _emit(envelope: dict[str, Any]) -> None:
    code = envelope.get("exitCode", 0 if envelope.get("success") else 1)
    envelope.setdefault("name", TOOL_NAME)
    envelope.setdefault("exitCode", code)
    sys.stdout.write(json.dumps(envelope, ensure_ascii=False) + "\n")
    sys.exit(code)


def _fail(message: str) -> None:
    _emit({"name": TOOL_NAME, "success": False, "exitCode": 1, "message": message, "error": message})


def _strip_known_suffixes(path: Path) -> Path:
    for suffix in ALLOWED_SUFFIXES:
        if path.name.endswith(suffix):
            return path.with_name(path.name[: -len(suffix)])
    return path


def main() -> None:
    try:
        raw = sys.stdin.read()
        req = json.loads(raw) if raw.strip() else {}
    except json.JSONDecodeError as e:
        _fail(f"JSON inválido: {e}")

    file_path = req.get("file_path")
    suffix = req.get("suffix")
    if not isinstance(file_path, str) or not file_path.strip():
        _fail("file_path requerido")
    if suffix not in ALLOWED_SUFFIXES:
        _fail(f"suffix debe ser uno de {sorted(ALLOWED_SUFFIXES)}")

    repo = _repo_root()
    source = (repo / file_path).resolve()
    repo_resolved = repo.resolve()
    if not str(source).startswith(str(repo_resolved)):
        _fail("file_path fuera del workspace")
    if not source.is_file():
        _fail(f"archivo inexistente: {file_path}")

    base = _strip_known_suffixes(source)
    target = base.with_name(base.name + suffix)
    if target.exists():
        _emit(
            {
                "name": TOOL_NAME,
                "success": True,
                "exitCode": 0,
                "message": "Sufijo ya aplicado (idempotente).",
                "result": {
                    "source_path": str(source.relative_to(repo)).replace("\\", "/"),
                    "target_path": str(target.relative_to(repo)).replace("\\", "/"),
                    "modified": False,
                },
            }
        )

    source.rename(target)
    _emit(
        {
            "name": TOOL_NAME,
            "success": True,
            "exitCode": 0,
            "message": "Sufijo aplicado.",
            "result": {
                "source_path": str(source.relative_to(repo)).replace("\\", "/"),
                "target_path": str(target.relative_to(repo)).replace("\\", "/"),
                "modified": True,
            },
        }
    )


if __name__ == "__main__":
    main()
