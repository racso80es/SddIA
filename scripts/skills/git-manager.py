#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Cápsula git-manager: solo binario git nativo; I/O JSON por stdin/stdout (norma congelada)."""

from __future__ import annotations

import json
import re
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any, Mapping

ALLOWED_OPS = frozenset(
    {
        "status",
        "checkout",
        "commit",
        "push",
        "pull",
        "fetch",
        "branch_list",
        "get_last_commit",
        "merge",
        "delete_branch",
    }
)
UNSAFE_TOKEN = re.compile(r"[\n\r;|&$`<>()]")
COMMIT_HASH_RE = re.compile(r"^[0-9a-fA-F]{40}$")


def _emit(out: dict[str, Any]) -> None:
    sys.stdout.write(json.dumps(out, ensure_ascii=False))


def _fail(msg: str) -> None:
    _emit({"success": False, "exitCode": 1, "error": msg})
    sys.exit(1)


def _ok(data: dict[str, Any], git_exit: int) -> None:
    success = git_exit == 0
    out: dict[str, Any] = {
        "success": success,
        "exitCode": git_exit,
        "data": data,
    }
    if not success:
        out["error"] = data.get("errorSummary") or "git exited with non-zero status"
    _emit(out)
    sys.exit(0 if success else 1)


def _parse_commit_hash(raw: str, field: str) -> str:
    value = raw.strip()
    if not COMMIT_HASH_RE.fullmatch(value):
        _fail(f"{field} is not a valid 40-character hexadecimal commit hash")
    return value.lower()


def _assert_safe_token(value: str, field: str) -> None:
    if not isinstance(value, str):
        _fail(f"{field} must be a string")
    if UNSAFE_TOKEN.search(value):
        _fail(f"{field} contains forbidden shell-metacharacter sequences")


def _resolve_repo(path_str: str) -> Path:
    if not isinstance(path_str, str) or not path_str.strip():
        _fail("repository_path must be a non-empty string")
    repo = Path(path_str)
    try:
        repo = repo.resolve()
    except OSError as e:
        _fail(f"repository_path invalid: {e}")
    if not repo.is_dir():
        _fail("repository_path must be an existing directory")
    return repo


def _git_exe() -> str:
    exe = shutil.which("git")
    if not exe:
        _fail("git executable not found on PATH")
    return exe


def _run_git(repo: Path, git: str, args: list[str]) -> subprocess.CompletedProcess[str]:
    argv = [git, "-C", str(repo), *args]
    return subprocess.run(
        argv,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        shell=False,
    )


def _verify_git_repo(repo: Path, git: str) -> None:
    p = _run_git(repo, git, ["rev-parse", "--is-inside-work-tree"])
    if p.returncode != 0 or p.stdout.strip() != "true":
        _fail("repository_path is not a git work tree")


def _payload_exact(payload: Mapping[str, Any], op: str, keys: frozenset[str]) -> None:
    if set(payload.keys()) != keys:
        _fail(
            f"operation_payload_json for {op} must have exactly keys {sorted(keys)}, "
            f"got {sorted(payload.keys())}"
        )


def _rel_path_under_repo(repo: Path, rel_or_abs: str) -> str:
    _assert_safe_token(rel_or_abs, "files[]")
    p = Path(rel_or_abs)
    if p.is_absolute():
        cand = p.resolve()
    else:
        cand = (repo / p).resolve()
    try:
        cand.relative_to(repo.resolve())
    except ValueError:
        _fail("file path escapes repository root")
    try:
        return str(cand.relative_to(repo.resolve()))
    except ValueError:
        _fail("file path could not be made relative to repository")


