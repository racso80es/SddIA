#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Oráculo sensor DLT: detecta PRs remotos, ancla en IOTA y materializa PullRequest_Presented.

Uso:
  python SddIA/scripts/daemons/github_bridge_watcher.py
  python SddIA/scripts/daemons/github_bridge_watcher.py --once
  SDDIA_LAB_SIMULATE_REMOTE_PR=1 python SddIA/scripts/daemons/github_bridge_watcher.py --once
"""

from __future__ import annotations

import argparse
import json
import os
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any

_SCRIPT_DIR = Path(__file__).resolve().parent
_QA_DIR = _SCRIPT_DIR.parent / "qa"
if str(_QA_DIR) not in sys.path:
    sys.path.insert(0, str(_QA_DIR))

from dlt_bus_materializer import (
    build_bus_event,
    compose_pre_anchor_event,
    load_wallet_secret,
    materialize_to_bus,
    publish_with_retries,
    write_fallback_dead_letter,
)
from env_loader import load_hierarchical_env

DEFAULT_REPO = "racso80es/SddIA"
STATE_REL = ".SddIA/.dev/github_bridge_state.json"
SIMULATION_REL = ".SddIA/.dev/remote_pr_simulation.json"


def _repo_root() -> Path:
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace")


def _lab_simulate() -> bool:
    return os.environ.get("SDDIA_LAB_SIMULATE_REMOTE_PR", "").strip().lower() in (
        "1",
        "true",
        "yes",
    )


def _poll_seconds() -> int:
    raw = os.environ.get("SDDIA_GITHUB_BRIDGE_POLL_SECONDS", "30")
    try:
        return max(5, int(raw))
    except ValueError:
        return 30


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


def _github_request(url: str, token: str) -> Any:
    req = urllib.request.Request(
        url,
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {token}",
            "User-Agent": "sddia-github-bridge-watcher",
        },
    )
    with urllib.request.urlopen(req, timeout=30) as resp:
        return json.loads(resp.read().decode("utf-8"))


def _parse_repo_slug(slug: str) -> tuple[str, str]:
    if "/" not in slug:
        raise ValueError(f"repository inválido: {slug}")
    owner, name = slug.split("/", 1)
    return owner.strip(), name.strip()


def _pr_record_from_github(item: dict[str, Any], repository: str) -> dict[str, Any]:
    head = item.get("head") or {}
    branch = head.get("ref") if isinstance(head, dict) else None
    return {
        "repository": repository,
        "branch": branch or "",
        "pr_url": item.get("html_url") or "",
        "origin_agent": "jules",
        "github_number": item.get("number"),
    }


def _validate_pr_against_github(repo_slug: str, pr: dict[str, Any], token: str) -> bool:
    """Filtro A: contrastar intención contra REST GitHub."""
    pr_url = pr.get("pr_url") or ""
    number = pr.get("github_number")
    owner, name = _parse_repo_slug(repo_slug)
    if number is not None:
        url = f"https://api.github.com/repos/{owner}/{name}/pulls/{number}"
    elif isinstance(pr_url, str) and "/pull/" in pr_url:
        num = pr_url.rstrip("/").split("/")[-1]
        url = f"https://api.github.com/repos/{owner}/{name}/pulls/{num}"
    else:
        return False
    try:
        remote = _github_request(url, token)
    except (urllib.error.URLError, urllib.error.HTTPError, json.JSONDecodeError, TimeoutError):
        return False
    if not isinstance(remote, dict):
        return False
    head = remote.get("head") or {}
    remote_branch = head.get("ref") if isinstance(head, dict) else None
    return (
        remote.get("html_url") == pr.get("pr_url")
        and remote_branch == pr.get("branch")
    )


def fetch_open_prs(repo: Path, repository: str, token: str) -> list[dict[str, Any]]:
    owner, name = _parse_repo_slug(repository)
    url = f"https://api.github.com/repos/{owner}/{name}/pulls?state=open&per_page=30"
    try:
        data = _github_request(url, token)
    except (urllib.error.URLError, urllib.error.HTTPError, json.JSONDecodeError, TimeoutError) as e:
        print(f"[GITHUB-BRIDGE] Error polling GitHub: {e}", flush=True)
        return []
    if not isinstance(data, list):
        return []
    records: list[dict[str, Any]] = []
    for item in data:
        if not isinstance(item, dict):
            continue
        pr = _pr_record_from_github(item, repository)
        if pr.get("branch") and pr.get("pr_url"):
            if _validate_pr_against_github(repository, pr, token):
                records.append(pr)
            else:
                print(f"[GITHUB-BRIDGE] Filtro A: descartado PR corrupto {pr.get('pr_url')}", flush=True)
    return records


def fetch_lab_simulation(repo: Path) -> list[dict[str, Any]]:
    path = repo / SIMULATION_REL
    if not path.is_file():
        print(f"[GITHUB-BRIDGE] Lab: sin fixture {SIMULATION_REL}", flush=True)
        return []
    try:
        data = json.loads(path.read_text(encoding="utf-8-sig"))
    except json.JSONDecodeError:
        return []
    if isinstance(data, dict):
        return [data]
    if isinstance(data, list):
        return [x for x in data if isinstance(x, dict)]
    return []


def process_pr(repo: Path, pr: dict[str, Any], state: dict[str, Any]) -> bool:
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


def run_cycle(repo: Path) -> int:
    state = _load_state(repo)
    repository = os.environ.get("SDDIA_GITHUB_REPOSITORY", DEFAULT_REPO).strip()

    if _lab_simulate():
        candidates = fetch_lab_simulation(repo)
    else:
        token = os.environ.get("GITHUB_TOKEN", "").strip()
        if not token:
            print("[GITHUB-BRIDGE] GITHUB_TOKEN ausente; use SDDIA_LAB_SIMULATE_REMOTE_PR=1 en lab", flush=True)
            return 1
        candidates = fetch_open_prs(repo, repository, token)

    if not candidates:
        return 0

    handled = 0
    for pr in candidates:
        if process_pr(repo, pr, state):
            handled += 1
    return 0 if handled >= 0 else 1


def main() -> None:
    parser = argparse.ArgumentParser(description="github-bridge-watcher — oráculo sensor DLT")
    parser.add_argument("--once", action="store_true", help="Un solo ciclo y salir")
    args = parser.parse_args()

    repo = _repo_root()
    load_hierarchical_env(repo)

    if args.once:
        sys.exit(run_cycle(repo))

    print(f"[GITHUB-BRIDGE] Iniciado (poll={_poll_seconds()}s)", flush=True)
    while True:
        run_cycle(repo)
        time.sleep(_poll_seconds())


if __name__ == "__main__":
    main()
