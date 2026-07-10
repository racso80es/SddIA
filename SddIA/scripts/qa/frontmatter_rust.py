# -*- coding: utf-8 -*-
"""Frontmatter YAML vía parser Rust (`execute-process --parse-frontmatter`)."""

from __future__ import annotations

import json
import subprocess
import tempfile
from pathlib import Path
from typing import Any

from orchestrator_resolve import resolve_orchestrator_executable


def _repo_root_from_path(md_path: Path) -> Path:
    resolved = md_path.resolve()
    for parent in resolved.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError(
        "No se encontró raíz del workspace (SddIA/core/cumulo.paths.json)"
    )


def parse_frontmatter_path(md_path: Path) -> dict[str, Any]:
    path = md_path.resolve()
    if not path.is_file():
        raise FileNotFoundError(f"no existe: {path}")
    repo = _repo_root_from_path(path)
    exe = resolve_orchestrator_executable(repo)
    proc = subprocess.run(
        [str(exe), "--parse-frontmatter", str(path)],
        cwd=str(repo),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    if proc.returncode != 0:
        err = (proc.stderr or proc.stdout or "parse-frontmatter falló").strip()
        raise RuntimeError(err)
    line = (proc.stdout or "").strip()
    if not line:
        raise RuntimeError("parse-frontmatter: stdout vacío")
    data = json.loads(line)
    return data if isinstance(data, dict) else {}


def parse_frontmatter_text(text: str) -> dict[str, Any]:
    parts = text.split("---", 2)
    if len(parts) < 3:
        return {}
    with tempfile.NamedTemporaryFile(
        mode="w",
        suffix=".md",
        encoding="utf-8",
        delete=False,
    ) as tmp:
        tmp.write(text)
        tmp_path = Path(tmp.name)
    try:
        return parse_frontmatter_path(tmp_path)
    finally:
        tmp_path.unlink(missing_ok=True)
