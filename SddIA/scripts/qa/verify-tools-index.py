#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Cortafuegos CI: sincronización index.md <-> definiciones {name}.md (Core y local)."""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path

SCRIPT = Path(__file__).resolve()
REPO_ROOT = SCRIPT.parents[3] if (SCRIPT.parents[2] / "tools").is_dir() else SCRIPT.parents[2]

CORE_DIR = REPO_ROOT / "SddIA" / "tools"
CORE_INDEX = CORE_DIR / "index.md"
LOCAL_DIR = REPO_ROOT / ".SddIA" / "tools"
LOCAL_INDEX = LOCAL_DIR / "index.md"
LEGACY_LOCAL_DIRNAME = "Tools"

EXCLUDE_STEMS = frozenset({"index", "tools-contract", "README"})
EXCLUDE_SUFFIXES = ("-contract.md",)


def _is_tool_definition(path: Path) -> bool:
    if path.suffix.lower() != ".md":
        return False
    stem = path.stem
    if stem in EXCLUDE_STEMS:
        return False
    if any(path.name.endswith(s) for s in EXCLUDE_SUFFIXES):
        return False
    return True


def _scan_definitions(tools_dir: Path) -> set[str]:
    if not tools_dir.is_dir():
        return set()
    return {p.name for p in tools_dir.iterdir() if p.is_file() and _is_tool_definition(p)}


def _parse_index_filenames(index_path: Path) -> set[str]:
    if not index_path.is_file():
        return set()
    text = index_path.read_text(encoding="utf-8")
    in_catalog = False
    indexed: set[str] = set()
    for line in text.splitlines():
        if "## Catálogo" in line or "## Catalogo" in line:
            in_catalog = True
            continue
        if in_catalog and line.startswith("## ") and "Catálogo" not in line and "Catalogo" not in line:
            break
        if not in_catalog or not line.strip().startswith("|"):
            continue
        if re.match(r"^\|\s*[-:]+", line):
            continue
        if "Archivo fuente" in line or line.strip() == "| name |":
            continue
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if not cells:
            continue
        first = cells[0].strip()
        m = re.search(r"`([^`]+)`", first)
        token = (m.group(1) if m else first).strip()
        if not token or token.lower() in ("archivo fuente", "name"):
            continue
        if not token.endswith(".md"):
            token = f"{token}.md"
        indexed.add(token)
    return indexed


def _audit_legacy_tools_dirs(repo_root: Path) -> list[str]:
    errors: list[str] = []
    for dot in repo_root.rglob(".SddIA"):
        if not dot.is_dir():
            continue
        try:
            names = {n for n in os.listdir(dot) if (dot / n).is_dir()}
        except OSError:
            continue
        if LEGACY_LOCAL_DIRNAME in names:
            legacy = dot / LEGACY_LOCAL_DIRNAME
            errors.append(
                f"Violación de Simetría Fractal: {legacy} — exigido `{dot / 'tools'}` en minúscula estricta"
            )
    return errors


def _audit_scope(label: str, tools_dir: Path, index_path: Path) -> list[str]:
    errors: list[str] = []
    if not tools_dir.is_dir() and not index_path.is_file():
        return errors
    if index_path.is_file() and not tools_dir.is_dir():
        errors.append(f"{label}: index exists but directory missing: {tools_dir}")
        return errors
    if tools_dir.is_dir() and not index_path.is_file():
        errors.append(f"{label}: definitions present but index missing: {index_path}")
        return errors

    on_disk = _scan_definitions(tools_dir)
    in_index = _parse_index_filenames(index_path)

    for name in sorted(on_disk - in_index):
        errors.append(f"{label}: orphan file (not in index): {tools_dir / name}")
    for name in sorted(in_index - on_disk):
        errors.append(f"{label}: orphan index row (no file): {name}")
    return errors


def main() -> int:
    errors: list[str] = []
    errors.extend(_audit_legacy_tools_dirs(REPO_ROOT))
    errors.extend(_audit_scope("core", CORE_DIR, CORE_INDEX))
    if LOCAL_INDEX.is_file() or LOCAL_DIR.is_dir():
        errors.extend(_audit_scope("local", LOCAL_DIR, LOCAL_INDEX))

    if errors:
        print("verify-tools-index: FAILED", file=sys.stderr)
        for e in errors:
            print(e, file=sys.stderr)
        return 1
    print("verify-tools-index: OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
