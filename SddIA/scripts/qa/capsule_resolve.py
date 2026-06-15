# -*- coding: utf-8 -*-
"""Resolución e invocación de cápsulas skill/tool Rust (WASI / nativo) — Kaizen Ola 1/2."""

from __future__ import annotations

import json
import os
import shutil
import subprocess
from pathlib import Path
from typing import Any


def sddia_target_dir(repo: Path) -> Path:
    return repo / "SddIA" / "target"


def resolve_capsule_wasm(repo: Path, name: str) -> Path | None:
    base = sddia_target_dir(repo) / "wasm32-wasip1"
    for profile in ("release", "debug"):
        path = base / profile / f"{name}.wasm"
        if path.is_file():
            return path
    return None


def resolve_capsule_native(repo: Path, name: str) -> Path | None:
    base = sddia_target_dir(repo)
    for profile in ("release", "debug"):
        path = base / profile / name
        if path.is_file():
            return path
    return None


def _resolve_capsule(
    repo: Path,
    name: str,
    *,
    prefer_wasm: bool,
    entity_label: str,
) -> tuple[str, Path]:
    wasm = resolve_capsule_wasm(repo, name)
    native = resolve_capsule_native(repo, name)
    has_wasmtime = shutil.which("wasmtime") is not None

    if prefer_wasm and wasm is not None and has_wasmtime:
        return ("wasm", wasm)
    if native is not None:
        return ("native", native)
    if wasm is not None:
        return ("wasm", wasm)
    raise FileNotFoundError(
        f"cápsula {entity_label} '{name}' no encontrada bajo SddIA/target "
        f"(wasm32-wasip1/release|debug o release|debug nativo). "
        "Run: cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo build "
        "&& cargo build --target wasm32-wasip1"
    )


def resolve_skill_wasm(repo: Path, name: str) -> Path | None:
    return resolve_capsule_wasm(repo, name)


def resolve_skill_native(repo: Path, name: str) -> Path | None:
    return resolve_capsule_native(repo, name)


def resolve_skill_capsule(
    repo: Path,
    name: str,
    *,
    prefer_wasm: bool = True,
) -> tuple[str, Path]:
    return _resolve_capsule(repo, name, prefer_wasm=prefer_wasm, entity_label="skill")


def resolve_tool_wasm(repo: Path, name: str) -> Path | None:
    return resolve_capsule_wasm(repo, name)


def resolve_tool_native(repo: Path, name: str) -> Path | None:
    return resolve_capsule_native(repo, name)


def resolve_tool_capsule(
    repo: Path,
    name: str,
    *,
    prefer_wasm: bool = True,
) -> tuple[str, Path]:
    return _resolve_capsule(repo, name, prefer_wasm=prefer_wasm, entity_label="tool")


def parse_capsule_stdout(stdout: str) -> dict[str, Any]:
    line = (stdout or "").strip()
    if not line:
        return {}
    return json.loads(line.splitlines()[-1])


def parse_skill_stdout(stdout: str) -> dict[str, Any]:
    return parse_capsule_stdout(stdout)


def parse_tool_stdout(stdout: str) -> dict[str, Any]:
    return parse_capsule_stdout(stdout)


_OFFLINE_GIT_MARKERS = (
    "could not resolve host",
    "connection refused",
    "network is unreachable",
    "unable to access",
    "authentication failed",
    "failed to connect",
    "no such host is known",
    "name or service not known",
    "repository not found",
    "terminal prompts disabled",
    "could not read username",
    "could not read password",
)


def _is_offline_git_data(data: dict[str, Any]) -> bool:
    if not isinstance(data, dict):
        return False
    blob = "\n".join(
        str(data.get(k) or "")
        for k in ("gitStderr", "gitStdout", "errorSummary", "error")
    ).lower()
    return any(marker in blob for marker in _OFFLINE_GIT_MARKERS)


def unwrap_git_manager_body(body: dict[str, Any]) -> dict[str, Any]:
    inner = body.get("result")
    if isinstance(inner, dict) and ("data" in inner or inner.get("success") is not None):
        body = inner
    data = body.get("data") or {}
    if body.get("success"):
        return data if isinstance(data, dict) else {}
    if isinstance(data, dict) and data.get("offline"):
        return data
    offline_probe = dict(data) if isinstance(data, dict) else {}
    if body.get("error"):
        offline_probe["error"] = body.get("error")
    if _is_offline_git_data(offline_probe):
        summary = (data.get("errorSummary") or data.get("gitStderr") or "remote unavailable — local mode")
        return {
            "offline": True,
            "exitCode": 0,
            "gitStdout": data.get("gitStdout", ""),
            "gitStderr": data.get("gitStderr", ""),
            "errorSummary": summary if isinstance(summary, str) else str(summary),
        }
    raise RuntimeError(body.get("error") or "git-manager failed")


def unwrap_shell_executor_body(body: dict[str, Any]) -> dict[str, Any]:
    inner = body.get("result")
    if isinstance(inner, dict) and ("data" in inner or inner.get("success") is not None):
        body = inner
    data = body.get("data") or {}
    if body.get("success"):
        return data if isinstance(data, dict) else {}
    err = str(body.get("error") or "shell-executor failed")
    if data:
        enriched = dict(data)
        enriched["_shell_exit_code"] = body.get("exitCode")
        enriched["_shell_error"] = err
        return enriched
    raise RuntimeError(err)


