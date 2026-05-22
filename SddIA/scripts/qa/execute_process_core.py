# -*- coding: utf-8 -*-
"""Núcleo del intérprete dinámico de procesos SddIA (laboratorio)."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Any

try:
    import yaml
except ImportError:
    yaml = None  # type: ignore

SCRIPT = Path(__file__).resolve()

# Inyectados por runtime IDE / Cúmulo — no exigidos en laboratorio v1.
RUNTIME_INJECTED_INPUTS = frozenset(
    {"cumulo_topology", "active_norm_pack", "active_norms", "target_executor_rbac"}
)

# Inputs con default documentado en contratos feature-like.
DEFAULTABLE_INPUTS = frozenset(
    {
        "persist_ref",
        "base_branch",
        "branch_name",
        "pbi_ref",
        "refined_requirements",
        "description",
        "pr_title",
        "pr_body",
        "target_branch",
    }
)

_INPUT_KEY_RE = re.compile(r'"([A-Za-z_][A-Za-z0-9_]*)"\s*:')


def repo_root() -> Path:
    for parent in SCRIPT.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace (SddIA/core/cumulo.paths.json)")


def emit(envelope: dict[str, Any], code: int | None = None) -> None:
    if code is None:
        code = 0 if envelope.get("success") else 1
    envelope.setdefault("exitCode", code)
    sys.stdout.write(json.dumps(envelope, ensure_ascii=False) + "\n")
    sys.exit(code)


def warn_stderr(message: str) -> None:
    yellow = "\033[33m"
    reset = "\033[0m"
    try:
        if not sys.stderr.isatty():
            yellow = reset = ""
    except Exception:
        yellow = reset = ""
    sys.stderr.write(f"{yellow}WARNING: {message}{reset}\n")


def parse_frontmatter(md_path: Path) -> dict[str, Any]:
    text = md_path.read_text(encoding="utf-8")
    parts = text.split("---", 2)
    if len(parts) < 3 or yaml is None:
        return {}
    data = yaml.safe_load(parts[1])
    return data if isinstance(data, dict) else {}


def resolve_process_path(repo: Path, process_name: str) -> Path:
    process_dir = repo / "SddIA" / "process"
    direct = process_dir / f"{process_name}.md"
    if direct.is_file():
        return direct
    if yaml is None:
        raise RuntimeError("PyYAML requerido para resolver aliases de proceso")
    for md in process_dir.glob("*.md"):
        if md.stem in ("index", "process-contract"):
            continue
        try:
            fm = yaml.safe_load(md.read_text(encoding="utf-8").split("---", 2)[1])
        except (IndexError, yaml.YAMLError):
            continue
        if not isinstance(fm, dict):
            continue
        if fm.get("name") == process_name:
            return md
        aliases = fm.get("aliases") or []
        if isinstance(aliases, list) and process_name in aliases:
            return md
    raise FileNotFoundError(f"Proceso no encontrado: {process_name}")


def load_process_def(repo: Path, process_name: str) -> tuple[str, dict[str, Any], list[dict[str, Any]]]:
    path = resolve_process_path(repo, process_name)
    if yaml is None:
        raise RuntimeError("PyYAML requerido")
    fm = yaml.safe_load(path.read_text(encoding="utf-8").split("---", 2)[1])
    if not isinstance(fm, dict):
        raise ValueError(f"Frontmatter inválido en {path}")
    phases = fm.get("phases") or []
    if not isinstance(phases, list):
        phases = []
    return fm.get("name") or path.stem, fm, phases


def extract_input_keys(process_def: dict[str, Any]) -> list[str]:
    raw = process_def.get("inputs_schema") or process_def.get("inputs") or []
    keys: list[str] = []
    if isinstance(raw, dict):
        return list(raw.keys())
    if not isinstance(raw, list):
        return keys
    for item in raw:
        if isinstance(item, dict):
            keys.extend(str(k) for k in item.keys())
        elif isinstance(item, str):
            m = _INPUT_KEY_RE.search(item)
            if m:
                keys.append(m.group(1))
    return keys


def validate_process_inputs(
    process_def: dict[str, Any],
    process_inputs: dict[str, Any],
    canonical: str,
) -> None:
    declared = extract_input_keys(process_def)
    if not declared:
        return

    required = [
        k
        for k in declared
        if k not in RUNTIME_INJECTED_INPUTS and k not in DEFAULTABLE_INPUTS
    ]
    missing = [k for k in required if k not in process_inputs or process_inputs[k] in (None, "")]
    if "refined_requirements" in missing and process_inputs.get("description"):
        missing = [k for k in missing if k != "refined_requirements"]
    if missing:
        raise ValueError(
            json.dumps(
                {
                    "code": "INPUT_VALIDATION",
                    "message": f"Faltan variables obligatorias para proceso '{canonical}'",
                    "missing": missing,
                    "declared_inputs": declared,
                },
                ensure_ascii=False,
            )
        )


def normalize_request(raw: dict[str, Any]) -> tuple[str, dict[str, Any]]:
    raise ValueError(
        "Entrada inválida o formato legacy no soportado: use estrictamente "
        "--process <nombre> --inputs '<json>'"
    )


def phase_invocations_index(process_def: dict[str, Any]) -> dict[str, dict[str, Any]]:
    raw = process_def.get("phase_invocations") or []
    if not isinstance(raw, list):
        return {}
    out: dict[str, dict[str, Any]] = {}
    for block in raw:
        if isinstance(block, dict) and block.get("phase_name"):
            out[str(block["phase_name"])] = block
    return out


def delegates_are_only_agents(delegates: list[Any]) -> bool:
    if not delegates:
        return True
    for d in delegates:
        if isinstance(d, str) and not d.startswith("agent:"):
            return False
    return True
