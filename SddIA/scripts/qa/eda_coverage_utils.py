# -*- coding: utf-8 -*-
"""SSOT eda-coverage.json — correlación genómica durable."""

from __future__ import annotations

import json
import os
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

_DEFAULT_COVERAGE = "SddIA/core/eda-coverage.json"


def _iso_now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat()


def _load_cumulo(repo: Path) -> dict[str, Any]:
    cfg_path = repo / "SddIA" / "core" / "cumulo.paths.json"
    return json.loads(cfg_path.read_text(encoding="utf-8"))


def coverage_path(repo: Path) -> Path:
    rel = _DEFAULT_COVERAGE
    try:
        cfg = _load_cumulo(repo)
        if isinstance(cfg.get("eda_coverage"), str) and cfg["eda_coverage"].strip():
            rel = cfg["eda_coverage"].strip().replace("\\", "/")
    except (OSError, ValueError, json.JSONDecodeError):
        pass
    return repo / rel


def load_coverage(repo: Path) -> dict[str, Any]:
    path = coverage_path(repo)
    if not path.is_file():
        return {"version": "1.0.0", "coverage_matrix": {}}
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {"version": "1.0.0", "coverage_matrix": {}}
    if not isinstance(data.get("coverage_matrix"), dict):
        data["coverage_matrix"] = {}
    if not isinstance(data.get("version"), str):
        data["version"] = "1.0.0"
    return data


def _atomic_write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + ".tmp")
    tmp.write_text(content, encoding="utf-8")
    os.replace(tmp, path)


def save_coverage(repo: Path, data: dict[str, Any]) -> None:
    path = coverage_path(repo)
    _atomic_write(path, json.dumps(data, indent=2, ensure_ascii=False) + "\n")


def upsert_entity_coverage(
    repo: Path,
    entity_uuid: str,
    *,
    event_type: str,
    last_hash: str,
) -> None:
    data = load_coverage(repo)
    matrix = data.setdefault("coverage_matrix", {})
    matrix[entity_uuid] = {
        "is_covered": True,
        "last_emitted_event": event_type,
        "last_hash": last_hash,
        "correlation_timestamp": _iso_now(),
    }
    save_coverage(repo, data)


def remove_entity_coverage(repo: Path, entity_uuid: str) -> None:
    data = load_coverage(repo)
    matrix = data.get("coverage_matrix")
    if isinstance(matrix, dict) and entity_uuid in matrix:
        del matrix[entity_uuid]
        save_coverage(repo, data)


def is_entity_covered(repo: Path, entity_uuid: str) -> bool:
    data = load_coverage(repo)
    matrix = data.get("coverage_matrix") or {}
    entry = matrix.get(entity_uuid)
    return isinstance(entry, dict) and entry.get("is_covered") is True


def get_entity_coverage(repo: Path, entity_uuid: str) -> dict[str, Any] | None:
    data = load_coverage(repo)
    matrix = data.get("coverage_matrix") or {}
    entry = matrix.get(entity_uuid)
    return entry if isinstance(entry, dict) else None
