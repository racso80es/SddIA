#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Utilidades compartidas para hooks Git SddIA (Ola A + Ola B)."""

from __future__ import annotations

import json
import os
import re
import subprocess
import sys
import uuid
from pathlib import Path
from typing import Any

SCRIPT = Path(__file__).resolve()
REPO = SCRIPT.parents[4]
QA = REPO / "SddIA" / "scripts" / "qa"
CUMULO_PATH = REPO / "SddIA" / "core" / "cumulo.paths.json"
EXECUTE_PROCESS = QA / "execute-process.py"
TMP_DIR = REPO / "tmp"

BRANCH_PREFIXES = ("feat/", "fix/", "refactor/", "hotfix/")
MAIN_GUARD_MSG = (
    "Violación de Soberanía: main solo muta mediante el proceso accept-pr (PR merge). Push bloqueado."
)

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
    candidate = REPO / "docs" / "features" / slug
    if candidate.is_dir():
        return f"docs/features/{slug}"
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


def gh_pr_open_for_branch(branch_name: str) -> bool:
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
        return False
    if proc.returncode != 0:
        return False
    state = (proc.stdout or "").strip().upper()
    return state == "OPEN"


def should_skip_pre_push_present(branch_name: str) -> bool:
    if gh_pr_open_for_branch(branch_name):
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
    TMP_DIR.mkdir(parents=True, exist_ok=True)
    path = TMP_DIR / f"{prefix}-{uuid.uuid4().hex[:12]}.json"
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return path


def invoke_process(process_name: str, inputs: dict[str, Any]) -> int:
    payload_path = write_inputs_payload(f"hook-{process_name}", inputs)
    proc = subprocess.run(
        [sys.executable, str(EXECUTE_PROCESS), "--process", process_name, "--inputs-file", str(payload_path)],
        cwd=str(REPO),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    if proc.stdout:
        print(proc.stdout, file=sys.stderr, end="")
    if proc.stderr:
        print(proc.stderr, file=sys.stderr, end="")
    return proc.returncode


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
