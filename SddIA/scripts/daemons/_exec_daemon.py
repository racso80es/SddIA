#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Ejecuta un Centinela nativo con bóveda cargada (Windows / fallback)."""

from __future__ import annotations

import os
import sys
from pathlib import Path


def load_vault(repo: Path) -> None:
    qa = repo / "SddIA" / "scripts" / "qa"
    sys.path.insert(0, str(qa))
    from env_loader import load_hierarchical_env

    load_hierarchical_env(repo)


def resolve_binary(repo: Path, daemon: str) -> Path:
    target = repo / "SddIA" / "target"
    for profile in ("release", "debug"):
        for name in (f"{daemon}.exe", daemon):
            path = target / profile / name
            if path.is_file():
                return path
    raise FileNotFoundError(
        f"Binario no encontrado para {daemon} bajo SddIA/target/{{release|debug}}/"
    )


def emit_bat_env(repo: Path) -> int:
    qa = repo / "SddIA" / "scripts" / "qa"
    sys.path.insert(0, str(qa))
    from env_loader import load_hierarchical_env

    merged = load_hierarchical_env(repo)
    for key, value in merged.items():
        escaped = value.replace("%", "%%").replace('"', '""')
        print(f'set "{key}={escaped}"')
    return 0


def main(argv: list[str]) -> int:
    if len(argv) >= 2 and argv[1] == "--emit-bat-env":
        repo = Path(argv[2]).resolve()
        return emit_bat_env(repo)

    if len(argv) < 3:
        print("Uso: _exec_daemon.py <repo> <daemon> [args...]", file=sys.stderr)
        return 1

    repo = Path(argv[1]).resolve()
    daemon = argv[2]
    extra = argv[3:]

    os.environ["PYTHONUTF8"] = "1"
    load_vault(repo)

    node_glob = sorted((repo / ".tools").glob("node-v*-win-x64/bin/node.exe"))
    if not node_glob:
        node_glob = sorted((repo / ".tools").glob("node-v*-linux-x64/bin/node"))
    if node_glob:
        node_bin = node_glob[0].parent
        os.environ["PATH"] = f"{node_bin}{os.pathsep}{os.environ.get('PATH', '')}"

    binary = resolve_binary(repo, daemon)
    os.chdir(repo)
    os.execv(str(binary), [str(binary), *extra])
    return 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
