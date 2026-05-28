#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Cápsula sandbox-breacher — intento de escape workspace_path (Caos S+)."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

TOOL_NAME = "sandbox-breacher"


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace (SddIA/core/cumulo.paths.json)")


def _qa_import():
    qa = Path(__file__).resolve().parents[2] / "qa"
    if str(qa) not in sys.path:
        sys.path.insert(0, str(qa))
    from chaos_workspace_utils import assert_workspace_bound

    return assert_workspace_bound


def _emit(envelope: dict[str, Any]) -> None:
    code = envelope.get("exitCode", 0 if envelope.get("success") else 1)
    envelope.setdefault("name", TOOL_NAME)
    envelope.setdefault("exitCode", code)
    sys.stdout.write(json.dumps(envelope, ensure_ascii=False) + "\n")
    sys.exit(code)


def _fail(message: str, *, code: int = 1) -> None:
    _emit({
        "name": TOOL_NAME,
        "success": False,
        "exitCode": code,
        "message": message,
        "error": message,
        "result": {"breach_blocked": True},
    })


def _load_request() -> dict[str, Any]:
    raw = sys.stdin.read()
    if not raw.strip():
        return {}
    data = json.loads(raw)
    return data if isinstance(data, dict) else {}


def main() -> None:
    req = _load_request()
    ws_raw = req.get("workspace_path")
    if not isinstance(ws_raw, str) or not ws_raw.strip():
        _fail("workspace_path obligatorio")
    workspace_path = Path(ws_raw.strip())
    escape = req.get("escape_target")
    if not isinstance(escape, str) or not escape.strip():
        escape = "../breach-marker.txt"
    else:
        escape = escape.strip()

    repo = _repo_root()
    assert_workspace_bound = _qa_import()
    target = (workspace_path / escape).resolve()
    ok, err = assert_workspace_bound(repo, target, workspace_path)
    if not ok:
        _fail(err or "breach bloqueado por assert_workspace_bound")

    try:
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text("breach\n", encoding="utf-8")
    except OSError as exc:
        _fail(f"escritura fallida: {exc}")

    _emit({
        "name": TOOL_NAME,
        "success": True,
        "exitCode": 0,
        "message": "escritura dentro del workspace",
        "result": {"breach_blocked": False, "target": str(target)},
    })


if __name__ == "__main__":
    main()
