# -*- coding: utf-8 -*-
"""Forjas físicas de entidades genómicas (laboratorio QA)."""

from __future__ import annotations

import json
import os
import re
import subprocess
from pathlib import Path
from typing import Any, Callable

from execute_process_core import parse_frontmatter


def try_native_forge(repo: Path, inputs: dict[str, Any]) -> dict[str, Any] | None:
    """Delega en binario Rust `--forge` cuando está disponible (P6/P7)."""
    if os.environ.get("SDDIA_DISABLE_NATIVE_FORGES", "").strip().lower() in (
        "1",
        "true",
        "yes",
    ):
        return None
    try:
        from orchestrator_resolve import resolve_orchestrator_cmd

        proc = subprocess.run(
            resolve_orchestrator_cmd(
                repo,
                ["--forge", "--inputs", json.dumps(inputs, ensure_ascii=False)],
            ),
            cwd=str(repo),
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            env=os.environ.copy(),
            check=False,
        )
        line = (proc.stdout or "").strip().splitlines()[-1] if proc.stdout else ""
        if not line:
            return None
        body = json.loads(line)
        if body.get("success") and isinstance(body.get("data"), dict):
            return body["data"]
    except (OSError, json.JSONDecodeError, RuntimeError, ValueError):
        return None
    return None


def _sha256_canon(repo: Path, canon: dict[str, Any]) -> str:
    lab = os.environ.get("SDDIA_FORGE_LAB_SHA256", "").strip()
    if lab:
        return f"sha256:{lab}"
    from execute_process_capsules import crypto

    hex_sig = crypto(
        repo,
        {
            "operation": "GENERATE_SHA256",
            "target_type": "STRING",
            "target_payload": json.dumps(
                canon, sort_keys=True, separators=(",", ":"), ensure_ascii=False
            ),
        },
    )
    return f"sha256:{hex_sig}"


def _uuid(repo: Path) -> str:
    lab = os.environ.get("SDDIA_FORGE_LAB_UUID", "").strip()
    if lab:
        return lab
    from execute_process_capsules import crypto

    return crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})


def _append_row(index_path: Path, row: str, name: str) -> None:
    if not index_path.is_file():
        return
    idx = index_path.read_text(encoding="utf-8")
    if name in idx:
        return
    lines = idx.splitlines()
    for i, line in enumerate(lines):
        if line.startswith("|") and i + 1 < len(lines) and set(lines[i + 1].strip()) <= {"|", "-", " ", ":"}:
            lines.insert(i + 2, row)
            index_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
            return
    index_path.write_text(idx.rstrip() + "\n" + row + "\n", encoding="utf-8")


def idempotent_forge_handoff(repo: Path, artifact: Path, lifecycle: str) -> dict[str, Any] | None:
    if lifecycle != "create" or not artifact.is_file():
        return None
    fm = parse_frontmatter(artifact)
    if not fm.get("uuid"):
        return None
    return {
        "handoff_entity_uuid": fm.get("uuid"),
        "handoff_hash_signature_new": fm.get("hash_signature"),
        "handoff_hash_signature_old": None,
        "handoff_version": fm.get("version"),
        "idempotent": True,
    }


