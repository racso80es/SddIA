#!/usr/bin/env python3
"""Enumeración y borrado seguro de generaciones viejas en SddIA/target/*/deps (AUT-79)."""
from __future__ import annotations

import json
import os
import sys
from pathlib import Path

ARTIFACT_SUFFIXES = (".rlib", ".so", ".a", ".dylib")
DEPS_DIRS = (
    "SddIA/target/debug/deps",
    "SddIA/target/release/deps",
)
FORBIDDEN = "SddIA/target/native-debug-bins"


def split_logical_and_hash(filename: str) -> tuple[str, str]:
    stem = Path(filename).stem
    if "-" not in stem:
        return stem, ""
    base, suffix = stem.rsplit("-", 1)
    if len(suffix) >= 8 and all(c in "0123456789abcdefABCDEF" for c in suffix):
        return base, suffix.lower()
    return stem, ""


def hash_has_artifact(paths: list[Path]) -> bool:
    for p in paths:
        if p.suffix in ARTIFACT_SUFFIXES:
            return True
    return False


def newest_hash_mtime(files_by_hash: dict[str, list[Path]]) -> str:
    best_hash = ""
    best_mtime = -1.0
    for h, paths in files_by_hash.items():
        if not h:
            continue
        mt = max(p.stat().st_mtime for p in paths)
        if mt > best_mtime:
            best_mtime = mt
            best_hash = h
    return best_hash


def classify_deps_dir(repo: Path, rel_deps: str) -> tuple[list[Path], int]:
    """Return paths to delete and count of held groups."""
    deps = repo / rel_deps
    if not deps.is_dir():
        return [], 0
    by_logical: dict[str, dict[str, list[Path]]] = {}
    for entry in deps.iterdir():
        if not entry.is_file():
            continue
        logical, h = split_logical_and_hash(entry.name)
        by_logical.setdefault(logical, {}).setdefault(h, []).append(entry)

    to_delete: list[Path] = []
    hold_groups = 0
    for _logical, hashes in by_logical.items():
        if len(hashes) <= 1:
            continue
        keep = newest_hash_mtime(hashes)
        if not keep:
            hold_groups += 1
            continue
        keep_paths = hashes.get(keep, [])
        if not hash_has_artifact(keep_paths):
            if any(hash_has_artifact(v) for hk, v in hashes.items() if hk != keep):
                hold_groups += 1
                continue
        for h, paths in hashes.items():
            if h == keep:
                continue
            to_delete.extend(paths)
    return to_delete, hold_groups


def validate_path(repo: Path, path: Path) -> None:
    real = path.resolve()
    repo_real = repo.resolve()
    try:
        real.relative_to(repo_real)
    except ValueError as exc:
        raise SystemExit(f"path fuera del repo: {path}") from exc
    rel = str(real.relative_to(repo_real)).replace("\\", "/")
    if FORBIDDEN in rel or rel.startswith(FORBIDDEN + "/"):
        raise SystemExit(f"path prohibido: {rel}")
    ok = any(rel.startswith(d + "/") or rel == d for d in DEPS_DIRS)
    if not ok:
        raise SystemExit(f"path fuera de deps allowlist: {rel}")


def run(repo_s: str, apply: bool) -> dict:
    repo = Path(repo_s)
    cumulo = repo / "SddIA/core/cumulo.paths.json"
    if not cumulo.is_file():
        raise SystemExit(f"no es repo SddIA: {repo}")

    all_delete: list[Path] = []
    hold_total = 0
    for rel in DEPS_DIRS:
        deleted, hold = classify_deps_dir(repo, rel)
        all_delete.extend(deleted)
        hold_total += hold

    bytes_total = sum(p.stat().st_size for p in all_delete)
    for p in all_delete:
        validate_path(repo, p)

    action = "empty"
    if all_delete:
        action = "pruned" if apply else "would-prune"
    elif hold_total:
        action = "hold-incomplete"

    if apply and all_delete:
        for p in all_delete:
            p.unlink()

    return {
        "cargo_deps": len(all_delete),
        "cargo_deps_bytes": bytes_total,
        "cargo_deps_action": action,
        "cargo_deps_hold_groups": hold_total,
    }


def main() -> None:
    if len(sys.argv) < 2:
        print("uso: clean-project-ephemeral-cargo-deps.py REPO [--apply]", file=sys.stderr)
        sys.exit(2)
    repo = sys.argv[1]
    apply = "--apply" in sys.argv[2:]
    print(json.dumps(run(repo, apply), separators=(",", ":")))


if __name__ == "__main__":
    main()
