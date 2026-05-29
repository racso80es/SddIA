#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Centinela Capa 0: long polling Telegram → execute-process telegram-gateway."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Any

_SCRIPT_DIR = Path(__file__).resolve().parent
_QA_DIR = _SCRIPT_DIR.parent / "qa"
if str(_QA_DIR) not in sys.path:
    sys.path.insert(0, str(_QA_DIR))

from env_loader import load_hierarchical_env

POLL_TIMEOUT = 30
STATE_REL = ".SddIA/.state/telegram_last_id"
_LEGACY_STATE_REL = ".SddIA/daemons/state/telegram-watcher.json"


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace")


def _state_path(repo: Path) -> Path:
    return repo / STATE_REL


def _load_last_update_id(repo: Path) -> int:
    path = _state_path(repo)
    if path.is_file():
        try:
            raw = path.read_text(encoding="utf-8").strip()
            if raw.isdigit():
                return max(0, int(raw))
            body = json.loads(raw)
            if isinstance(body, dict) and isinstance(body.get("last_update_id"), int):
                return max(0, body["last_update_id"])
        except (OSError, json.JSONDecodeError, ValueError):
            pass
    legacy = repo / _LEGACY_STATE_REL
    if legacy.is_file():
        try:
            body = json.loads(legacy.read_text(encoding="utf-8"))
            if isinstance(body, dict) and isinstance(body.get("last_update_id"), int):
                return max(0, body["last_update_id"])
        except (OSError, json.JSONDecodeError):
            pass
    return 0


def _save_last_update_id(repo: Path, last_update_id: int) -> None:
    path = _state_path(repo)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(str(int(last_update_id)), encoding="utf-8")


def _require_env() -> tuple[str, str]:
    token = os.environ.get("TELEGRAM_BOT_TOKEN", "").strip()
    chat_id = os.environ.get("TELEGRAM_ALLOWED_CHAT_ID", "").strip()
    if not token or not chat_id:
        print("[telegram-watcher] TELEGRAM_BOT_TOKEN / TELEGRAM_ALLOWED_CHAT_ID no configurados", file=sys.stderr)
        sys.exit(2)
    return token, chat_id


def _get_updates(token: str, offset: int) -> list[dict[str, Any]]:
    params = urllib.parse.urlencode({"timeout": POLL_TIMEOUT, "offset": offset})
    url = f"https://api.telegram.org/bot{token}/getUpdates?{params}"
    req = urllib.request.Request(url, method="GET")
    try:
        with urllib.request.urlopen(req, timeout=POLL_TIMEOUT + 10) as resp:
            raw = resp.read().decode("utf-8", errors="replace")
    except urllib.error.URLError as exc:
        print(f"[telegram-watcher] getUpdates error: {exc}", file=sys.stderr, flush=True)
        return []
    try:
        body = json.loads(raw)
    except json.JSONDecodeError:
        return []
    if not body.get("ok"):
        return []
    result = body.get("result")
    return result if isinstance(result, list) else []


def _extract_text(update: dict[str, Any]) -> str | None:
    msg = update.get("message") or update.get("edited_message")
    if not isinstance(msg, dict):
        return None
    text = msg.get("text")
    if isinstance(text, str) and text.strip():
        return text
    return None


def _chat_id(update: dict[str, Any]) -> str | None:
    msg = update.get("message") or update.get("edited_message")
    if not isinstance(msg, dict):
        return None
    chat = msg.get("chat")
    if not isinstance(chat, dict):
        return None
    cid = chat.get("id")
    if cid is None:
        return None
    return str(cid)


def _invoke_gateway(repo: Path, text: str, *, dry_run: bool) -> int:
    if dry_run:
        return 0
    runner = repo / "SddIA" / "scripts" / "qa" / "execute-process.py"
    payload = json.dumps({"text": text}, ensure_ascii=False)
    proc = subprocess.run(
        [sys.executable, str(runner), "--process", "telegram-gateway", "--inputs", payload],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )
    if proc.returncode != 0 and proc.stderr:
        print(proc.stderr.strip(), file=sys.stderr, flush=True)
    return proc.returncode


def _process_updates(
    repo: Path,
    updates: list[dict[str, Any]],
    allowed_chat: str,
    *,
    dry_run: bool,
) -> int:
    max_id = _load_last_update_id(repo)
    for upd in updates:
        if not isinstance(upd, dict):
            continue
        uid = upd.get("update_id")
        if isinstance(uid, int):
            max_id = max(max_id, uid)
        chat = _chat_id(upd)
        if chat != allowed_chat:
            continue
        text = _extract_text(upd)
        if text is None:
            continue
        _invoke_gateway(repo, text, dry_run=dry_run)
    if updates and max_id > 0:
        _save_last_update_id(repo, max_id)
    return max_id


def run_once(repo: Path, *, dry_run: bool = False) -> None:
    token, allowed = _require_env()
    last = _load_last_update_id(repo)
    offset = last + 1 if last else 0
    updates = _get_updates(token, offset)
    _process_updates(repo, updates, allowed, dry_run=dry_run)


def run_loop(repo: Path, *, dry_run: bool = False) -> None:
    _require_env()
    print("[telegram-watcher] bucle iniciado", flush=True)
    while True:
        run_once(repo, dry_run=dry_run)
        time.sleep(1)


def main() -> None:
    parser = argparse.ArgumentParser(description="telegram-watcher — Capa 0 Telegram")
    parser.add_argument("--once", action="store_true", help="Un ciclo de polling")
    parser.add_argument("--dry-run", action="store_true", help="No invocar execute-process")
    args = parser.parse_args()
    repo = _repo_root()
    load_hierarchical_env(repo)
    if args.once:
        run_once(repo, dry_run=args.dry_run)
    else:
        run_loop(repo, dry_run=args.dry_run)


if __name__ == "__main__":
    main()
