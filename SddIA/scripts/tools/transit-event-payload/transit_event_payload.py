#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""tool:transit-event-payload — traslado seguro de JSON entre carpetas del bus EDA."""

from __future__ import annotations

import json
import shutil
import sys
from pathlib import Path
from typing import Any

_SCRIPT_DIR = Path(__file__).resolve().parent
_QA_DIR = _SCRIPT_DIR.parents[2] / "qa"
if str(_QA_DIR) not in sys.path:
    sys.path.insert(0, str(_QA_DIR))

from eda_bus_utils import ensure_event_bus_topology, load_eda_bus  # noqa: E402

TOOL_NAME = "transit-event-payload"
VALID_BUCKETS = frozenset({"pending", "processing", "processed", "dead_letter"})


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


def main() -> None:
    try:
        raw = sys.stdin.read()
        req = json.loads(raw) if raw.strip() else {}
    except json.JSONDecodeError as e:
        _fail(f"JSON inválido: {e}")

    file_name = req.get("file_name")
    from_bucket = req.get("from_bucket")
    to_bucket = req.get("to_bucket")
    if not isinstance(file_name, str) or not file_name.strip():
        _fail("file_name requerido")
    if from_bucket not in VALID_BUCKETS or to_bucket not in VALID_BUCKETS:
        _fail(f"from_bucket y to_bucket deben ser uno de {sorted(VALID_BUCKETS)}")

    repo = _repo_root()
    bus = ensure_event_bus_topology(repo)
    source = (repo / bus[from_bucket] / file_name.strip()).resolve()
    dest_dir = (repo / bus[to_bucket]).resolve()
    dest = dest_dir / file_name.strip()
    repo_resolved = repo.resolve()
    if not str(source).startswith(str(repo_resolved)) or not str(dest).startswith(str(repo_resolved)):
        _fail("ruta fuera del workspace")
    if not source.is_file():
        _fail(f"origen inexistente: {source}")

    dest_dir.mkdir(parents=True, exist_ok=True)
    if dest.is_file():
        _emit(
            {
                "name": TOOL_NAME,
                "success": True,
                "exitCode": 0,
                "message": "Destino ya existe (idempotente).",
                "result": {
                    "target_path": str(dest.relative_to(repo)).replace("\\", "/"),
                    "modified": False,
                },
            }
        )

    shutil.move(str(source), str(dest))
    _emit(
        {
            "name": TOOL_NAME,
            "success": True,
            "exitCode": 0,
            "message": "Tránsito completado.",
            "result": {
                "target_path": str(dest.relative_to(repo)).replace("\\", "/"),
                "modified": True,
            },
        }
    )


if __name__ == "__main__":
    main()
