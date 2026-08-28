#!/usr/bin/env python3
"""Migración única CA12: anotar fracture_hash y fracture_process en PBI de fractura."""

from __future__ import annotations

import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
DOC_ID_RE = re.compile(r"^PBI-FIX-FRACTURE-([0-9a-f]{12})(?:-R\d+)?$")
TITLE_RE = re.compile(
    r"^\[(?:FIX|REGRESIÓN)\]\s+(.+?)\s+—\s+fractura sistémica",
    re.IGNORECASE,
)
PROCESS_CELL_RE = re.compile(r"\|\s*Proceso\s*\|\s*`([^`]+)`\s*\|")


def slugify(name: str) -> str:
    lower = name.strip().lower()
    slug = re.sub(r"[^\w\-]+", "-", lower)
    slug = re.sub(r"-+", "-", slug).strip("-")
    if not slug:
        return "fracture"
    return slug[:48]


def extract_process(title: str, body: str) -> str | None:
    m = TITLE_RE.match(title.strip())
    if m:
        return slugify(m.group(1))
    m2 = PROCESS_CELL_RE.search(body)
    if m2:
        return slugify(m2.group(1))
    return None


def inject_frontmatter_fields(text: str, fields: dict[str, str]) -> str:
    if not text.startswith("---"):
        return text
    parts = text.split("---", 2)
    if len(parts) < 3:
        return text
    fm = parts[1]
    body = parts[2]
    lines = fm.splitlines()
    out: list[str] = []
    inserted = {k: False for k in fields}
    for line in lines:
        key = line.split(":", 1)[0].strip()
        if key in fields:
            out.append(f"{key}: {fields[key]}")
            inserted[key] = True
        else:
            out.append(line)
    for key, val in fields.items():
        if not inserted[key]:
            # Insertar tras process: bug-fix si existe
            idx = next((i for i, l in enumerate(out) if l.startswith("process:")), len(out))
            out.insert(idx + 1, f"{key}: {val}")
    return "---\n" + "\n".join(out) + "\n" + body


def backfill_file(path: Path) -> bool:
    raw = path.read_text(encoding="utf-8")
    if "fracture_hash:" in raw.split("---", 2)[1] if raw.startswith("---") else "":
        return False
    fm_block = raw.split("---", 2)[1] if raw.startswith("---") else ""
    doc_m = re.search(r"^document_id:\s*(\S+)", fm_block, re.M)
    if not doc_m:
        return False
    doc_id = doc_m.group(1).strip().strip('"').strip("'")
    hash_m = DOC_ID_RE.match(doc_id)
    if not hash_m:
        return False
    fracture_hash = hash_m.group(1)
    title_m = re.search(r'^title:\s*"?([^"\n]+)"?', fm_block, re.M)
    title = title_m.group(1) if title_m else ""
    body = raw.split("---", 2)[2] if raw.startswith("---") else raw
    fracture_process = extract_process(title, body)
    if not fracture_process:
        print(f"SKIP (sin proceso): {path}", file=sys.stderr)
        return False
    updated = inject_frontmatter_fields(
        raw,
        {
            "fracture_hash": fracture_hash,
            "fracture_process": fracture_process,
        },
    )
    if updated != raw:
        path.write_text(updated, encoding="utf-8")
        return True
    return False


def main() -> int:
    roots = [
        REPO / "docs/todos/done",
        REPO / "docs/todos/pending",
    ]
    changed = 0
    for root in roots:
        if not root.is_dir():
            continue
        for path in sorted(root.glob("*.md")):
            if backfill_file(path):
                print(path.relative_to(REPO))
                changed += 1
    print(f"backfill_complete changed={changed}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
