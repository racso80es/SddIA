#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Cápsula io-choke — simula fallo E/S al escribir en workspace (Caos S+)."""

from __future__ import annotations

import json
import os
import stat
import sys
from pathlib import Path
from typing import Any

TOOL_NAME = "io-choke"


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
        "result": None,
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
    target_name = req.get("target_file")
    if not isinstance(target_name, str) or not target_name.strip():
        target_name = ".io-choke-target"
    else:
        target_name = target_name.strip()

    repo = _repo_root()
    assert_workspace_bound = _qa_import()
    target = (workspace_path / target_name).resolve()
    ok, err = assert_workspace_bound(repo, target, workspace_path)
    if not ok:
        _fail(err or "target fuera de workspace_path")

    target.parent.mkdir(parents=True, exist_ok=True)
    if not target.exists():
        target.write_text("io-choke-seed\n", encoding="utf-8")

    try:
        os.chmod(target, stat.S_IREAD)
    except OSError as exc:
        _fail(f"no se pudo marcar read-only: {exc}")

    try:
        with target.open("a", encoding="utf-8") as handle:
            handle.write("choke-attempt\n")
        _fail("escritura no bloqueada — io-choke no aplicó asfixia", code=2)
    except OSError:
        _emit({
            "name": TOOL_NAME,
            "success": True,
            "exitCode": 0,
            "message": "asfixia E/S simulada",
            "result": {"io_choked": True, "target": str(target)},
        })


if __name__ == "__main__":
    main()
