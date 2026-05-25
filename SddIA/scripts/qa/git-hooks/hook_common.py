#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Utilidades compartidas para hooks Git SddIA (Ola A + Ola B)."""

from __future__ import annotations

import json
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import Any

if str(QA := Path(__file__).resolve().parents[1]) not in sys.path:
    sys.path.insert(0, str(QA))

from tmp_paths import cleanup_path, write_ephemeral_json

SCRIPT = Path(__file__).resolve()
REPO = SCRIPT.parents[4]
QA = REPO / "SddIA" / "scripts" / "qa"
CUMULO_PATH = REPO / "SddIA" / "core" / "cumulo.paths.json"
EXECUTE_PROCESS = QA / "execute-process.py"

BRANCH_PREFIXES = ("feat/", "fix/", "refactor/", "hotfix/")
MAIN_GUARD_MSG = (
    "Violación de Soberanía: main solo muta mediante el proceso accept-pr (PR merge). Push bloqueado."
)
HOOK_DELIVERY_CLOSE_ENV = "SDDIA_HOOK_DELIVERY_CLOSE"

GIT_HOOK_NAMES = frozenset(
    {
        "applypatch-msg",
        "pre-applypatch",
        "post-applypatch",
        "pre-commit",
        "pre-merge-commit",
        "prepare-commit-msg",
        "commit-msg",
        "post-commit",
        "pre-rebase",
        "post-checkout",
        "post-merge",
        "pre-push",
        "pre-receive",
        "update",
        "post-receive",
        "post-update",
        "push-to-checkout",
        "pre-auto-gc",
        "post-rewrite",
        "sendemail-validate",
        "fsmonitor-watchman",
        "p4-changelist",
        "p4-prepare-changelist",
        "p4-post-changelist",
        "p4-pre-submit",
        "post-index-change",
    }
)

INSTALLER_EXCLUDE_SUFFIXES = (".py", ".ps1", ".sh", ".md", ".json", ".txt")
INSTALLER_EXCLUDE_NAMES = frozenset({"install-hooks"})


def skip_hooks() -> bool:
    return os.environ.get("SDDIA_SKIP_HOOKS") == "1"


def in_delivery_close_cycle() -> bool:
    return os.environ.get(HOOK_DELIVERY_CLOSE_ENV) == "1"


def ref_to_branch(ref: str) -> str:
    ref = ref.strip()
    if ref.startswith("refs/heads/"):
        return ref[len("refs/heads/") :]
    return ref


def is_main_ref(ref: str) -> bool:
    branch = ref_to_branch(ref)
    return branch == "main"


def load_cumulo() -> dict[str, Any]:
    return json.loads(CUMULO_PATH.read_text(encoding="utf-8"))


def eda_bus_dirs() -> list[Path]:
    cumulo = load_cumulo()
    eda = cumulo.get("eda_bus") or {}
    dirs: list[Path] = []
    for key in ("pending", "processing", "processed"):
        rel = eda.get(key)
        if isinstance(rel, str) and rel.strip():
            p = REPO / rel.strip()
            if p.is_dir():
                dirs.append(p)
    return dirs


def branch_slug(branch_name: str) -> str:
    name = branch_name.strip()
    for prefix in BRANCH_PREFIXES:
        if name.startswith(prefix):
            return name[len(prefix) :]
    if "/" in name:
        return name.split("/", 1)[1]
    return name


def resolve_persist_ref(branch_name: str) -> str | None:
    slug = branch_slug(branch_name)
    if not slug:
        return None
    for docs_kind in ("features", "fixes"):
        candidate = REPO / "docs" / docs_kind / slug
        if candidate.is_dir():
            return f"docs/{docs_kind}/{slug}"
    return None


def scan_presented_for_branch(branch_name: str) -> bool:
    target = branch_name.strip()
    for bus_dir in eda_bus_dirs():
        for path in bus_dir.glob("*.json"):
            try:
                event = json.loads(path.read_text(encoding="utf-8"))
            except (json.JSONDecodeError, OSError):
                continue
            if event.get("event_type") != "PullRequest_Presented":
                continue
            payload = event.get("payload") or {}
            if payload.get("branch") == target:
                return True
    return False


