#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Laboratorio E2E Fase B: pending → watcher → processed (+ sync simulado)."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Any

SCRIPT = Path(__file__).resolve()
if str(SCRIPT.parent) not in sys.path:
    sys.path.insert(0, str(SCRIPT.parent))

from eda_bus_utils import ensure_event_bus_topology, list_witnesses, load_eda_bus

EXECUTE_PROCESS = SCRIPT.parent / "execute-process.py"
WATCHER = SCRIPT.parent.parent / "daemons" / "event-watcher.py"


def _repo_root() -> Path:
    for parent in SCRIPT.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace")


def _run_json(cmd: list[str], *, env: dict[str, str] | None = None) -> dict[str, Any]:
    proc = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(_repo_root()),
        env=env,
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if not line:
        raise RuntimeError(proc.stderr or "sin salida JSON")
    body = json.loads(line)
    if not body.get("success"):
        raise RuntimeError(body.get("error") or "comando falló")
    return body


def create_entity(repo: Path, entity_class: str, entity_name: str) -> dict[str, Any]:
    seed_key = {
        "tool": "tool_name",
        "action": "action_name",
        "process": "process_name",
        "agent": "agent_name",
        "norm": "tactical_norm_name",
        "codex": "domain_codex_slug",
        "skill": "skill_name",
        "event": "event_name",
    }.get(entity_class, "entity_name")
    payload = {
        "entity_class": entity_class,
        "entity_name": entity_name,
        "lifecycle_operation": "create",
        "semantic_seed": {
            seed_key: entity_name,
            "execution_logic": f"E2E lab {entity_class}",
            "orchestration_logic": f"E2E lab {entity_class}",
            "process_description": f"E2E {entity_name}",
            "agent_purpose": f"E2E {entity_name}",
            "tactical_norm_friction": f"E2E {entity_name}",
            "domain_codex_name": entity_name,
            "event_type": "E2E_Smoke_Event",
            "event_description": "Smoke E2E",
            "payload_required": ["entity_uuid"],
            "skill_inputs_schema": [],
            "skill_outputs_schema": [],
        },
    }
    if entity_class == "event":
        payload["semantic_seed"]["event_type"] = "E2E_" + entity_name.replace("-", "_").title()
    body = _run_json(
        [sys.executable, str(EXECUTE_PROCESS), "--process", "entity-manager", "--inputs", json.dumps(payload)]
    )
    return body.get("data") or {}


def route_event(repo: Path, rel_path: str) -> dict[str, Any]:
    env = os.environ.copy()
    env.setdefault("SDDIA_LAB_SIMULATE_IOTA", "1")
    env.setdefault("SDDIA_LAB_SIMULATE_SYNC_INDEX", "1")
    proc = subprocess.run(
        [sys.executable, str(WATCHER), "--event-file-path", rel_path],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        env=env,
        check=False,
    )
    line = (proc.stdout or "").strip()
    if not line:
        raise RuntimeError(proc.stderr or "watcher sin salida")
    return json.loads(line.splitlines()[-1])


def main() -> int:
    parser = argparse.ArgumentParser(description="E2E lab EDA Fase B")
    parser.add_argument("--entity-class", default="tool", help="Clase a forjar (default tool)")
    parser.add_argument("--entity-name", help="Nombre kebab-case; default auto")
    parser.add_argument("--event-file-path", help="Ruta relativa pending/ existente (omitir create)")
    parser.add_argument("--skip-create", action="store_true", help="Solo enrutar event-file-path")
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()

    repo = _repo_root()
    bus = ensure_event_bus_topology(repo)
    pending_dir = repo / bus["pending"]

    report: dict[str, Any] = {"steps": []}

    if args.event_file_path:
        rel = args.event_file_path.replace("\\", "/")
        event_id = Path(rel).stem
    elif args.skip_create:
        print("Indique --event-file-path o omita --skip-create", file=sys.stderr)
        return 1
    else:
        import uuid

        entity_name = args.entity_name or f"eda-e2e-{args.entity_class}-{uuid.uuid4().hex[:8]}"
        create_data = create_entity(repo, args.entity_class, entity_name)
        handoff = create_data.get("handoff") or {}
        event_id = handoff.get("event_id")
        rel = handoff.get("target_path")
        if not rel or not event_id:
            print(json.dumps({"success": False, "error": "entity-manager sin event_id", "data": create_data}))
            return 1
        report["steps"].append({"create": entity_name, "event_id": event_id})
        report["entity_class"] = args.entity_class
        report["entity_name"] = entity_name

    pending = repo / rel
    if not pending.is_file():
        print(json.dumps({"success": False, "error": f"pending no encontrado: {rel}"}))
        return 1

    route_result = route_event(repo, rel)
    report["steps"].append({"route": route_result})

    event_id = event_id or Path(rel).stem
    witnesses = list_witnesses(repo, bus, "processed_subscribers", event_id)
    processing_header = repo / bus["processing"] / f"{event_id}.json"
    report["witnesses_processed"] = [p.name for p in witnesses]
    report["processing_header_created"] = processing_header.is_file()
    report["parent_still_pending"] = pending.is_file()
    report["dispatch_mode"] = route_result.get("data", {}).get("dispatch_mode")
    report["success"] = bool(route_result.get("success")) and len(witnesses) > 0

    if args.json:
        print(json.dumps(report, indent=2, ensure_ascii=False))
    else:
        print(json.dumps(report, ensure_ascii=False))
    return 0 if report["success"] else 1


if __name__ == "__main__":
    sys.exit(main())
