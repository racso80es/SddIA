#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Cápsula bus-operator: orquestación ciega de tools del bus EDA."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Any

ALLOWED_OPS = frozenset(
    {"resolve_subscribers", "transit_payload", "apply_receipt", "sync_entity_index"}
)

TOOL_SCRIPTS: dict[str, str] = {
    "read-event-subscriptions": "SddIA/scripts/tools/read-event-subscriptions/read_event_subscriptions.py",
    "manage-event-receipt": "SddIA/scripts/tools/manage-event-receipt/manage_event_receipt.py",
    "transit-event-payload": "SddIA/scripts/tools/transit-event-payload/transit_event_payload.py",
    "markdown-table-editor": "SddIA/scripts/tools/markdown-table-editor/markdown_table_editor.py",
}


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace")


def _emit(out: dict[str, Any]) -> None:
    sys.stdout.write(json.dumps(out, ensure_ascii=False))


def _fail(msg: str) -> None:
    _emit({"success": False, "exitCode": 1, "error": msg})
    sys.exit(1)


def _invoke_tool(repo: Path, tool_key: str, payload: dict[str, Any]) -> dict[str, Any]:
    rel = TOOL_SCRIPTS.get(tool_key)
    if not rel:
        _fail(f"tool no mapeada: {tool_key}")
    script = repo / rel
    if not script.is_file():
        _fail(f"cápsula inexistente: {rel}")
    proc = subprocess.run(
        [sys.executable, str(script)],
        input=json.dumps(payload, ensure_ascii=False),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )
    stdout = (proc.stdout or "").strip()
    if not stdout:
        _fail(proc.stderr or f"{tool_key} sin salida")
    body = json.loads(stdout.splitlines()[-1])
    if not body.get("success"):
        _fail(body.get("error") or body.get("message") or f"{tool_key} failed")
    return body.get("result") or {}


def _sync_entity_index_payload(payload: dict[str, Any]) -> dict[str, Any]:
    entity_class = str(payload.get("entity_class", ""))
    entity_name = str(payload.get("entity_name", ""))
    lifecycle = str(payload.get("lifecycle_operation", ""))

    index_map = {
        "process": "SddIA/process/index.md",
        "agent": "SddIA/agents/index.md",
        "skill": "SddIA/skills/index.md",
        "tool": "SddIA/tools/index.md",
        "action": "SddIA/actions/index.md",
        "codex": "SddIA/library/codexes/index.md",
    }
    rel_path = index_map.get(entity_class)
    if not rel_path:
        return {"noop": True, "entity_class": entity_class}

    if lifecycle == "delete":
        return {
            "file_path": rel_path,
            "operation": "delete_row",
            "key_column": "name",
            "row_data": {"name": entity_name},
            "match_token": entity_name,
        }
    if lifecycle in ("create", "update"):
        return {
            "file_path": rel_path,
            "operation": "row_exists",
            "key_column": "name",
            "row_data": {"name": entity_name},
            "match_token": entity_name,
        }
    raise ValueError(f"lifecycle_operation no soportada: {lifecycle}")


def main() -> None:
    try:
        doc = json.loads(sys.stdin.read() or "{}")
    except json.JSONDecodeError as e:
        _fail(f"JSON inválido: {e}")

    op = doc.get("operation")
    payload = doc.get("operation_payload")
    if op not in ALLOWED_OPS:
        _fail(f"operation debe ser uno de {sorted(ALLOWED_OPS)}")
    if not isinstance(payload, dict):
        _fail("operation_payload debe ser objeto")

    repo = _repo_root()
    try:
        if op == "resolve_subscribers":
            result = _invoke_tool(repo, "read-event-subscriptions", payload)
        elif op == "transit_payload":
            result = _invoke_tool(repo, "transit-event-payload", payload)
        elif op == "apply_receipt":
            result = _invoke_tool(repo, "manage-event-receipt", payload)
        elif op == "sync_entity_index":
            if payload.get("entity_class") == "norm":
                result = {"skipped": True, "message": "norm no indexada"}
            else:
                tool_payload = _sync_entity_index_payload(payload)
                if tool_payload.get("noop"):
                    result = tool_payload
                else:
                    result = _invoke_tool(repo, "markdown-table-editor", tool_payload)
        else:
            _fail(f"operation no implementada: {op}")
    except ValueError as e:
        _fail(str(e))

    _emit({"success": True, "exitCode": 0, "result": result})
    sys.exit(0)


if __name__ == "__main__":
    main()
