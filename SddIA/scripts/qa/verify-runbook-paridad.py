#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Gate documental L1-O5: detecta git-manager suelto para merge/push/delete en runbooks activos."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

SCRIPT = Path(__file__).resolve()
HISTORICAL_START = "<!-- runbook-historical -->"
HISTORICAL_END = "<!-- /runbook-historical -->"
GIT_MANAGER_RE = re.compile(r"git-manager\.py", re.IGNORECASE)
FORBIDDEN_OP_RE = re.compile(r"\bmerge\b|delete_branch|\bpush\b", re.IGNORECASE)
INVOCATION_RE = re.compile(
    r"(?:python|Get-Content|\|).*git-manager\.py|git-manager\.py",
    re.IGNORECASE,
)
PLANNING_SUFFIXES = frozenset(
    {"objectives.md", "clarify.md", "spec.md", "plan.md", "validacion.md", "implementation.md"}
)


def _should_scan(path: Path, repo: Path) -> bool:
    rel = path.relative_to(repo).as_posix()
    if rel.startswith("docs/todos/done/"):
        return False
    if path.name in PLANNING_SUFFIXES:
        return False
    if rel.startswith("docs/features/") and path.name == "execution.md":
        return True
    if rel.startswith("docs/todos/pending/"):
        return True
    return False


def _repo_root(explicit: str | None) -> Path:
    if explicit:
        return Path(explicit).resolve()
    if (SCRIPT.parents[2] / "tools").is_dir():
        return SCRIPT.parents[3]
    return SCRIPT.parents[2]


def _strip_historical(text: str) -> str:
    pattern = re.compile(
        re.escape(HISTORICAL_START) + r".*?" + re.escape(HISTORICAL_END),
        re.DOTALL,
    )
    return pattern.sub("", text)


def _scan_roots(repo: Path) -> list[Path]:
    roots: list[Path] = []
    features = repo / "docs" / "features"
    pending = repo / "docs" / "todos" / "pending"
    if features.is_dir():
        roots.append(features)
    if pending.is_dir():
        roots.append(pending)
    return roots


def _iter_markdown_files(roots: list[Path], repo: Path) -> list[Path]:
    files: list[Path] = []
    for root in roots:
        for md in sorted(root.rglob("*.md")):
            if _should_scan(md, repo):
                files.append(md)
    return files


def _line_violates(line: str) -> bool:
    if not GIT_MANAGER_RE.search(line):
        return False
    if not FORBIDDEN_OP_RE.search(line):
        return False
    return bool(INVOCATION_RE.search(line))


def scan_repo(repo: Path) -> list[dict[str, object]]:
    violations: list[dict[str, object]] = []
    for md in _iter_markdown_files(_scan_roots(repo), repo):
        rel = md.relative_to(repo).as_posix()
        if rel.startswith("docs/todos/done/"):
            continue
        body = _strip_historical(md.read_text(encoding="utf-8"))
        for i, line in enumerate(body.splitlines(), start=1):
            if _line_violates(line):
                violations.append(
                    {
                        "file": rel,
                        "line": i,
                        "snippet": line.strip()[:200],
                    }
                )
    return violations


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        default="",
        help="Raíz del repositorio (default: inferida desde script)",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Emitir JSON en stdout (default si no hay violaciones)",
    )
    args = parser.parse_args()
    repo = _repo_root(args.repo_root or None)
    violations = scan_repo(repo)
    payload = {"success": not violations, "violations": violations}
    print(json.dumps(payload, ensure_ascii=False, indent=2))
    if violations:
        for v in violations:
            print(
                f"VIOLATION: {v['file']}:{v['line']}: {v['snippet']}",
                file=sys.stderr,
            )
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
