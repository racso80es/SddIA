# -*- coding: utf-8 -*-
"""Rutas efímeras del workspace — SSOT `.tmp/` (gitignored)."""

from __future__ import annotations

import json
import os
import uuid
from pathlib import Path
from typing import Any


def keep_tmp() -> bool:
    return os.environ.get("SDDIA_KEEP_TMP", "").strip().lower() in ("1", "true", "yes")


def repo_tmp_dir(repo: Path) -> Path:
    path = repo / ".tmp"
    path.mkdir(parents=True, exist_ok=True)
    return path


def write_ephemeral_json(repo: Path, prefix: str, payload: dict[str, Any]) -> Path:
    path = repo_tmp_dir(repo) / f"{prefix}-{uuid.uuid4().hex[:12]}.json"
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return path


def cleanup_path(path: Path | None, *, keep: bool | None = None) -> bool:
    if path is None:
        return False
    if keep if keep is not None else keep_tmp():
        return False
    if path.is_file():
        path.unlink(missing_ok=True)
        return True
    return False
