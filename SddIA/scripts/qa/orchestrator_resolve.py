#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Resolución SSOT del ejecutable orquestador (binario Rust preferente, fallback Python)."""

from __future__ import annotations

import os
import sys
from pathlib import Path


def resolve_orchestrator_cmd(repo: Path, extra_args: list[str]) -> list[str]:
    """Devuelve argv para invocar el orquestador con `extra_args` (--process, --inputs, …)."""
    override = os.environ.get("SDDIA_EXECUTE_PROCESS_BIN", "").strip()
    if override:
        return [override, *extra_args]
    for rel in (
        "SddIA/target/debug/execute-process",
        "SddIA/target/release/execute-process",
    ):
        candidate = repo / rel
        if candidate.is_file() and os.access(candidate, os.X_OK):
            return [str(candidate), *extra_args]
    return [
        sys.executable,
        str(repo / "SddIA" / "scripts" / "qa" / "execute-process.py"),
        *extra_args,
    ]


def resolve_orchestrator_executable(repo: Path) -> Path:
    """Ruta al ejecutable orquestador (binario o script Python)."""
    cmd = resolve_orchestrator_cmd(repo, [])
    return Path(cmd[0])


def _find_repo_from_script() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del repo (cumulo.paths.json)")


if __name__ == "__main__":
    import os

    repo = _find_repo_from_script()
    cmd = resolve_orchestrator_cmd(repo, sys.argv[1:])
    os.execvp(cmd[0], cmd)
