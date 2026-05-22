#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Gate documental: cierre en un PR (pbi_archived + PBI en done/, sin Fase B obligatoria)."""

from __future__ import annotations

import argparse
import os
import re
import sys
from pathlib import Path

try:
    import yaml
except ImportError:  # pragma: no cover
    print("Requires PyYAML: pip install pyyaml", file=sys.stderr)
    sys.exit(2)

SCRIPT = Path(__file__).resolve()


def _repo_root() -> Path:
    override = os.environ.get("SDDIA_REPO_ROOT", "").strip()
    if override:
        return Path(override).resolve()
    if (SCRIPT.parents[2] / "tools").is_dir():
        return SCRIPT.parents[3]
    return SCRIPT.parents[2]


REPO = _repo_root()
WORK_BRANCH_RE = re.compile(r"^(feat|fix|refactor|docs)/")


def _load_frontmatter(md: Path) -> dict | None:
    try:
        text = md.read_text(encoding="utf-8")
    except OSError as e:
        return {"_error": str(e)}
    parts = text.split("---", 2)
    if len(parts) < 3:
        return None
    data = yaml.safe_load(parts[1])
    return data if isinstance(data, dict) else None


def _find_pbi_in_done(feature_or_fix_name: str, persist_ref: str | None = None) -> list[Path]:
    done = REPO / "docs" / "todos" / "done"
    if not done.is_dir():
        return []
    hits: list[Path] = []
    slug = feature_or_fix_name.replace("_", "-").lower()
    persist_slug = ""
    if isinstance(persist_ref, str) and persist_ref.strip():
        persist_slug = persist_ref.strip().replace("\\", "/").rstrip("/").split("/")[-1].lower()
    for path in done.glob("*.md"):
        stem = path.stem.lower()
        if slug in stem or persist_slug in stem:
            hits.append(path)
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except OSError:
            continue
        if slug in text.lower() or (persist_ref and persist_ref in text):
            hits.append(path)
    return hits


def audit_validacion(path: Path, branch_hint: str | None) -> list[str]:
    errors: list[str] = []
    data = _load_frontmatter(path)
    if data is None:
        return [f"{path}: frontmatter YAML obligatorio"]
    if data.get("_error"):
        return [f"{path}: {data['_error']}"]

    rel = path.relative_to(REPO).as_posix()
    branch = data.get("branch") or branch_hint or ""
    if isinstance(branch, str) and branch.strip() and WORK_BRANCH_RE.match(branch.strip()):
        if data.get("pbi_archived") is not True:
            errors.append(f"{rel}: pbi_archived debe ser true en rama de trabajo pre-merge")

        merged_pr = data.get("merged_pr")
        merge_commit = data.get("merge_commit")
        if merged_pr not in (None, "", "null") and not merge_commit:
            errors.append(
                f"{rel}: merged_pr sin merge_commit — use ambos opcionales o ninguno (v1.2.0)"
            )

        feature_name = data.get("feature_name") or path.parent.name
        persist_ref = f"docs/features/{feature_name}"
        if path.parts[-3:-1] == ("docs", "fixes"):
            persist_ref = f"docs/fixes/{path.parent.name}"
        if data.get("pbi_archived") is True and not _find_pbi_in_done(
            str(feature_name), persist_ref
        ):
            errors.append(
                f"{rel}: pbi_archived true pero sin PBI coincidente en docs/todos/done/"
            )

    global_verdict = data.get("global")
    if global_verdict == "APTO":
        pending = REPO / "docs" / "todos" / "pending"
        feature_name = data.get("feature_name") or path.parent.name
        for p in pending.glob("*.md"):
            if feature_name in p.stem or str(feature_name).replace("-", "") in p.stem.replace("-", ""):
                if data.get("pbi_archived") is True:
                    errors.append(
                        f"{rel}: global APTO y pbi_archived true pero PBI sigue en pending/: {p.name}"
                    )
                break

    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description="Verificar cierre documental un PR")
    parser.add_argument("--path", help="validacion.md concreto")
    parser.add_argument("--scan", action="store_true", help="Escanear docs/features y docs/fixes")
    args = parser.parse_args()

    paths: list[Path] = []
    if args.path:
        paths.append(REPO / args.path.replace("\\", "/"))
    elif args.scan:
        for base in ("docs/features", "docs/fixes"):
            root = REPO / base
            if root.is_dir():
                for d in root.iterdir():
                    if d.is_dir():
                        v = d / "validacion.md"
                        if v.is_file():
                            paths.append(v)
    else:
        parser.error("Indique --path o --scan")

    errors: list[str] = []
    for path in paths:
        if not path.is_file():
            continue
        branch = None
        try:
            proc = __import__("subprocess").run(
                ["git", "branch", "--show-current"],
                cwd=str(REPO),
                capture_output=True,
                text=True,
                encoding="utf-8",
                errors="replace",
                check=False,
            )
            branch = (proc.stdout or "").strip() or None
        except OSError:
            branch = None
        errors.extend(audit_validacion(path, branch))

    if errors:
        print("verify-task-closure: FAILED", file=sys.stderr)
        for e in errors:
            print(e, file=sys.stderr)
        return 1
    print("verify-task-closure: OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
