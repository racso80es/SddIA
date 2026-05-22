#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Smoke: fetch + checkout origin-tracking + verify (mismo patrón que aduana PR).

Uso:
  python SddIA/scripts/qa/smoke-pr-review-verify-integrity.py --branch fix/foo
  python SddIA/scripts/qa/smoke-pr-review-verify-integrity.py --branch fix/foo --legacy-checkout
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
from pathlib import Path

_QA = Path(__file__).resolve().parent
if str(_QA) not in sys.path:
    sys.path.insert(0, str(_QA))

from execute_process_capsules import _sync_pr_review_worktree, invoke_git_manager
from execute_process_core import repo_root


def _run_verify(repo: Path) -> int:
    script = repo / "SddIA" / "scripts" / "qa" / "verify-process-integrity.py"
    env = os.environ.copy()
    env["SDDIA_REPO_ROOT"] = str(repo.resolve())
    proc = subprocess.run(
        [sys.executable, str(script)],
        cwd=str(repo),
        env=env,
        check=False,
    )
    return int(proc.returncode)


def main() -> int:
    parser = argparse.ArgumentParser(description="Smoke PR review verify integrity sync")
    parser.add_argument("--branch", required=True, help="Rama bajo revisión (pr_branch)")
    parser.add_argument(
        "--legacy-checkout",
        action="store_true",
        help="Reproducir patrón antiguo: fetch + checkout local sin origin/-B",
    )
    args = parser.parse_args()
    repo = repo_root()
    branch = args.branch.strip()

    if args.legacy_checkout:
        invoke_git_manager(repo, "fetch", {"remote": "origin", "prune": True})
        invoke_git_manager(repo, "checkout", {"branch_name": branch, "create_if_not_exists": False})
        mode = "legacy-checkout"
    else:
        sync = _sync_pr_review_worktree(repo, branch)
        mode = sync.get("mode", "unknown")

    code = _run_verify(repo)
    print(f"smoke-pr-review-verify-integrity: mode={mode} verify_exit={code}")
    return code


if __name__ == "__main__":
    sys.exit(main())
