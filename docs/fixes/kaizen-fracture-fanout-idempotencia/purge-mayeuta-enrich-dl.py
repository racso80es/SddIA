#!/usr/bin/env python3
"""CA8: archiva DL históricos de mayeuta.enrich-fracture-pbi-kaizen (PBI de Cúmulo no encontrado)."""

from __future__ import annotations

import json
import shutil
import sys
from datetime import datetime, timezone
from pathlib import Path

REPO = Path(__file__).resolve().parents[3]
SRC = REPO / ".events/dead-letter/subscribers"
ARCHIVE = (
    REPO
    / ".events/dead-letter/archive/kaizen-fracture-fanout-idempotencia/mayeuta-enrich"
)
NEEDLE = "PBI de Cúmulo no encontrado"
SUFFIX = ".mayeuta.enrich-fracture-pbi-kaizen.json"


def main() -> int:
    if not SRC.is_dir():
        print(json.dumps({"error": f"source missing: {SRC}"}))
        return 1

    ARCHIVE.mkdir(parents=True, exist_ok=True)
    moved: list[str] = []
    skipped = 0

    for path in sorted(SRC.glob(f"*{SUFFIX}")):
        text = path.read_text(encoding="utf-8", errors="replace")
        if NEEDLE not in text:
            skipped += 1
            continue
        dest = ARCHIVE / path.name
        shutil.move(str(path), str(dest))
        moved.append(path.name)

    manifest = {
        "purged_at": datetime.now(timezone.utc).isoformat(),
        "pbi": "PBI-KAIZEN-FRACTURE-FANOUT-IDEMPOTENCIA",
        "reason": "Histórico pre-idempotencia: enrich-fracture-pbi-kaizen Err por ruta reconstruida",
        "needle": NEEDLE,
        "source_dir": str(SRC.relative_to(REPO)),
        "archive_dir": str(ARCHIVE.relative_to(REPO)),
        "moved_count": len(moved),
        "skipped_count": skipped,
        "files": moved,
    }
    manifest_path = ARCHIVE / "manifest.json"
    manifest_path.write_text(
        json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )
    print(
        json.dumps(
            {
                "purge_complete": True,
                "moved_count": len(moved),
                "skipped_count": skipped,
                "manifest": str(manifest_path.relative_to(REPO)),
            },
            ensure_ascii=False,
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
