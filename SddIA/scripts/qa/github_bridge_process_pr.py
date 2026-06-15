# -*- coding: utf-8 -*-
"""Núcleo process_pr para github-bridge-watcher (delegado desde binario Rust)."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

_QA = Path(__file__).resolve().parent
if str(_QA) not in sys.path:
    sys.path.insert(0, str(_QA))

from dlt_bus_materializer import (
    build_bus_event,
    compose_pre_anchor_event,
    load_wallet_secret,
    materialize_to_bus,
    publish_with_retries,
    write_fallback_dead_letter,
)

STATE_REL = ".SddIA/.dev/github_bridge_state.json"


def _load_state(repo: Path) -> dict[str, Any]:
    path = repo / STATE_REL
    if not path.is_file():
        return {"processed_pr_urls": []}
    try:
        data = json.loads(path.read_text(encoding="utf-8-sig"))
    except json.JSONDecodeError:
        return {"processed_pr_urls": []}
    if not isinstance(data, dict):
        return {"processed_pr_urls": []}
    urls = data.get("processed_pr_urls")
    if not isinstance(urls, list):
        urls = []
    return {"processed_pr_urls": [u for u in urls if isinstance(u, str)]}


def _save_state(repo: Path, state: dict[str, Any]) -> None:
    path = repo / STATE_REL
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(state, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def process_pr(repo: Path, pr: dict[str, Any], state: dict[str, Any] | None = None) -> bool:
    """Materializa un PR remoto (IOTA + bus). Retorna True si se procesó."""
    if state is None:
        state = _load_state(repo)
    pr_url = pr.get("pr_url") or ""
    if not pr_url:
        return False
    processed: list[str] = state.setdefault("processed_pr_urls", [])
    if pr_url in processed:
        return False

    print(f"[GITHUB-BRIDGE] Procesando PR remoto: {pr_url}", flush=True)
    load_wallet_secret(repo)
    pre_event = compose_pre_anchor_event(pr)
    digest, object_id, err = publish_with_retries(repo, pre_event)
    if not digest:
        dl = write_fallback_dead_letter(repo, pr, err or "iota publish failed")
        print(f"[GITHUB-BRIDGE] Fallback dead-letter: {dl.relative_to(repo)}", flush=True)
        return False

    bus_event = build_bus_event(pre_event, digest, object_id)
    target = materialize_to_bus(repo, bus_event)
    if target:
        print(f"[GITHUB-BRIDGE] Materializado: {target.relative_to(repo)}", flush=True)
    else:
        print(f"[GITHUB-BRIDGE] Idempotente: evento {digest} ya en pending", flush=True)

    processed.append(pr_url)
    _save_state(repo, state)
    return True


def main() -> None:
    raw = sys.stdin.read()
    try:
        body = json.loads(raw) if raw.strip() else {}
    except json.JSONDecodeError:
        print(json.dumps({"handled": False, "error": "invalid-json"}))
        sys.exit(1)
    repo_raw = body.get("repository_path") or "."
    pr = body.get("pr")
    if not isinstance(pr, dict):
        print(json.dumps({"handled": False, "error": "pr-required"}))
        sys.exit(1)
    repo = Path(repo_raw).resolve()
    handled = process_pr(repo, pr)
    print(json.dumps({"handled": handled, "success": True}))


if __name__ == "__main__":
    main()
