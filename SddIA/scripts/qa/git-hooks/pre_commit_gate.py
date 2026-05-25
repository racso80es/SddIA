#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Puerta Argos pre-commit: verify-process-integrity + audit EDA (bus completo)."""

from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path

SCRIPT = Path(__file__).resolve()
REPO = SCRIPT.parents[4]
QA = REPO / "SddIA" / "scripts" / "qa"
VPI = QA / "verify-process-integrity.py"
AUDIT = QA / "audit-entity-eda-coverage.py"

# Rutas genómicas auditadas por audit-entity-eda-coverage (ENTITY_DIRS)
_GENOME_PREFIXES = (
    "SddIA/skills/",
    "SddIA/events/",
    "SddIA/process/",
    "SddIA/agents/",
    "SddIA/tools/",
    "SddIA/actions/",
    "SddIA/library/norms/",
    "SddIA/library/codexes/",
    ".SddIA/",
)


def _staged_paths() -> list[str]:
    proc = subprocess.run(
        ["git", "diff", "--cached", "--name-only", "--diff-filter=ACMR"],
        cwd=str(REPO),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    return [line.strip().replace("\\", "/") for line in (proc.stdout or "").splitlines() if line.strip()]


def _staged_touches_genome(staged: list[str]) -> bool:
    return any(p.startswith(_GENOME_PREFIXES) for p in staged)


def _run(cmd: list[str], *, isolate_stdio: bool = True) -> subprocess.CompletedProcess[str]:
    env = os.environ.copy()
    if isolate_stdio:
        env.pop("PYTHONIOENCODING", None)
    return subprocess.run(
        cmd,
        cwd=str(REPO),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        env=env,
    )


def main() -> int:
    if os.environ.get("SDDIA_SKIP_HOOKS") == "1":
        print("SddIA pre-commit: SKIPPED (SDDIA_SKIP_HOOKS=1)", file=sys.stderr)
        return 0

    if not VPI.is_file() or not AUDIT.is_file():
        print("SddIA pre-commit: missing QA scripts", file=sys.stderr)
        return 1

    py = sys.executable

    vpi = _run([py, str(VPI)])
    if vpi.returncode != 0:
        if vpi.stderr:
            print(vpi.stderr, file=sys.stderr, end="")
        if vpi.stdout:
            print(vpi.stdout, file=sys.stderr, end="")
        print("SddIA pre-commit: BLOCKED — verify-process-integrity failed", file=sys.stderr)
        return vpi.returncode or 1

    audit = _run([py, str(AUDIT), "--scan", "--json"], isolate_stdio=False)
    if audit.returncode != 0 and not audit.stdout:
        if audit.stderr:
            print(audit.stderr, file=sys.stderr, end="")
        print("SddIA pre-commit: BLOCKED — audit-entity-eda-coverage error", file=sys.stderr)
        return audit.returncode or 1

    try:
        report = json.loads(audit.stdout or "{}")
    except json.JSONDecodeError:
        print("SddIA pre-commit: invalid JSON from audit-entity-eda-coverage", file=sys.stderr)
        if audit.stdout:
            print(audit.stdout, file=sys.stderr)
        return 1

    orphan_count = int(report.get("orphan_count") or 0)
    staged = _staged_paths()
    if orphan_count > 0 and _staged_touches_genome(staged):
        print(f"SddIA pre-commit: BLOCKED — Argos orphan_count={orphan_count}", file=sys.stderr)
        for o in report.get("orphans") or []:
            ec = o.get("entity_class", "?")
            en = o.get("entity_name", "?")
            ap = o.get("artifact_path", "?")
            print(f"  - {ec}/{en} → {ap}", file=sys.stderr)
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
