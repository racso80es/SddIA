#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Golden harness P8/P9: compara envelope Rust vs Python (normaliza no-deterministas)."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any

SCRIPT = Path(__file__).resolve()
QA = SCRIPT.parent
if str(QA) not in sys.path:
    sys.path.insert(0, str(QA))

from orchestrator_resolve import resolve_orchestrator_cmd  # noqa: E402

UUID_RE = re.compile(
    r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}",
    re.I,
)


def repo_root() -> Path:
    for parent in SCRIPT.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("repo root not found")


def normalize(obj: Any) -> Any:
    """Elimina campos no deterministas para comparación estructural."""
    if isinstance(obj, dict):
        skip_keys = {
            "asset_id",
            "execution_id",
            "duration_ms",
            "event_id",
            "target_path",
            "thermodynamic_toll",
            "created",
        }
        out = {}
        for k, v in obj.items():
            if k in skip_keys:
                continue
            out[k] = normalize(v)
        return out
    if isinstance(obj, list):
        return [normalize(x) for x in obj]
    if isinstance(obj, str):
        return UUID_RE.sub("<UUID>", obj)
    return obj


def run_orchestrator(repo: Path, use_rust: bool, process: str, inputs: dict[str, Any]) -> dict[str, Any]:
    env = dict(__import__("os").environ)
    if use_rust:
        bin_path = repo / "SddIA/target/debug/execute-process"
        if not bin_path.is_file():
            bin_path = repo / "SddIA/target/release/execute-process"
        if not bin_path.is_file():
            raise FileNotFoundError("binario execute-process no compilado")
        cmd = [str(bin_path), "--process", process, "--inputs", json.dumps(inputs, ensure_ascii=False)]
    else:
        cmd = resolve_orchestrator_cmd(
            repo,
            ["--process", process, "--inputs", json.dumps(inputs, ensure_ascii=False)],
        )
        # Forzar Python legacy para referencia
        cmd = [sys.executable, str(repo / "SddIA/scripts/qa/execute-process.py"), "--process", process, "--inputs", json.dumps(inputs, ensure_ascii=False)]

    proc = subprocess.run(
        cmd,
        cwd=str(repo),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        env=env,
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if not line:
        raise RuntimeError(proc.stderr or "sin salida")
    return json.loads(line)


CASES: list[tuple[str, dict[str, Any]]] = [
    ("kalma2-interact", {"prompt": "golden parity ping"}),
]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--process", help="Proceso individual")
    parser.add_argument("--inputs", help="JSON inputs")
    args = parser.parse_args()
    repo = repo_root()

    cases = CASES
    if args.process:
        inputs = json.loads(args.inputs or "{}")
        cases = [(args.process, inputs)]

    failed = 0
    for process, inputs in cases:
        try:
            rust_env = dict(__import__("os").environ)
            py_body = run_orchestrator(repo, False, process, inputs)
            rust_body = run_orchestrator(repo, True, process, inputs)
            nr = normalize(rust_body)
            np = normalize(py_body)
            if nr == np and rust_body.get("success") == py_body.get("success"):
                print(f"OK  {process}")
            else:
                failed += 1
                print(f"FAIL {process}")
                print("  rust:", json.dumps(nr, ensure_ascii=False)[:200])
                print("  py  :", json.dumps(np, ensure_ascii=False)[:200])
        except Exception as exc:
            failed += 1
            print(f"ERR {process}: {exc}")

    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
