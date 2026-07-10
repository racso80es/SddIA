#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Recalcula hash_signature (SHA-256 de phases canónico) en SddIA/process/*.md."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

from execute_process_core import parse_frontmatter

SCRIPT = Path(__file__).resolve()
REPO = SCRIPT.parents[3] if (SCRIPT.parents[2] / "tools").is_dir() else SCRIPT.parents[2]
PROCESS_DIR = REPO / "SddIA" / "process"
SKIP_NAMES = frozenset({"process-contract", "index"})
HASH_LINE_RE = re.compile(r"^(hash_signature:\s*)sha256:[0-9a-f]{64}\s*$", re.MULTILINE)


def _sha256_phases(phases: list) -> str:
    import hashlib

    canon = json.dumps(phases, separators=(",", ":"), ensure_ascii=False, sort_keys=True)
    return hashlib.sha256(canon.encode("utf-8")).hexdigest()


def _load_frontmatter(md: Path) -> tuple[dict, str, str]:
    text = md.read_text(encoding="utf-8")
    parts = text.split("---", 2)
    if len(parts) < 3:
        raise ValueError(f"no frontmatter: {md}")
    data = parse_frontmatter(md)
    return data, parts[0], parts[2]


def recalc_file(md: Path, write: bool) -> dict[str, str] | None:
    data, _, _ = _load_frontmatter(md)
    phases = data.get("phases")
    if not isinstance(phases, list):
        return None
    hs = data.get("hash_signature") or ""
    if not hs.startswith("sha256:"):
        return None
    old = hs.split(":", 1)[1]
    new = _sha256_phases(phases)
    if old == new:
        return None
    if write:
        text = md.read_text(encoding="utf-8")
        if not HASH_LINE_RE.search(text):
            raise ValueError(f"{md.name}: hash_signature line not found")
        md.write_text(HASH_LINE_RE.sub(f"hash_signature: sha256:{new}", text, count=1), encoding="utf-8")
    return {"file": md.name, "old": old[:16], "new": new[:16], "full_new": new}


def main() -> int:
    parser = argparse.ArgumentParser(description="Recalcular hash_signature de procesos")
    parser.add_argument("--write", action="store_true", help="Persistir cambios en archivos")
    parser.add_argument("--files", nargs="*", help="Stems concretos (sin .md); default todos")
    args = parser.parse_args()

    stems = set(args.files) if args.files else None
    changed: list[dict[str, str]] = []

    for md in sorted(PROCESS_DIR.glob("*.md")):
        if md.stem in SKIP_NAMES:
            continue
        if stems is not None and md.stem not in stems:
            continue
        try:
            row = recalc_file(md, args.write)
        except Exception as e:
            print(f"{md.name}: error {e}", file=sys.stderr)
            return 1
        if row:
            changed.append(row)
            print(f"{row['file']}: {row['old']} -> {row['new']}")

    if not changed:
        print("recalc-process-hash-signatures: nothing to update")
        return 0
    print(f"recalc-process-hash-signatures: {len(changed)} file(s)" + (" written" if args.write else " (dry-run)"))
    return 0


if __name__ == "__main__":
    sys.exit(main())
