# -*- coding: utf-8 -*-
"""Invocación tool send-telegram-notification y plantillas desde eventos EDA."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Any

from capsule_resolve import invoke_tool_capsule_json

TOOL_NAME = "send-telegram-notification"


def _limbo_script(repo: Path) -> Path:
    return (
        repo
        / "SddIA"
        / "scripts"
        / "limbo"
        / "tools"
        / "send-telegram-notification"
        / "main.py"
    )


def build_telegram_message_from_event(event: dict[str, Any]) -> str | None:
    event_type = event.get("event_type")
    payload = event.get("payload")
    if not isinstance(payload, dict):
        payload = {}
    if event_type == "PullRequest_Presented":
        branch = payload.get("branch") or "?"
        lines = [f"PR presentado: {branch}"]
        pr_url = payload.get("pr_url")
        if isinstance(pr_url, str) and pr_url.strip():
            lines.append(pr_url.strip())
        return "\n".join(lines)
    if event_type == "System_Fracture_Detected":
        proc = payload.get("process_name") or "?"
        trace = payload.get("trace_hash")
        if not isinstance(trace, str) or not trace.strip():
            err = payload.get("error_trace")
            if isinstance(err, str) and err.strip():
                trace = err.strip()[:120]
            else:
                trace = "sin-traza"
        return f"Fractura detectada: {proc}\n{trace}"
    return None


def _parse_capsule_stdout(stdout: str) -> dict[str, Any]:
    line = (stdout or "").strip().splitlines()[-1] if stdout else ""
    if not line:
        return {}
    try:
        parsed = json.loads(line)
        if isinstance(parsed, dict):
            return parsed
    except json.JSONDecodeError:
        return {"parse_error": line[:200]}
    return {}


def _invoke_limbo_python(
    repo: Path,
    script: Path,
    req: dict[str, Any],
) -> tuple[bool, dict[str, Any]]:
    proc = subprocess.run(
        [sys.executable, str(script)],
        input=json.dumps(req, ensure_ascii=False),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )
    body = _parse_capsule_stdout(proc.stdout or "")
    ok = bool(body.get("success")) and proc.returncode == 0
    if not ok and not body.get("error"):
        body["error"] = (proc.stderr or "send-telegram-notification failed").strip()
    return ok, body


def invoke_send_telegram_notification(
    repo: Path,
    message: str,
    *,
    parse_mode: str | None = "MarkdownV2",
) -> tuple[bool, dict[str, Any]]:
    req: dict[str, Any] = {"message": message}
    if parse_mode is not None:
        req["parse_mode"] = parse_mode

    try:
        _rc, body = invoke_tool_capsule_json(
            repo,
            TOOL_NAME,
            req,
            prefer_wasm=False,
        )
        if isinstance(body, dict) and body.get("success"):
            return True, body
    except FileNotFoundError:
        pass

    script = _limbo_script(repo)
    if not script.is_file():
        return False, {
            "error": (
                f"cápsula no encontrada: {TOOL_NAME} "
                f"(Rust SSOT ni limbo {script})"
            )
        }
    return _invoke_limbo_python(repo, script, req)