def _handle(
    op: str, repo: Path, git: str, payload: Mapping[str, Any]
) -> tuple[dict[str, Any], int]:
    if op == "status":
        _payload_exact(payload, op, frozenset())
        proc = _run_git(repo, git, ["status", "--porcelain=v1"])
        return (
            {
                "gitStdout": proc.stdout,
                "gitStderr": proc.stderr,
                "errorSummary": proc.stderr.strip() or None,
            },
            proc.returncode,
        )

    if op == "branch_list":
        _payload_exact(payload, op, frozenset())
        proc = _run_git(repo, git, ["branch", "-a", "-v", "--no-abbrev"])
        lines = [ln for ln in proc.stdout.splitlines() if ln.strip()]
        return (
            {
                "gitStdout": proc.stdout,
                "gitStderr": proc.stderr,
                "branches": lines,
                "errorSummary": proc.stderr.strip() or None,
            },
            proc.returncode,
        )

    if op == "get_last_commit":
        _payload_exact(payload, op, frozenset({"ref"}))
        ref = payload["ref"]
        if not isinstance(ref, str) or not ref.strip():
            _fail("ref must be a non-empty string")
        _assert_safe_token(ref, "ref")
        proc = _run_git(repo, git, ["rev-parse", ref])
        if proc.returncode != 0:
            return (
                {
                    "gitStdout": proc.stdout,
                    "gitStderr": proc.stderr,
                    "errorSummary": proc.stderr.strip() or None,
                },
                proc.returncode,
            )
        commit_hash = _parse_commit_hash(proc.stdout, "commitHash")
        return (
            {
                "gitStdout": proc.stdout,
                "gitStderr": proc.stderr,
                "commitHash": commit_hash,
                "errorSummary": proc.stderr.strip() or None,
            },
            proc.returncode,
        )

    if op == "merge":
        _payload_exact(payload, op, frozenset({"branch_name", "no_ff"}))
        branch = payload["branch_name"]
        no_ff = payload["no_ff"]
        if not isinstance(branch, str) or not branch:
            _fail("branch_name must be a non-empty string")
        if not isinstance(no_ff, bool):
            _fail("no_ff must be boolean")
        _assert_safe_token(branch, "branch_name")
        merge_args = ["merge"]
        if no_ff:
            merge_args.append("--no-ff")
        merge_args.append(branch)
        proc = _run_git(repo, git, merge_args)
        if proc.returncode != 0:
            return (
                {
                    "gitStdout": proc.stdout,
                    "gitStderr": proc.stderr,
                    "errorSummary": proc.stderr.strip() or None,
                },
                proc.returncode,
            )
        rev = _run_git(repo, git, ["rev-parse", "HEAD"])
        if rev.returncode != 0:
            return (
                {
                    "gitStdout": proc.stdout + rev.stdout,
                    "gitStderr": proc.stderr + rev.stderr,
                    "errorSummary": rev.stderr.strip() or "rev-parse HEAD failed after merge",
                },
                rev.returncode,
            )
        merge_commit_hash = _parse_commit_hash(rev.stdout, "mergeCommitHash")
        return (
            {
                "gitStdout": proc.stdout + rev.stdout,
                "gitStderr": proc.stderr + rev.stderr,
                "mergeCommitHash": merge_commit_hash,
                "errorSummary": proc.stderr.strip() or None,
            },
            proc.returncode,
        )

    if op == "delete_branch":
        _payload_exact(payload, op, frozenset({"branch_name", "remote", "force"}))
        branch = payload["branch_name"]
        remote = payload["remote"]
        force = payload["force"]
        if not isinstance(branch, str) or not branch:
            _fail("branch_name must be a non-empty string")
        if not isinstance(remote, bool):
            _fail("remote must be boolean")
        if not isinstance(force, bool):
            _fail("force must be boolean")
        _assert_safe_token(branch, "branch_name")
        if remote:
            proc = _run_git(repo, git, ["push", "origin", "--delete", branch])
        else:
            flag = "-D" if force else "-d"
            proc = _run_git(repo, git, ["branch", flag, branch])
        return (
            {
                "gitStdout": proc.stdout,
                "gitStderr": proc.stderr,
                "errorSummary": proc.stderr.strip() or None,
            },
            proc.returncode,
        )

    if op == "checkout":
        _payload_exact(payload, op, frozenset({"branch_name", "create_if_not_exists"}))
        branch = payload["branch_name"]
        create = payload["create_if_not_exists"]
        if not isinstance(branch, str) or not branch:
            _fail("branch_name must be a non-empty string")
        if not isinstance(create, bool):
            _fail("create_if_not_exists must be boolean")
        _assert_safe_token(branch, "branch_name")
        if create:
            proc = _run_git(repo, git, ["checkout", "-b", branch])
        else:
            proc = _run_git(repo, git, ["checkout", branch])
        return (
            {
                "gitStdout": proc.stdout,
                "gitStderr": proc.stderr,
                "errorSummary": proc.stderr.strip() or None,
            },
            proc.returncode,
        )

    if op == "commit":
        _payload_exact(payload, op, frozenset({"message", "files"}))
        message = payload["message"]
        files = payload["files"]
        if not isinstance(message, str) or not message.strip():
            _fail("message must be a non-empty string")
        if not isinstance(files, list) or not all(isinstance(x, str) for x in files):
            _fail("files must be an array of strings")
        for f in files:
            rel = _rel_path_under_repo(repo, f)
            addp = _run_git(repo, git, ["add", "--", rel])
            if addp.returncode != 0:
                return (
                    {
                        "gitStdout": addp.stdout,
                        "gitStderr": addp.stderr,
                        "errorSummary": f"git add failed: {addp.stderr.strip()}",
                    },
                    addp.returncode,
                )
        proc = _run_git(repo, git, ["commit", "-m", message])
        return (
            {
                "gitStdout": proc.stdout,
                "gitStderr": proc.stderr,
                "errorSummary": proc.stderr.strip() or None,
            },
            proc.returncode,
        )

    if op == "push":
        _payload_exact(payload, op, frozenset({"remote", "branch", "force"}))
        remote = payload["remote"]
        branch = payload["branch"]
        force = payload["force"]
        if not isinstance(remote, str) or not remote:
            _fail("remote must be a non-empty string")
        if not isinstance(branch, str) or not branch:
            _fail("branch must be a non-empty string")
        if not isinstance(force, bool):
            _fail("force must be boolean")
        _assert_safe_token(remote, "remote")
        _assert_safe_token(branch, "branch")
        args = ["push", remote, branch]
        if force:
            args.insert(1, "--force")
        proc = _run_git(repo, git, args)
        return (
            {
                "gitStdout": proc.stdout,
                "gitStderr": proc.stderr,
                "errorSummary": proc.stderr.strip() or None,
            },
            proc.returncode,
        )

    if op == "pull":
        _payload_exact(payload, op, frozenset({"remote", "branch"}))
        remote = payload["remote"]
        branch = payload["branch"]
        if not isinstance(remote, str) or not remote:
            _fail("remote must be a non-empty string")
        if not isinstance(branch, str) or not branch:
            _fail("branch must be a non-empty string")
        _assert_safe_token(remote, "remote")
        _assert_safe_token(branch, "branch")
        proc = _run_git(repo, git, ["pull", remote, branch])
        return (
            {
                "gitStdout": proc.stdout,
                "gitStderr": proc.stderr,
                "errorSummary": proc.stderr.strip() or None,
            },
            proc.returncode,
        )

    if op == "fetch":
        _payload_exact(payload, op, frozenset({"remote", "prune"}))
        remote = payload["remote"]
        prune = payload["prune"]
        if not isinstance(remote, str) or not remote:
            _fail("remote must be a non-empty string")
        if not isinstance(prune, bool):
            _fail("prune must be boolean")
        _assert_safe_token(remote, "remote")
        args = ["fetch", remote]
        if prune:
            args.append("--prune")
        proc = _run_git(repo, git, args)
        return (
            {
                "gitStdout": proc.stdout,
                "gitStderr": proc.stderr,
                "errorSummary": proc.stderr.strip() or None,
            },
            proc.returncode,
        )

    _fail(f"unsupported operation_type: {op}")


def main() -> None:
    try:
        raw = sys.stdin.read()
        doc = json.loads(raw) if raw.strip() else {}
    except json.JSONDecodeError as e:
        _fail(f"invalid JSON stdin: {e}")

    op = doc.get("operation_type")
    repo_path = doc.get("repository_path")
    payload = doc.get("operation_payload_json")

    if op not in ALLOWED_OPS:
        _fail(f"operation_type must be one of {sorted(ALLOWED_OPS)}")
    if not isinstance(payload, dict):
        _fail("operation_payload_json must be a JSON object")

    repo = _resolve_repo(str(repo_path))
    git = _git_exe()
    _verify_git_repo(repo, git)

    data, code = _handle(op, repo, git, payload)
    _ok(data, code)


if __name__ == "__main__":
    main()
