#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Verificación de integridad de procesos SddIA (hash de fases, broker crypto, phase_invocations).

Recálculo masivo de hash_signature: SddIA/scripts/qa/recalc-process-hash-signatures.py --write
"""

from __future__ import annotations

import json
import os
import sys
from pathlib import Path

from execute_process_core import parse_frontmatter

SCRIPT = Path(__file__).resolve()


def _repo_root() -> Path:
    override = os.environ.get("SDDIA_REPO_ROOT", "").strip()
    if override:
        return Path(override).resolve()
    if (SCRIPT.parents[2] / "tools").is_dir():
        return SCRIPT.parents[3]
    return SCRIPT.parents[2]


REPO = _repo_root()
PROCESS_DIR = REPO / "SddIA" / "process"

SKIP_NAMES = frozenset({"process-contract", "index"})


def _sha256_phases_via_capsule(phases: list) -> str:
    import hashlib

    canon = json.dumps(phases, separators=(",", ":"), ensure_ascii=False, sort_keys=True)
    return hashlib.sha256(canon.encode("utf-8")).hexdigest()


def _load_frontmatter(md: Path) -> dict:
    data = parse_frontmatter(md)
    if not data:
        raise ValueError(f"no frontmatter: {md}")
    return data


def main() -> int:
    errors: list[str] = []
    if not PROCESS_DIR.is_dir():
        errors.append(f"Missing {PROCESS_DIR}")
        print("\n".join(errors))
        return 1

    for md in sorted(PROCESS_DIR.glob("*.md")):
        stem = md.stem
        if stem in SKIP_NAMES:
            continue
        try:
            data = _load_frontmatter(md)
        except Exception as e:
            errors.append(f"{md.name}: frontmatter error: {e}")
            continue

        phases = data.get("phases")
        if not isinstance(phases, list):
            errors.append(f"{md.name}: missing phases array")
            continue

        for i, ph in enumerate(phases):
            dt = ph.get("delegates_to") or []
            if "skill:cryptography-manager" in dt:
                errors.append(
                    f"{md.name}: phase {i} declares skill:cryptography-manager; use action:crypto-broker per process-contract v1.2.0+"
                )

        hs = data.get("hash_signature") or ""
        if hs.startswith("sha256:"):
            expected = hs.split(":", 1)[1]
            try:
                computed = _sha256_phases_via_capsule(phases)
            except Exception as e:
                errors.append(f"{md.name}: hash compute failed: {e}")
                continue
            if computed != expected:
                errors.append(
                    f"{md.name}: hash_signature mismatch (file {expected[:16]}… vs computed {computed[:16]}…)"
                )

        inv = data.get("phase_invocations") or []
        for ph in phases:
            if not isinstance(ph, dict):
                continue
            if "action:crypto-broker" in (ph.get("delegates_to") or []):
                pname = ph.get("name")
                if not any(isinstance(b, dict) and b.get("phase_name") == pname for b in inv):
                    errors.append(
                        f"{md.name}: phase {pname!r} delegates to crypto-broker but has no phase_invocations block"
                    )

    if errors:
        print("verify-process-integrity: FAILED", file=sys.stderr)
        for e in errors:
            print(e, file=sys.stderr)
        return 1
    print("verify-process-integrity: OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
