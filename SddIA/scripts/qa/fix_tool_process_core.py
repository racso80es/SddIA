# -*- coding: utf-8 -*-
"""fix-tool-process — sandbox Self-Healing (Fase 4)."""

from __future__ import annotations

import json
import os
from pathlib import Path
from typing import Any

from eda_bus_utils import _write_json_atomic, load_radamanto_config
from radamanto_batch_core import set_structure_valid


def sandbox_root(repo: Path) -> Path:
    cfg = load_radamanto_config(repo)
    return repo / (cfg.get("sandbox_root") or ".SddIA/sandbox/")


def materialize_sandbox(repo: Path, entity_id: str, recovery_attempt: int) -> Path:
    root = sandbox_root(repo) / entity_id.replace(":", "_") / str(recovery_attempt)
    root.mkdir(parents=True, exist_ok=True)
    (root / "fix-artifact.md").write_text(
        f"# Fix sandbox\n\nentity={entity_id}\nattempt={recovery_attempt}\n",
        encoding="utf-8",
    )
    return root


def _production_roots(repo: Path) -> list[Path]:
    return [
        (repo / "SddIA" / "tools").resolve(),
        (repo / "SddIA" / "skills").resolve(),
    ]


def assert_sandbox_write(repo: Path, target: Path, sandbox: Path) -> tuple[bool, str | None]:
    if os.environ.get("SDDIA_SANDBOX_STRICT", "1").strip().lower() in ("0", "false", "no"):
        return True, None
    try:
        target.resolve().relative_to(sandbox.resolve())
        return True, None
    except ValueError:
        pass
    for prod in _production_roots(repo):
        try:
            target.resolve().relative_to(prod)
            return False, f"write prohibido en produccion: {target}"
        except ValueError:
            continue
    return True, None


def run_argos_structure_gate(repo: Path, sandbox: Path, entity_id: str) -> dict[str, Any]:
    artifact = sandbox / "fix-artifact.md"
    structure_valid = artifact.is_file() and artifact.stat().st_size > 0
    gate = {
        "structure_valid": structure_valid,
        "entity_id": entity_id,
        "sandbox": str(sandbox.relative_to(repo)).replace("\\", "/"),
        "emitter": "argos",
        "emits_status_restored": False,
    }
    _write_json_atomic(sandbox / "argos_gate.json", gate)
    if structure_valid:
        set_structure_valid(repo, entity_id, True)
    return gate


def process_fix_tool(repo: Path, rel_path: str) -> dict[str, Any]:
    event_path = (repo / rel_path.strip()).resolve()
    if not event_path.is_file():
        return {"ok": False, "error": f"no existe: {rel_path}"}
    try:
        event = json.loads(event_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        return {"ok": False, "error": str(exc)}

    if event.get("event_type") != "Tool_Degraded":
        return {"ok": False, "error": "solo Tool_Degraded inicia fix-tool-process"}

    payload = event.get("payload") or {}
    if not isinstance(payload, dict):
        return {"ok": False, "error": "payload invalido"}
    entity_id = str(payload.get("target_entity_id") or "unknown")
    attempt = int(payload.get("recovery_attempt") or 1)

    sandbox = materialize_sandbox(repo, entity_id, attempt)
    ok_write, err = assert_sandbox_write(repo, sandbox / "fix-artifact.md", sandbox)
    if not ok_write:
        return {"ok": False, "error": err}

    gate = run_argos_structure_gate(repo, sandbox, entity_id)
    return {
        "ok": True,
        "entity_id": entity_id,
        "sandbox": str(sandbox.relative_to(repo)).replace("\\", "/"),
        "argos_gate": gate,
        "status_restored_emitted": False,
    }
