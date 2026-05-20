#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Puerta post-merge Ola B: accept-pr con merge_already_done (O4 en cápsula)."""

from __future__ import annotations

import sys
import uuid

from hook_common import git_config, git_run, infer_merged_branch, invoke_process, skip_hooks


def main() -> int:
    if skip_hooks():
        print("SddIA post-merge: SKIPPED (SDDIA_SKIP_HOOKS=1)", file=sys.stderr)
        return 0

    head = git_run(["symbolic-ref", "-q", "HEAD"])
    if head.returncode != 0:
        return 0
    ref = (head.stdout or "").strip()
    if ref != "refs/heads/main":
        return 0

    source_branch = infer_merged_branch()
    if not source_branch:
        print("SddIA post-merge: no merge branch inferred — no-op", file=sys.stderr)
        return 0

    author = git_config("user.email", "unknown@sddia.local")
    payload = {
        "source_branch": source_branch,
        "author": author,
        "correlation_id": str(uuid.uuid4()),
        "merge_already_done": True,
    }

    code = invoke_process("accept-pr", payload)
    if code != 0:
        print(f"SddIA post-merge: BLOCKED — accept-pr failed for {source_branch}", file=sys.stderr)
    return code


if __name__ == "__main__":
    sys.exit(main())
