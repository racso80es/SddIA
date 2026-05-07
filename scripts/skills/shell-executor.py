#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Cápsula shell-executor: ejecución determinista sin shell; I/O JSON por stdin/stdout (SSOT congelado).

Reglas principales:
- Prohibido ejecutar `git` (por nombre y por basename de ruta resuelta).
- `arguments` debe ser un array de tokens; se rechazan metacaracteres típicos de shell por token.
- `subprocess.run(..., shell=False)` obligatorio.
"""

from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any


UNSAFE_TOKEN_RE = re.compile(r"[\n\r;|><`]")


def _emit(out: dict[str, Any]) -> None:
    sys.stdout.write(json.dumps(out, ensure_ascii=False))


def _fail(msg: str, exit_code: int = 1) -> None:
    _emit({"success": False, "exitCode": exit_code, "error": msg})
    sys.exit(1)


def _ok(stdout: str, stderr: str, exit_code: int) -> None:
    success = exit_code == 0
    out: dict[str, Any] = {
        "success": success,
        "exitCode": exit_code,
        "data": {"stdout": stdout, "stderr": stderr},
    }
    if not success:
        out["error"] = "command exited with non-zero status"
    _emit(out)
    sys.exit(0 if success else 1)


def _load_json() -> dict[str, Any]:
    try:
        raw = sys.stdin.read()
        return json.loads(raw) if raw.strip() else {}
    except json.JSONDecodeError as e:
        _fail(f"invalid JSON stdin: {e}")


def _assert_str(v: Any, field: str) -> str:
    if not isinstance(v, str):
        _fail(f"{field} must be a string")
    return v


def _assert_arguments(v: Any) -> list[str]:
    if not isinstance(v, list) or not all(isinstance(x, str) for x in v):
        _fail("arguments must be an array of strings")
    return v


def _assert_env_vars(v: Any) -> dict[str, str]:
    if v is None:
        return {}
    if not isinstance(v, dict):
        _fail("environment_vars must be an object (string->string)")
    out: dict[str, str] = {}
    for k, val in v.items():
        if not isinstance(k, str) or not isinstance(val, str):
            _fail("environment_vars must be an object (string->string)")
        out[k] = val
    return out


def _assert_safe_token(token: str, field: str) -> None:
    # Reglas anti-inyección (congeladas): prohibido ; && | > < ` $( ) y & suelto/embebido
    if UNSAFE_TOKEN_RE.search(token):
        _fail(f"{field} contains forbidden shell metacharacters")
    if "&&" in token or "$(" in token or "&" in token:
        _fail(f"{field} contains forbidden shell metacharacters")


def _resolve_working_dir(path_str: str) -> Path:
    p = Path(path_str)
    if not p.is_absolute():
        _fail("working_directory must be an absolute path")
    try:
        rp = p.resolve()
    except OSError as e:
        _fail(f"working_directory invalid: {e}")
    if not rp.is_dir():
        _fail("working_directory must exist and be a directory")
    return rp


def _allowlist() -> set[str]:
    # Lista blanca local mínima; Cerbero puede imponer otra capa antes de invocar.
    base = {"gh", "npm", "node", "python", "python3", "pwsh", "dotnet", "docker"}
    extra = os.environ.get("SDDIA_SHELL_EXECUTOR_ALLOWLIST", "").strip()
    if extra:
        for item in extra.split(","):
            it = item.strip()
            if it:
                base.add(it)
    return base


def _reject_if_git(executable: str) -> None:
    ex = executable.strip().strip("\"").strip("'")
    name = Path(ex).name.lower()
    if name in {"git", "git.exe"}:
        _fail("executable git is forbidden; route via git-manager")
    which = shutil.which(ex)
    if which and Path(which).name.lower() in {"git", "git.exe"}:
        _fail("executable resolves to git; route via git-manager")


def main() -> None:
    doc = _load_json()

    executable = _assert_str(doc.get("executable"), "executable").strip()
    arguments = _assert_arguments(doc.get("arguments"))
    working_directory = _assert_str(doc.get("working_directory"), "working_directory")
    env_vars = _assert_env_vars(doc.get("environment_vars"))

    if not executable:
        _fail("executable must be non-empty")

    _reject_if_git(executable)

    # Sanitización tokens
    _assert_safe_token(executable, "executable")
    for i, a in enumerate(arguments):
        _assert_safe_token(a, f"arguments[{i}]")

    # Whitelist simple (por nombre si no es ruta)
    allow = _allowlist()
    exe_name = Path(executable).name.lower()
    if Path(executable).is_absolute():
        # Si es ruta absoluta, se valida por basename.
        if exe_name not in allow and executable.lower() not in allow:
            _fail("executable is not allowlisted")
    else:
        if exe_name not in allow and executable.lower() not in allow:
            _fail("executable is not allowlisted")

    wd = _resolve_working_dir(working_directory)

    # Resolver ejecutable desde PATH si aplica
    resolved = shutil.which(executable) if not Path(executable).is_absolute() else executable
    if not resolved:
        _fail("executable not found on PATH")

    env = os.environ.copy()
    env.update(env_vars)

    proc = subprocess.run(
        [resolved, *arguments],
        cwd=str(wd),
        env=env,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        shell=False,
    )

    _ok(proc.stdout, proc.stderr, int(proc.returncode))


if __name__ == "__main__":
    main()

