# -*- coding: utf-8 -*-
"""Inocuidad del Caos — confinamiento workspace_path (Fase 1.C)."""

from __future__ import annotations

from pathlib import Path


def assert_workspace_bound(
    repo: Path,
    target: Path,
    workspace_path: Path,
) -> tuple[bool, str | None]:
    _ = repo
    if ".." in target.parts:
        return False, f"path traversal prohibido: {target}"
    try:
        ws = workspace_path.resolve()
        resolved = target.resolve()
        resolved.relative_to(ws)
        return True, None
    except ValueError:
        return False, f"target fuera de workspace_path: {target}"