def _gh_pr_state_for_branch(branch_name: str) -> str | None:
    try:
        proc = subprocess.run(
            ["gh", "pr", "view", branch_name, "--json", "state", "-q", ".state"],
            cwd=str(REPO),
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            check=False,
        )
    except OSError:
        return None
    if proc.returncode != 0:
        return None
    state = (proc.stdout or "").strip().upper()
    return state or None


def gh_pr_open_for_branch(branch_name: str) -> bool:
    return _gh_pr_state_for_branch(branch_name) == "OPEN"


def gh_pr_merged_for_branch(branch_name: str) -> bool:
    return _gh_pr_state_for_branch(branch_name) == "MERGED"


def should_skip_pre_push_present(branch_name: str) -> bool:
    state = _gh_pr_state_for_branch(branch_name)
    if state in ("OPEN", "MERGED"):
        return True
    if scan_presented_for_branch(branch_name):
        return True
    return False


def git_run(args: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["git", *args],
        cwd=str(REPO),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )


def git_config(key: str, default: str = "") -> str:
    proc = git_run(["config", "--get", key])
    if proc.returncode != 0:
        return default
    return (proc.stdout or "").strip() or default


def write_inputs_payload(prefix: str, payload: dict[str, Any]) -> Path:
    return write_ephemeral_json(REPO, prefix, payload)


def invoke_process(process_name: str, inputs: dict[str, Any]) -> int:
    payload_path = write_inputs_payload(f"hook-{process_name}", inputs)
    env = os.environ.copy()
    env[HOOK_DELIVERY_CLOSE_ENV] = "1"
    try:
        proc = subprocess.run(
            [sys.executable, str(EXECUTE_PROCESS), "--process", process_name, "--inputs-file", str(payload_path)],
            cwd=str(REPO),
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            check=False,
            env=env,
        )
        if proc.stdout:
            print(proc.stdout, file=sys.stderr, end="")
        if proc.stderr:
            print(proc.stderr, file=sys.stderr, end="")
        return proc.returncode
    finally:
        cleanup_path(payload_path)


def parse_pre_push_stdin(text: str) -> list[dict[str, str]]:
    refs: list[dict[str, str]] = []
    for line in text.splitlines():
        parts = line.split()
        if len(parts) < 4:
            continue
        refs.append(
            {
                "local_ref": parts[0],
                "local_sha": parts[1],
                "remote_ref": parts[2],
                "remote_sha": parts[3],
            }
        )
    return refs


def is_delete_push(remote_sha: str) -> bool:
    return bool(re.fullmatch(r"0+", remote_sha or ""))


def infer_merged_branch() -> str | None:
    proc = git_run(["rev-parse", "--verify", "HEAD^2"])
    if proc.returncode != 0:
        return None
    msg = git_run(["log", "-1", "--pretty=%B"])
    if msg.returncode == 0:
        m = re.search(r"Merge branch '([^']+)'", msg.stdout or "")
        if m:
            return m.group(1)
    name = git_run(["name-rev", "--name-only", "HEAD^2"])
    if name.returncode == 0:
        raw = (name.stdout or "").strip()
        if raw.startswith("remotes/"):
            parts = raw.split("/")
            if len(parts) >= 3:
                return "/".join(parts[2:])
        return raw.replace("~", "").replace("^", "")
    return None


def list_installable_hooks(source_dir: Path) -> list[Path]:
    hooks: list[Path] = []
    if not source_dir.is_dir():
        return hooks
    for entry in sorted(source_dir.iterdir()):
        if not entry.is_file():
            continue
        if entry.suffix.lower() in INSTALLER_EXCLUDE_SUFFIXES:
            continue
        if entry.stem in INSTALLER_EXCLUDE_NAMES:
            continue
        if entry.name not in GIT_HOOK_NAMES:
            continue
        hooks.append(entry)
    return hooks