def unwrap_bus_operator_body(body: dict[str, Any]) -> dict[str, Any]:
    if not body.get("success"):
        raise RuntimeError(body.get("error") or "bus-operator failed")
    result = body.get("result")
    if isinstance(result, dict):
        return result
    return {}


def unwrap_crypto_result(body: dict[str, Any]) -> Any:
    if not body.get("success"):
        raise RuntimeError(body.get("error") or "cryptography-manager failed")
    inner = body.get("result")
    if isinstance(inner, dict) and "result" in inner:
        return inner["result"]
    return inner


def invoke_capsule_subprocess(
    repo: Path,
    capsule_kind: str,
    capsule_path: Path,
    stdin_payload: str,
    *,
    wasm_dir: str = ".",
    extra_env: dict[str, str] | None = None,
) -> tuple[str, str, int]:
    env = os.environ.copy()
    if extra_env:
        env.update(extra_env)
    if capsule_kind == "wasm":
        if shutil.which("wasmtime") is None:
            raise RuntimeError("wasmtime not in PATH; install wasmtime to run WASI capsules")
        proc = subprocess.run(
            ["wasmtime", "run", f"--dir={wasm_dir}", str(capsule_path)],
            input=stdin_payload,
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            cwd=str(repo),
            check=False,
            env=env,
        )
    else:
        proc = subprocess.run(
            [str(capsule_path)],
            input=stdin_payload,
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            cwd=str(repo),
            check=False,
            env=env,
        )
    return (proc.stdout or "").strip(), proc.stderr or "", proc.returncode


def invoke_skill_subprocess(
    repo: Path,
    capsule_kind: str,
    capsule_path: Path,
    stdin_payload: str,
    *,
    wasm_dir: str = ".",
    extra_env: dict[str, str] | None = None,
) -> tuple[str, str, int]:
    return invoke_capsule_subprocess(
        repo,
        capsule_kind,
        capsule_path,
        stdin_payload,
        wasm_dir=wasm_dir,
        extra_env=extra_env,
    )


invoke_tool_subprocess = invoke_skill_subprocess


def unwrap_tool_envelope(body: dict[str, Any]) -> dict[str, Any]:
    """Aplana envelopes anidados cuando la tool pasó un envelope completo a emit_success."""
    if not isinstance(body, dict):
        return body
    inner = body.get("result")
    if not isinstance(inner, dict):
        return body
    if not any(
        key in inner
        for key in ("emitted", "telemetry_receipt", "event", "message", "error")
    ):
        return body
    merged = dict(inner)
    nested = inner.get("result")
    if isinstance(nested, dict):
        merged["result"] = nested
    return merged


_WASM_NATIVE_FALLBACK_MARKERS = (
    "function not implemented",
    "operation not supported",
    "failed to get current exe",
    "read-only",
    "no se pudo marcar read-only",
)


def invoke_tool_capsule_json(
    repo: Path,
    name: str,
    payload: dict[str, Any],
    *,
    prefer_wasm: bool = True,
    wasm_dir: str = ".",
    extra_env: dict[str, str] | None = None,
) -> tuple[int, dict[str, Any]]:
    kind, path = resolve_tool_capsule(repo, name, prefer_wasm=prefer_wasm)
    stdout, _stderr, rc = invoke_tool_subprocess(
        repo,
        kind,
        path,
        json.dumps(payload, ensure_ascii=False),
        wasm_dir=wasm_dir,
        extra_env=extra_env,
    )
    body: dict[str, Any] = {}
    try:
        parsed = parse_tool_stdout(stdout)
        if isinstance(parsed, dict):
            body = unwrap_tool_envelope(parsed)
    except json.JSONDecodeError:
        body = {"parse_error": (stdout or "")[:200]}

    exit_code = body.get("exitCode")
    if isinstance(exit_code, int):
        rc = exit_code
    elif body.get("success") is False and rc == 0:
        rc = 1

    if (
        prefer_wasm
        and kind == "wasm"
        and resolve_tool_native(repo, name) is not None
        and (not body.get("success") or rc != 0)
    ):
        err_blob = str(body.get("error") or body.get("message") or "").lower()
        if any(marker in err_blob for marker in _WASM_NATIVE_FALLBACK_MARKERS):
            return invoke_tool_capsule_json(
                repo,
                name,
                payload,
                prefer_wasm=False,
                wasm_dir=wasm_dir,
                extra_env=extra_env,
            )

    if body.get("success") is None and rc == 0:
        body["success"] = True
    elif body.get("success") is None and rc != 0:
        body["success"] = False

    return rc, body


def resolve_daemon_native(repo: Path, name: str) -> Path | None:
    return resolve_capsule_native(repo, name)


def resolve_daemon_capsule(repo: Path, name: str) -> Path:
    native = resolve_daemon_native(repo, name)
    if native is not None:
        return native
    raise FileNotFoundError(
        f"cápsula daemon '{name}' no encontrada bajo SddIA/target "
        f"(release|debug nativo). "
        "Run: cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo build -p {name}"
    )


def git_manager_should_fallback_native(stderr: str, stdout: str) -> bool:
    blob = f"{stderr}\n{stdout}".lower()
    return (
        "failed to execute git" in blob
        or "operation not supported" in blob
    )


def shell_executor_should_fallback_native(stderr: str, stdout: str, error: str | None) -> bool:
    blob = f"{stderr}\n{stdout}\n{error or ''}".lower()
    markers = (
        "working_directory invalid",
        "executable not found on path",
        "failed to execute",
        "no such file or directory (os error 44)",
    )
    return any(marker in blob for marker in markers)