def run_tool_forge(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    name = inputs.get("tool_name") or inputs.get("entity_name")
    if not isinstance(name, str) or not name:
        raise ValueError("tool_name requerido")
    scope = inputs.get("scope", "core")
    base = repo / ".SddIA" / "tools" if scope == "local" else repo / "SddIA" / "tools"
    tool_path = base / f"{name}.md"
    lifecycle = inputs.get("lifecycle_operation", "create")
    skip = idempotent_forge_handoff(repo, tool_path, lifecycle)
    if skip:
        return skip

    context = inputs.get("tool_context", "ecosystem-evolution")
    version = inputs.get("tool_version", "1.0.0")
    contract_ver = inputs.get("tools_contract_version", "1.2.0")
    domain_origin = inputs.get("domain_origin", "SddIA")
    desc = inputs.get("execution_logic", f"Tool {name}")
    tool_uuid = _uuid(repo)
    cap = name.replace("-", "_")[:32] or "tool-cap"
    hash_sig = _sha256_canon(repo, {"tool_name": name, "tool_context": context, "scope": scope})

    body = f"""---
uuid: "{tool_uuid}"
name: "{name}"
version: "{version}"
contract: "tools-contract v{contract_ver}"
domain_origin: "{domain_origin}"
context: "{context}"
capabilities:
  - "{cap}"
hash_signature: "{hash_sig}"
implementation_path_ref: "SddIA/tools/{name}"
---

# {name}

{desc}
"""
    tool_path.parent.mkdir(parents=True, exist_ok=True)
    tool_path.write_text(body, encoding="utf-8")
    row = (
        f"| `{name}.md` | `{tool_uuid}` | {name} | {version} | "
        f"tools-contract v{contract_ver} | {context} | `{cap}` |"
    )
    _append_row(base / "index.md", row, name)
    return {
        "handoff_entity_uuid": tool_uuid,
        "handoff_hash_signature_new": hash_sig,
        "handoff_hash_signature_old": None,
        "handoff_version": version,
        "origin_topology": "local" if scope == "local" else "core",
    }


def run_action_forge(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    name = inputs.get("action_name") or inputs.get("entity_name")
    if not isinstance(name, str) or not name:
        raise ValueError("action_name requerido")
    action_path = repo / "SddIA" / "actions" / f"{name}.md"
    lifecycle = inputs.get("lifecycle_operation", "create")
    skip = idempotent_forge_handoff(repo, action_path, lifecycle)
    if skip:
        return skip

    context = inputs.get("action_context", "ecosystem-evolution")
    version = inputs.get("action_version", "1.0.0")
    contract_ver = inputs.get("actions_contract_version", "1.2.0")
    desc = str(inputs.get("orchestration_logic", f"Acción {name}"))[:80]
    action_uuid = _uuid(repo)
    cap = name.replace("-", "_")[:32] or "action-cap"
    hash_sig = _sha256_canon(repo, {"action_name": name, "action_context": context})

    body = f"""---
uuid: "{action_uuid}"
name: "{name}"
version: "{version}"
contract: "actions-contract v{contract_ver}"
context: "{context}"
capabilities:
  - "{cap}"
hash_signature: "{hash_sig}"
---

# Acción: {name}

{desc}
"""
    action_path.parent.mkdir(parents=True, exist_ok=True)
    action_path.write_text(body, encoding="utf-8")
    row = f"| {name} | `{action_uuid}` | {version} | {context} | {desc} | `{cap}` |"
    _append_row(repo / "SddIA" / "actions" / "index.md", row, name)
    return {
        "handoff_entity_uuid": action_uuid,
        "handoff_hash_signature_new": hash_sig,
        "handoff_hash_signature_old": None,
        "handoff_version": version,
    }


def _sha256_phases_integrity(phases: list[Any]) -> str:
    """Alineado con verify-process-integrity / recalc-process-hash-signatures."""
    import hashlib

    canon = json.dumps(phases, separators=(",", ":"), ensure_ascii=False, sort_keys=True)
    return f"sha256:{hashlib.sha256(canon.encode('utf-8')).hexdigest()}"


def _refresh_process_hash(repo: Path, process_path: Path) -> tuple[str | None, str]:
    fm = parse_frontmatter(process_path)
    old_hash = fm.get("hash_signature")
    phases = fm.get("phases")
    if not isinstance(phases, list):
        phases = [{"name": "Fase inicial", "intent": "update"}]
    new_hash = _sha256_phases_integrity(phases)
    text = process_path.read_text(encoding="utf-8")
    if isinstance(old_hash, str) and old_hash in text:
        text = text.replace(f"hash_signature: {old_hash}", f"hash_signature: {new_hash}", 1)
    else:
        text = re.sub(
            r"^hash_signature:\s*.+$",
            f"hash_signature: {new_hash}",
            text,
            count=1,
            flags=re.MULTILINE,
        )
    process_path.write_text(text, encoding="utf-8")
    return (str(old_hash) if old_hash else None, new_hash)


def run_process_forge(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    name = inputs.get("process_name") or inputs.get("entity_name")
    if not isinstance(name, str) or not name:
        raise ValueError("process_name requerido")
    process_path = repo / "SddIA" / "process" / f"{name}.md"
    lifecycle = inputs.get("lifecycle_operation", "create")

    if lifecycle == "update" and process_path.is_file():
        fm = parse_frontmatter(process_path)
        old_hash, new_hash = _refresh_process_hash(repo, process_path)
        version = str(fm.get("version") or inputs.get("process_version") or "1.0.0")
        return {
            "handoff_entity_uuid": fm.get("uuid"),
            "handoff_hash_signature_new": new_hash,
            "handoff_hash_signature_old": old_hash,
            "handoff_version": version,
        }

    skip = idempotent_forge_handoff(repo, process_path, lifecycle)
    if skip:
        return skip

    context = inputs.get("process_context", "ecosystem-evolution")
    version = inputs.get("process_version", "1.0.0")
    contract_ver = inputs.get("process_contract_version", "1.3.0")
    desc = inputs.get("process_description", f"Proceso {name}")
    phases = inputs.get("process_phases", [{"name": "Fase inicial", "intent": desc}])
    process_uuid = _uuid(repo)
    hash_sig = _sha256_canon(repo, {"process_phases": phases})

    body = f"""---
uuid: "{process_uuid}"
name: "{name}"
version: "{version}"
contract: "process-contract v{contract_ver}"
context: "{context}"
hash_signature: "{hash_sig}"
phases:
  - name: "Fase inicial"
    intent: "{desc}"
---

# {name}

{desc}
"""
    process_path.parent.mkdir(parents=True, exist_ok=True)
    process_path.write_text(body, encoding="utf-8")
    row = f"| {name} | {process_uuid} | {version} | {context} | — | {desc[:60]} |"
    _append_row(repo / "SddIA" / "process" / "index.md", row, name)
    return {
        "handoff_entity_uuid": process_uuid,
        "handoff_hash_signature_new": hash_sig,
        "handoff_hash_signature_old": None,
        "handoff_version": version,
    }


def run_agent_forge(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    name = inputs.get("agent_name") or inputs.get("entity_name")
    if not isinstance(name, str) or not name:
        raise ValueError("agent_name requerido")
    agent_path = repo / "SddIA" / "agents" / f"{name}.md"
    lifecycle = inputs.get("lifecycle_operation", "create")
    skip = idempotent_forge_handoff(repo, agent_path, lifecycle)
    if skip:
        return skip

    policies = inputs.get("allowed_policies", ["ecosystem-evolution"])
    version = inputs.get("agent_version", "1.0.0")
    contract_ver = inputs.get("agents_contract_version", "1.0.0")
    purpose = inputs.get("agent_purpose", f"Agente {name}")
    agent_uuid = _uuid(repo)
    hash_sig = _sha256_canon(repo, {"agent_name": name, "allowed_policies": policies})
    pol_str = ", ".join(f"`{p}`" for p in policies)

    body = f"""---
uuid: "{agent_uuid}"
name: "{name}"
version: "{version}"
contract: "agents-contract v{contract_ver}"
allowed_policies:
{chr(10).join(f'  - "{p}"' for p in policies)}
hash_signature: "{hash_sig}"
---

# Agente: {name}

{purpose}
"""
    agent_path.parent.mkdir(parents=True, exist_ok=True)
    agent_path.write_text(body, encoding="utf-8")
    row = (
        f"| `{name}.md` | `{agent_uuid}` | {name} | {version} | "
        f"agents-contract v{contract_ver} | {pol_str} |"
    )
    _append_row(repo / "SddIA" / "agents" / "index.md", row, name)
    return {
        "handoff_entity_uuid": agent_uuid,
        "handoff_hash_signature_new": hash_sig,
        "handoff_hash_signature_old": None,
        "handoff_version": version,
    }


def run_norm_forge(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    name = inputs.get("tactical_norm_name") or inputs.get("entity_name")
    if not isinstance(name, str) or not name:
        raise ValueError("tactical_norm_name requerido")
    norm_path = repo / "SddIA" / "library" / "norms" / f"{name}.md"
    lifecycle = inputs.get("lifecycle_operation", "create")
    skip = idempotent_forge_handoff(repo, norm_path, lifecycle)
    if skip:
        return skip

    version = inputs.get("tactical_norm_version", "1.0.0")
    friction = inputs.get("tactical_norm_friction", f"Norma {name}")
    author = inputs.get("tactical_norm_author", "laboratorio")
    scope = inputs.get("norm_scope", "agnostic")
    category = inputs.get("norm_category", "workflow")
    norm_uuid = _uuid(repo)
    hash_sig = _sha256_canon(repo, {"tactical_norm_name": name, "friction": friction, "scope": scope})

    body = f"""---
uuid: "{norm_uuid}"
name: "{name}"
version: "{version}"
nature: "tactical-norm"
author: "{author}"
scope: "{scope}"
category: "{category}"
hash_signature: "{hash_sig}"
---

## Directriz Core

{friction}
"""
    norm_path.parent.mkdir(parents=True, exist_ok=True)
    norm_path.write_text(body, encoding="utf-8")
    row = f"| `{name}.md` | `{norm_uuid}` | {name} | {version} | {scope} | {category} |"
    _append_row(repo / "SddIA" / "library" / "norms" / "index.md", row, name)
    return {
        "handoff_entity_uuid": norm_uuid,
        "handoff_hash_signature_new": hash_sig,
        "handoff_hash_signature_old": None,
        "handoff_version": version,
    }


def run_codex_forge(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    slug = inputs.get("domain_codex_slug") or inputs.get("entity_name")
    if not isinstance(slug, str) or not slug:
        raise ValueError("domain_codex_slug requerido")
    codex_path = repo / "SddIA" / "library" / "codexes" / f"{slug}.md"
    lifecycle = inputs.get("lifecycle_operation", "create")
    skip = idempotent_forge_handoff(repo, codex_path, lifecycle)
    if skip:
        return skip

    cname = inputs.get("domain_codex_name", slug)
    version = inputs.get("domain_codex_version", "1.0.0")
    author = inputs.get("domain_codex_author", "laboratorio")
    envs = inputs.get("target_environment", ["dev"])
    grade = inputs.get("domain_codex_certification_grade", "Pendiente")
    codex_uuid = _uuid(repo)
    env_str = ", ".join(str(e) for e in envs)
    hash_sig = _sha256_canon(repo, {"domain_codex_slug": slug, "target_environment": envs})

    body = f"""---
uuid: "{codex_uuid}"
name: "{cname}"
version: "{version}"
author: "{author}"
certification_grade: "{grade}"
target_environment:
{chr(10).join(f'  - "{e}"' for e in envs)}
hash_signature: "{hash_sig}"
---

# Códice: {cname}

Estrategia de dominio para {env_str}.
"""
    codex_path.parent.mkdir(parents=True, exist_ok=True)
    codex_path.write_text(body, encoding="utf-8")
    row = f"| `{slug}.md` | `{codex_uuid}` | {cname} | {version} | {env_str} | {grade} |"
    _append_row(repo / "SddIA" / "library" / "codexes" / "index.md", row, slug)
    return {
        "handoff_entity_uuid": codex_uuid,
        "handoff_hash_signature_new": hash_sig,
        "handoff_hash_signature_old": None,
        "handoff_version": version,
    }


def run_suite_forge(repo: Path, inputs: dict[str, Any]) -> dict[str, Any]:
    name = inputs.get("suite_name") or inputs.get("entity_name")
    if not isinstance(name, str) or not name:
        raise ValueError("suite_name requerido")
    suite_path = repo / "SddIA" / "suites" / f"{name}.md"
    lifecycle = inputs.get("lifecycle_operation", "create")
    skip = idempotent_forge_handoff(repo, suite_path, lifecycle)
    if skip:
        return skip

    strategy = inputs.get("execution_strategy", "run_all")
    if strategy not in ("fail_fast", "run_all"):
        raise ValueError("execution_strategy debe ser fail_fast o run_all")
    atomic_nodes = inputs.get("atomic_nodes")
    if not isinstance(atomic_nodes, list) or not atomic_nodes:
        raise ValueError("atomic_nodes no vacío requerido")

    context = inputs.get("suite_context", "chaos-engineering")
    version = inputs.get("suite_version", "1.0.0")
    contract_ver = inputs.get("suites_contract_version", "1.0.0")
    suite_uuid = _uuid(repo)
    hash_sig = _sha256_canon(
        repo,
        {
            "atomic_nodes": atomic_nodes,
            "execution_strategy": strategy,
            "version": version,
        },
    )

    nodes_yaml = "\n".join(
        f"- process_name: {n.get('process_name')}\n"
        f"  expected_exit_code: {n.get('expected_exit_code', 0)}\n"
        f"  timeout_ms: {n.get('timeout_ms', 120000)}"
        for n in atomic_nodes
        if isinstance(n, dict) and n.get("process_name")
    )
    ctx_list = context if isinstance(context, list) else [context]
    ctx_yaml = "\n".join(f"- {c}" for c in ctx_list)

    body = f"""---
uuid: "{suite_uuid}"
name: {name}
version: "{version}"
contract: suites-contract v{contract_ver}
context:
{ctx_yaml}
hash_signature: {hash_sig}
execution_strategy: {strategy}
atomic_nodes:
{nodes_yaml}
---

# {name}

Suite forjada por suite-creator (laboratorio SddIA).
"""
    suite_path.parent.mkdir(parents=True, exist_ok=True)
    suite_path.write_text(body, encoding="utf-8")
    node_count = len([n for n in atomic_nodes if isinstance(n, dict) and n.get("process_name")])
    row = (
        f"| `{name}.md` | `{suite_uuid}` | {name} | {version} | {strategy} | {node_count} | "
        f"Suite forjada ({name}). |"
    )
    _append_row(repo / "SddIA" / "suites" / "index.md", row, name)
    return {
        "handoff_entity_uuid": suite_uuid,
        "handoff_hash_signature_new": hash_sig,
        "handoff_hash_signature_old": None,
        "handoff_version": version,
    }


FORGE_BY_ENTITY_CLASS: dict[str, Callable[[Path, dict[str, Any]], dict[str, Any]]] = {
    "tool": run_tool_forge,
    "action": run_action_forge,
    "process": run_process_forge,
    "agent": run_agent_forge,
    "norm": run_norm_forge,
    "codex": run_codex_forge,
    "suite": run_suite_forge,
}
