#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Verificación de integridad de procesos SddIA (hash de fases, broker crypto, phase_invocations)."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

try:
    import yaml
except ImportError:  # pragma: no cover
    print("Requires PyYAML: pip install pyyaml", file=sys.stderr)
    sys.exit(2)


REPO = Path(__file__).resolve().parents[2]
PROCESS_DIR = REPO / "SddIA" / "process"
CRYPTO_SCRIPT = REPO / "scripts" / "skills" / "cryptography-manager.py"

SKIP_NAMES = frozenset({"process-contract", "index"})


def _sha256_phases_via_capsule(phases: list) -> str:
    canon = json.dumps(phases, separators=(",", ":"), ensure_ascii=False, sort_keys=True)
    payload = json.dumps(
        {
            "operation": "GENERATE_SHA256",
            "target_type": "STRING",
            "target_payload": canon,
        },
        ensure_ascii=False,
    )
    r = subprocess.run(
        [sys.executable, str(CRYPTO_SCRIPT)],
        input=payload,
        text=True,
        capture_output=True,
        cwd=str(REPO),
    )
    if r.returncode != 0:
        raise RuntimeError(r.stderr or r.stdout)
    out = json.loads(r.stdout)
    if not out.get("success"):
        raise RuntimeError(out.get("error", str(out)))
    return out["data"]["result"]


def _load_frontmatter(md: Path) -> dict:
    text = md.read_text(encoding="utf-8")
    parts = text.split("---", 2)
    if len(parts) < 3:
        raise ValueError(f"no frontmatter: {md}")
    return yaml.safe_load(parts[1])


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
