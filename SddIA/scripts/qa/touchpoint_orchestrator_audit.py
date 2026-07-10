#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Auditoría P10-P13: touchpoints de producción usan SSOT orquestador (binario preferente)."""

from __future__ import annotations

import re
import sys
from pathlib import Path

SCRIPT = Path(__file__).resolve()
REPO = next(
    p for p in SCRIPT.parents if (p / "SddIA" / "core" / "cumulo.paths.json").is_file()
)

# Rutas productivas que deben resolver vía orchestrator_resolve o execute_process_bin nativo.
SCAN_ROOTS = [
    REPO / "SddIA" / "scripts" / "qa" / "git-hooks",
    REPO / "SddIA" / "scripts" / "qa" / "run-eda-e2e-lab.py",
    REPO / "SddIA" / "scripts" / "qa" / "route_domain_event_core.py",
    REPO / ".SddIA" / "client" / "sddia-client-bridge.py",
    REPO / "SddIA" / "daemons" / "event-watcher" / "src" / "main.rs",
    REPO / "SddIA" / "daemons" / "telegram-watcher" / "src" / "main.rs",
    REPO / "sddia-run.sh",
]

ALLOWLIST = {
    REPO / "SddIA" / "scripts" / "qa" / "orchestrator_resolve.py",
    REPO / "SddIA" / "scripts" / "qa" / "golden_orchestrator_parity.py",
    REPO / "SddIA" / "scripts" / "qa" / "execute_process_capsules.py",
}

FORBIDDEN = re.compile(
    r"(\[sys\.executable,\s*str\(.*execute-process\.py|"
    r"python3?\s+SddIA/scripts/qa/execute-process\.py|"
    r"subprocess\.run\(\s*\[\s*sys\.executable,\s*str\(.*execute-process\.py)"
)


def scan_file(path: Path) -> list[str]:
    if path in ALLOWLIST or not path.is_file():
        return []
    text = path.read_text(encoding="utf-8", errors="replace")
    hits: list[str] = []
    for i, line in enumerate(text.splitlines(), start=1):
        if "execute-process.py" not in line:
            continue
        if "orchestrator_resolve" in line or "fallback" in line.lower():
            continue
        if FORBIDDEN.search(line):
            hits.append(f"{path.relative_to(REPO)}:{i}: {line.strip()}")
        elif "execute-process.py" in line and path.suffix == ".py":
            if "resolve_orchestrator" not in text and "_invoke_route_process" not in text:
                if "EXECUTE_PROCESS" in line or "execute-process.py" in line:
                    if path.name not in ("orchestrator_resolve.py", "execute-process.py"):
                        hits.append(
                            f"{path.relative_to(REPO)}:{i}: referencia directa sin SSOT"
                        )
    return hits


def main() -> int:
    failures: list[str] = []
    for root in SCAN_ROOTS:
        if root.is_file():
            failures.extend(scan_file(root))
        elif root.is_dir():
            for path in root.rglob("*"):
                if path.suffix in (".py", ".rs", ".sh") and path.is_file():
                    failures.extend(scan_file(path))

    # Verificar presencia de patrones SSOT en archivos clave
    checks = {
        REPO / "SddIA" / "scripts" / "qa" / "git-hooks" / "hook_common.py": "resolve_orchestrator_cmd",
        REPO / "SddIA" / "scripts" / "qa" / "execute_process_capsules.py": "resolve_orchestrator_cmd",
        REPO / "SddIA" / "scripts" / "qa" / "route_domain_event_core.py": "resolve_orchestrator_cmd",
        REPO / ".SddIA" / "client" / "sddia-client-bridge.py": "resolve_orchestrator_cmd",
        REPO / "SddIA" / "daemons" / "event-watcher" / "src" / "main.rs": "execute_process_bin",
        REPO / "SddIA" / "daemons" / "telegram-watcher" / "src" / "main.rs": "execute_process_bin",
    }
    for path, needle in checks.items():
        if not path.is_file():
            failures.append(f"missing: {path.relative_to(REPO)}")
            continue
        if needle not in path.read_text(encoding="utf-8", errors="replace"):
            failures.append(f"{path.relative_to(REPO)}: falta {needle}")

    if failures:
        print("FAIL touchpoint audit:")
        for f in failures:
            print(f"  {f}")
        return 1
    print("OK  touchpoint audit P10-P13")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
