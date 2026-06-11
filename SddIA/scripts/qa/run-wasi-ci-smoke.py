#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""CI smoke: valida wasmtime, artefactos WASI y E2E con SDDIA_CI_REQUIRE_WASI."""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any

SCRIPT = Path(__file__).resolve()


def _repo_root() -> Path:
    for parent in SCRIPT.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace")


def _wasm_artifact(repo: Path, name: str) -> Path:
    return repo / "SddIA" / "target" / "wasm32-wasip1" / "debug" / f"{name}.wasm"


def _run_wasmtime(wasm: Path, payload: dict[str, Any], *, dir_mount: str | None = None) -> dict[str, Any]:
    cmd = ["wasmtime", "run"]
    if dir_mount is not None:
        cmd.extend([f"--dir={dir_mount}", str(wasm)])
    else:
        cmd.append(str(wasm))
    proc = subprocess.run(
        cmd,
        input=json.dumps(payload, ensure_ascii=False),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
    if not line:
        raise RuntimeError(proc.stderr or "wasmtime sin salida JSON")
    body = json.loads(line)
    if not body.get("success"):
        raise RuntimeError(body.get("error") or body.get("message") or proc.stderr or "wasmtime falló")
    return body


def main() -> int:
    parser = argparse.ArgumentParser(description="WASI CI smoke — toolchain y cápsulas mínimas")
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--skip-e2e", action="store_true", help="Omitir run-eda-e2e-lab")
    args = parser.parse_args()

    repo = _repo_root()
    report: dict[str, Any] = {"steps": [], "wasi_path_verified": False, "success": False}
    exit_code = 1

    try:
        wasmtime_bin = shutil.which("wasmtime")
        if not wasmtime_bin:
            raise RuntimeError("wasmtime not in PATH")
        ver = subprocess.run(
            ["wasmtime", "--version"],
            capture_output=True,
            text=True,
            check=False,
        )
        report["steps"].append(
            {"wasmtime": wasmtime_bin, "version": (ver.stdout or ver.stderr or "").strip()}
        )

        crypto_wasm = _wasm_artifact(repo, "cryptography-manager")
        wasi_poc = _wasm_artifact(repo, "wasi-poc")
        for label, path in (("cryptography-manager", crypto_wasm), ("wasi-poc", wasi_poc)):
            if not path.is_file():
                raise RuntimeError(f"artefacto WASI ausente: {path}")
            report["steps"].append({"artifact": label, "path": str(path.relative_to(repo))})

        uuid_body = _run_wasmtime(
            crypto_wasm,
            {"operation": "GENERATE_UUID", "target_payload": None},
            dir_mount="/",
        )
        uuid_val = uuid_body.get("result")
        if not isinstance(uuid_val, str) or not uuid_val:
            raise RuntimeError("cryptography-manager WASM no devolvió UUID")
        report["steps"].append({"crypto_wasm": "GENERATE_UUID", "ok": True})

        poc_body = _run_wasmtime(
            wasi_poc,
            {
                "meta": {"schemaVersion": "2.0", "entityKind": "tool", "entityId": "wasi-poc"},
                "request": {"ping": True},
            },
        )
        if not (poc_body.get("result") or {}).get("echo"):
            raise RuntimeError("wasi-poc WASM no devolvió echo esperado")
        report["steps"].append({"wasi_poc": "echo", "ok": True})

        report["wasi_path_verified"] = True

        if not args.skip_e2e:
            e2e_script = SCRIPT.parent / "run-eda-e2e-lab.py"
            env = os.environ.copy()
            env["SDDIA_CI_REQUIRE_WASI"] = "1"
            env.setdefault("SDDIA_LAB_SIMULATE_IOTA", "1")
            env.setdefault("SDDIA_LAB_SIMULATE_SYNC_INDEX", "1")
            env.setdefault("SDDIA_LAB_ROUTE_SYNC", "1")
            proc = subprocess.run(
                [sys.executable, str(e2e_script), "--entity-class", "tool", "--json"],
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
                raise RuntimeError(proc.stderr or "run-eda-e2e-lab sin salida")
            e2e = json.loads(line)
            report["steps"].append({"eda_e2e": e2e.get("success"), "cleaned": e2e.get("cleaned")})
            if not e2e.get("success"):
                raise RuntimeError(e2e.get("error") or "run-eda-e2e-lab falló bajo SDDIA_CI_REQUIRE_WASI")

        report["success"] = True
        exit_code = 0
    except Exception as exc:
        report["error"] = str(exc)

    if args.json:
        print(json.dumps(report, indent=2, ensure_ascii=False))
    else:
        print(json.dumps(report, ensure_ascii=False))
    return exit_code


if __name__ == "__main__":
    sys.exit(main())
