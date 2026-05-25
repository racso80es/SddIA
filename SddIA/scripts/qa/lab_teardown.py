# -*- coding: utf-8 -*-
"""Teardown de artefactos de laboratorio (forges locales + bus EDA)."""

from __future__ import annotations

import re
from pathlib import Path

from eda_bus_utils import list_witnesses, load_eda_bus
from tmp_paths import keep_tmp


def remove_index_row(index_path: Path, name: str) -> bool:
    if not index_path.is_file():
        return False
    lines = index_path.read_text(encoding="utf-8").splitlines()
    filtered = [line for line in lines if not (line.startswith("|") and name in line)]
    if len(filtered) == len(lines):
        return False
    index_path.write_text("\n".join(filtered) + ("\n" if filtered else ""), encoding="utf-8")
    return True


def local_tool_paths(repo: Path, entity_name: str) -> tuple[Path, Path]:
    base = repo / ".SddIA" / "tools"
    return base / f"{entity_name}.md", base / "index.md"


def cleanup_eda_bus_event(repo: Path, event_id: str) -> None:
    bus = load_eda_bus(repo)
    for key in ("pending", "processing", "processed", "dead_letter"):
        rel = bus.get(key)
        if isinstance(rel, str) and rel.strip():
            (repo / rel / f"{event_id}.json").unlink(missing_ok=True)
    for state_key in (
        "processing_subscribers",
        "processed_subscribers",
        "dead_letter_subscribers",
    ):
        for witness in list_witnesses(repo, bus, state_key, event_id):
            witness.unlink(missing_ok=True)


def cleanup_lab_entity_forge(
    repo: Path,
    *,
    entity_class: str,
    entity_name: str,
    event_id: str | None,
) -> dict[str, bool]:
    if keep_tmp():
        return {"skipped": True}

    cleaned: dict[str, bool] = {"artifact_removed": False, "index_row_removed": False, "bus_cleaned": False}

    if entity_class == "tool":
        artifact, index_path = local_tool_paths(repo, entity_name)
        if artifact.is_file():
            artifact.unlink(missing_ok=True)
            cleaned["artifact_removed"] = True
        cleaned["index_row_removed"] = remove_index_row(index_path, entity_name)

    if event_id:
        cleanup_eda_bus_event(repo, event_id)
        cleaned["bus_cleaned"] = True

    return cleaned


def cleanup_orphan_core_eda_e2e_tools(repo: Path) -> list[str]:
    """Elimina forges lab huérfanos bajo SddIA/tools/eda-e2e-*."""
    if keep_tmp():
        return []
    removed: list[str] = []
    tools_dir = repo / "SddIA" / "tools"
    if not tools_dir.is_dir():
        return removed
    pattern = re.compile(r"^eda-e2e-(tool|action|process|agent|norm|codex|skill|event)-[0-9a-f]{8}\.md$")
    for path in tools_dir.glob("eda-e2e-*.md"):
        if pattern.match(path.name):
            path.unlink(missing_ok=True)
            remove_index_row(tools_dir / "index.md", path.stem)
            removed.append(str(path.relative_to(repo)).replace("\\", "/"))
    return removed
