#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Lanzador de cápsula tool — stdin JSON → stdout envelope JSON."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

SCRIPT = Path(__file__).resolve()
QA = SCRIPT.parent.parent / "qa"
if str(QA) not in sys.path:
    sys.path.insert(0, str(QA))

from capsule_resolve import invoke_tool_capsule_json  # noqa: E402
from env_loader import load_hierarchical_env  # noqa: E402


def repo_root() -> Path:
    for parent in SCRIPT.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise SystemExit("[ERROR] No se encontró raíz del workspace (cumulo.paths.json).")


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Invoca una tool catalogada (cápsula Rust WASI/nativa)."
    )
    parser.add_argument("tool_name", help="Nombre kebab-case de la tool")
    parser.add_argument(
        "--prefer-native",
        action="store_true",
        help="Preferir binario nativo sobre wasm32-wasip1",
    )
    args = parser.parse_args()

    repo = repo_root()
    load_hierarchical_env(repo)

    raw = sys.stdin.read()
    payload: dict = {}
    if raw.strip():
        try:
            parsed = json.loads(raw)
        except json.JSONDecodeError as exc:
            print(
                json.dumps(
                    {
                        "name": args.tool_name,
                        "success": False,
                        "exitCode": 1,
                        "message": f"stdin JSON inválido: {exc}",
                    },
                    ensure_ascii=False,
                )
            )
            return 1
        if not isinstance(parsed, dict):
            print(
                json.dumps(
                    {
                        "name": args.tool_name,
                        "success": False,
                        "exitCode": 1,
                        "message": "stdin debe ser un objeto JSON",
                    },
                    ensure_ascii=False,
                )
            )
            return 1
        payload = parsed

    try:
        rc, body = invoke_tool_capsule_json(
            repo,
            args.tool_name,
            payload,
            prefer_wasm=not args.prefer_native,
        )
    except FileNotFoundError as exc:
        print(
            json.dumps(
                {
                    "name": args.tool_name,
                    "success": False,
                    "exitCode": 1,
                    "message": str(exc),
                },
                ensure_ascii=False,
            )
        )
        return 1

    print(json.dumps(body, ensure_ascii=False))
    return int(rc)


if __name__ == "__main__":
    raise SystemExit(main())
