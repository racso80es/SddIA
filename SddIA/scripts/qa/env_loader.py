# -*- coding: utf-8 -*-
"""Jerarquía de Bóvedas: carga .dev/.env (global) → .SddIA/.dev/.env (local)."""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path

_CONFIG_LOG = "[CONFIG] Jerarquía detectada: Aplicando SddIA/.dev/.env sobre .dev/.env"
_EXPORT_RE = re.compile(r"^export\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.*)$")
_PAIR_RE = re.compile(r"^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.*)$")


def _strip_quotes(value: str) -> str:
    v = value.strip()
    if len(v) >= 2 and v[0] == v[-1] and v[0] in ('"', "'"):
        return v[1:-1]
    return v


def parse_dotenv_file(path: Path) -> dict[str, str]:
    """Parsea un fichero dotenv línea a línea."""
    result: dict[str, str] = {}
    text = path.read_text(encoding="utf-8")
    for lineno, raw_line in enumerate(text.splitlines(), start=1):
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        m = _EXPORT_RE.match(line) or _PAIR_RE.match(line)
        if not m:
            raise ValueError(f"{path}:{lineno}: línea dotenv inválida")
        key, value = m.group(1), _strip_quotes(m.group(2))
        result[key] = value
    return result


_VAULT_PRECEDENCE_KEYS = frozenset(
    {
        "SDDIA_LAB_SIMULATE_IOTA",
        "SDDIA_IOTA_TIMEOUT_SECONDS",
    }
)


def apply_env(merged: dict[str, str]) -> None:
    """Aplica bóveda al entorno; flags IOTA en _VAULT_PRECEDENCE_KEYS prevalecen sobre el SO."""
    for key, value in merged.items():
        os.environ.setdefault(key, value)
    for key in _VAULT_PRECEDENCE_KEYS:
        if key in merged:
            os.environ[key] = merged[key]


def load_hierarchical_env(repo_root: Path) -> dict[str, str]:
    """Carga bóveda global, luego local (local prevalece en dict); respeta SO en apply."""
    global_path = repo_root / ".dev" / ".env"
    local_path = repo_root / ".SddIA" / ".dev" / ".env"
    merged: dict[str, str] = {}
    global_exists = global_path.is_file()
    local_exists = local_path.is_file()

    if global_exists:
        merged.update(parse_dotenv_file(global_path))
    if local_exists:
        if global_exists and local_exists:
            sys.stderr.write(_CONFIG_LOG + "\n")
        merged.update(parse_dotenv_file(local_path))

    apply_env(merged)
    return merged


def load_test_env_overlay(repo_root: Path) -> dict[str, str]:
    """Carga .dev/.env.test (o .env.test.example) sobre el entorno actual."""
    test_path = repo_root / ".dev" / ".env.test"
    if not test_path.is_file():
        test_path = repo_root / ".dev" / ".env.test.example"
    if not test_path.is_file():
        return {}
    overlay = parse_dotenv_file(test_path)
    for key, value in overlay.items():
        os.environ[key] = value
    return overlay
