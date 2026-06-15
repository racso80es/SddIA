#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Cápsula shell-executor: binarios allowlisted; I/O JSON por stdin/stdout (norma congelada)."""

from __future__ import annotations

import json
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any

UNSAFE_TOKEN = re.compile(r"[\n\r;|&$`<>]")
DEFAULT_ALLOWLIST = frozenset(
    {"gh", "npm", "node", "python", "python3", "pwsh", "dotnet", "docker"}
)


def _emit(out: dict[str, Any]) -> None:
    sys.stdout.write(json.dumps(out, ensure_ascii=False))


def _fail(msg: str) -> None:
    _emit({"success": False, "exitCode": 1, "error": msg})
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


def _assert_safe_token(token: str, field: str) -> None:
    if UNSAFE_TOKEN.search(token) or "&&" in token or "$(" in token:
        _fail(f"{field} contains forbidden shell metacharacters")


def _allowlist() -> set[str]:
    base = set(DEFAULT_ALLOWLIST)
    extra = os.environ.get("SDDIA_SHELL_EXECUTOR_ALLOWLIST", "").strip()
    if extra:
        for item in extra.split(","):
            it = item.strip()
            if it:
                base.add(it)
    return base


def _reject_if_git(executable: str) -> None:
    ex = executable.strip().strip('"').strip("'")
    name = Path(ex).name.lower()
    if name in ("git", "git.exe"):
        _fail("executable git is forbidden; route via git-manager")


def _resolve_working_dir(path_str: str) -> Path:
    if not isinstance(path_str, str) or not path_str.strip():
        _fail("working_directory must be a non-empty string")
    p = Path(path_str)
    if not p.is_absolute():
        _fail("working_directory must be an absolute path")
    try:
        p = p.resolve()
    except OSError as e:
        _fail(f"working_directory invalid: {e}")
    if not p.is_dir():
        _fail("working_directory must exist and be a directory")
    return p


def main() -> None:
    try:
        raw = sys.stdin.read()
        doc = json.loads(raw) if raw.strip() else {}
    except json.JSONDecodeError as e:
        _fail(f"invalid JSON stdin: {e}")

    executable = doc.get("executable")
    arguments = doc.get("arguments")
    working_directory = doc.get("working_directory")
    env_vars = doc.get("environment_vars")

    if not isinstance(executable, str) or not executable.strip():
        _fail("executable must be a non-empty string")
    executable = executable.strip()
    if not isinstance(arguments, list):
        _fail("arguments must be an array of strings")
    if not isinstance(working_directory, str):
        _fail("working_directory must be a string")
    if env_vars is None:
        env_vars = {}
    elif not isinstance(env_vars, dict):
        _fail("environment_vars must be an object (string->string)")

    _reject_if_git(executable)
    _assert_safe_token(executable, "executable")

    args_vec: list[str] = []
    for i, arg in enumerate(arguments):
        if not isinstance(arg, str):
            _fail("arguments must be an array of strings")
        _assert_safe_token(arg, f"arguments[{i}]")
        args_vec.append(arg)

    allow = _allowlist()
    exe_name = Path(executable).name.lower()
    if exe_name not in allow and executable.lower() not in allow:
        _fail("executable is not allowlisted")

    wd = _resolve_working_dir(working_directory)
    env = os.environ.copy()
    for key, value in env_vars.items():
        if not isinstance(key, str) or not isinstance(value, str):
            _fail("environment_vars must be an object (string->string)")
        env[key] = value

    try:
        proc = subprocess.run(
            [executable, *args_vec],
            cwd=str(wd),
            env=env,
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            shell=False,
        )
    except FileNotFoundError:
        _fail("executable not found on PATH")

    _ok(proc.stdout or "", proc.stderr or "", proc.returncode)


if __name__ == "__main__":
    main()
