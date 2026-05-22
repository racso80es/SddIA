#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""CLI de laboratorio: intérprete universal de Acciones de Dominio SddIA.

Carga el contrato SddIA/actions/{action}.md y delega en handlers físicos
y tools asignadas (p. ej. markdown-table-editor para Cúmulo).

Uso:
  python SddIA/scripts/qa/execute-action.py --action sync-entity-index --inputs '{"entity_class":"skill",...}'
  echo '{"entity_class":"skill",...}' | python SddIA/scripts/qa/execute-action.py --action sync-entity-index
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

try:
    import yaml
except ImportError:
    yaml = None  # type: ignore

SCRIPT = Path(__file__).resolve()
_QA_DIR = SCRIPT.parent
if str(_QA_DIR) not in sys.path:
    sys.path.insert(0, str(_QA_DIR))

from eda_bus_utils import (  # noqa: E402
    find_existing_domain_event,
    resolve_origin_topology,
)
from env_loader import load_hierarchical_env  # noqa: E402

INDEX_MAP: dict[str, str] = {
    "process": "SddIA/process/index.md",
    "agent": "SddIA/agents/index.md",
    "skill": "SddIA/skills/index.md",
    "tool": "SddIA/tools/index.md",
    "action": "SddIA/actions/index.md",
    "codex": "SddIA/library/codexes/index.md",
}

ACTION_AGENT: dict[str, str] = {
    "sync-entity-index": "cumulo",
    "emit-pr-merged-event": "eda-bus",
    "emit-pr-presented-event": "eda-bus",
    "emit-domain-mutation": "eda-bus",
    "materialize-fracture-pbi": "cumulo",
    "enrich-fracture-pbi-kaizen": "mayeuta",
}


def _crypto(repo: Path, payload: dict[str, Any]) -> Any:
    crypto_script = repo / "scripts" / "skills" / "cryptography-manager.py"
    proc = subprocess.run(
        [sys.executable, str(crypto_script)],
        input=json.dumps(payload),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )
    out = json.loads(proc.stdout or "{}")
    if not out.get("success"):
        raise RuntimeError(out.get("error") or proc.stderr or "cryptography-manager failed")
    return out["data"]["result"]


def _load_cumulo(repo: Path) -> dict[str, Any]:
    return json.loads((repo / "SddIA" / "core" / "cumulo.paths.json").read_text(encoding="utf-8"))


def _write_pending_event(repo: Path, event: dict[str, Any]) -> dict[str, str]:
    cumulo = _load_cumulo(repo)
    pending_rel = cumulo.get("eda_bus", {}).get("pending", "docs/events/pending")
    pending = repo / pending_rel
    pending.mkdir(parents=True, exist_ok=True)
    event_id = event["event_id"]
    target = pending / f"{event_id}.json"
    target.write_text(json.dumps(event, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    return {
        "event_id": event_id,
        "target_path": str(target.relative_to(repo)).replace("\\", "/"),
    }


def _run_emit_pr_merged(repo: Path, inputs: dict[str, Any], action_def: dict[str, Any]) -> dict[str, Any]:
    _ = action_def
    merge_hash = inputs.get("merge_commit_hash") or inputs.get("hash_signature")
    if not isinstance(merge_hash, str) or not merge_hash.strip():
        raise ValueError("merge_commit_hash o hash_signature es obligatorio")

    source_branch = inputs.get("source_branch")
    if not isinstance(source_branch, str) or not source_branch.strip():
        pr_url = inputs.get("pr_url", "")
        if isinstance(pr_url, str) and "feature/" in pr_url:
            source_branch = pr_url.rsplit("/", 1)[-1]
        else:
            source_branch = "feature/eda-bus-v1"

    correlation_id = inputs.get("correlation_id")
    if not isinstance(correlation_id, str) or not correlation_id.strip():
        correlation_id = _crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})

    event_id = _crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    payload: dict[str, Any] = {
        "source_branch": source_branch.strip(),
        "target_branch": "main",
        "merge_commit_hash": merge_hash.strip(),
        "author": inputs.get("author", "integration-operator"),
        "security_clearance": {
            "auditor": "Argos",
            "audit_event_reference": "TODO: pending_argos_eda_emission",
            "policy_applied": "pr-acceptance-protocol",
        },
    }
    if inputs.get("pr_url"):
        payload["pr_url"] = inputs["pr_url"]
    if inputs.get("repository_name"):
        payload["repository_name"] = inputs["repository_name"]
    if inputs.get("hash_signature"):
        payload["hash_signature"] = inputs["hash_signature"]
    if inputs.get("traceability_anomaly"):
        payload["traceability_anomaly"] = inputs["traceability_anomaly"]
    if inputs.get("traceability_note"):
        payload["traceability_note"] = inputs["traceability_note"]

    event = {
        "event_id": event_id,
        "event_type": "PullRequest_Merged",
        "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "emitter_agent": inputs.get("emitter_agent", "emit-pr-merged-event"),
        "correlation_id": correlation_id,
        "payload": payload,
        "delivery_state": {},
    }
    seal = _write_pending_event(repo, event)
    return {
        "success": True,
        "event_id": seal["event_id"],
        "target_path": seal["target_path"],
        "event_type": "PullRequest_Merged",
        "merge_commit_hash": merge_hash.strip(),
    }


def _run_emit_pr_presented(repo: Path, inputs: dict[str, Any], action_def: dict[str, Any]) -> dict[str, Any]:
    _ = action_def
    branch = inputs.get("branch")
    status = inputs.get("status", "presented")
    if not isinstance(branch, str) or not branch.strip():
        raise ValueError("branch es obligatorio (string)")

    event_id = _crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    payload: dict[str, Any] = {"branch": branch.strip(), "status": status}
    pr_url = inputs.get("pr_url")
    if isinstance(pr_url, str) and pr_url.strip():
        payload["pr_url"] = pr_url.strip()

    event: dict[str, Any] = {
        "event_id": event_id,
        "event_type": "PullRequest_Presented",
        "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "emitter_agent": inputs.get("emitter_agent", "delivery-close-cycle"),
        "payload": payload,
        "delivery_state": {},
    }
    correlation_id = inputs.get("correlation_id")
    if isinstance(correlation_id, str) and correlation_id.strip():
        event["correlation_id"] = correlation_id.strip()
    seal = _write_pending_event(repo, event)
    return {
        "success": True,
        "event_id": seal["event_id"],
        "target_path": seal["target_path"],
        "event_type": "PullRequest_Presented",
    }


def _repo_root() -> Path:
    for parent in SCRIPT.parents:
        if (parent / "SddIA" / "core" / "cumulo.paths.json").is_file():
            return parent
    raise RuntimeError("No se encontró raíz del workspace (SddIA/core/cumulo.paths.json)")


def _emit(envelope: dict[str, Any], code: int | None = None) -> None:
    if code is None:
        code = 0 if envelope.get("success") else 1
    envelope.setdefault("exitCode", code)
    sys.stdout.write(json.dumps(envelope, ensure_ascii=False) + "\n")
    sys.exit(code)


def _resolve_action_path(repo: Path, action_name: str) -> Path:
    actions_dir = repo / "SddIA" / "actions"
    direct = actions_dir / f"{action_name}.md"
    if direct.is_file():
        return direct
    if yaml is None:
        raise RuntimeError("PyYAML requerido para resolver aliases de acción")
    for md in actions_dir.glob("*.md"):
        if md.stem in ("index", "actions-contract"):
            continue
        try:
            fm = yaml.safe_load(md.read_text(encoding="utf-8").split("---", 2)[1])
        except (IndexError, yaml.YAMLError):
            continue
        if isinstance(fm, dict) and fm.get("name") == action_name:
            return md
    raise FileNotFoundError(f"Acción no encontrada: {action_name}")


def _load_action_def(repo: Path, action_name: str) -> dict[str, Any]:
    path = _resolve_action_path(repo, action_name)
    if yaml is None:
        return {"name": path.stem}
    fm = yaml.safe_load(path.read_text(encoding="utf-8").split("---", 2)[1])
    return fm if isinstance(fm, dict) else {"name": path.stem}


def _invoke_bus_operator(repo: Path, operation: str, operation_payload: dict[str, Any]) -> dict[str, Any]:
    skill_script = repo / "scripts" / "skills" / "bus-operator.py"
    if not skill_script.is_file():
        raise FileNotFoundError(str(skill_script))
    req = {"operation": operation, "operation_payload": operation_payload}
    proc = subprocess.run(
        [sys.executable, str(skill_script)],
        input=json.dumps(req, ensure_ascii=False),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        cwd=str(repo),
        check=False,
    )
    stdout = (proc.stdout or "").strip()
    if not stdout:
        raise RuntimeError(proc.stderr or "bus-operator sin salida")
    body = json.loads(stdout.splitlines()[-1])
    if not body.get("success"):
        raise RuntimeError(body.get("error") or "bus-operator failed")
    return body.get("result") or {}


def _run_emit_domain_mutation(repo: Path, inputs: dict[str, Any], action_def: dict[str, Any]) -> dict[str, Any]:
    _ = action_def
    entity_class = inputs.get("entity_class")
    entity_name = inputs.get("entity_name")
    lifecycle = inputs.get("lifecycle_operation")
    entity_uuid = inputs.get("entity_uuid")
    if not all(isinstance(x, str) for x in (entity_class, entity_name, lifecycle)):
        raise ValueError("entity_class, entity_name y lifecycle_operation son obligatorios")
    if lifecycle != "delete" and not entity_uuid:
        raise ValueError("entity_uuid obligatorio salvo delete")

    op = lifecycle
    event_type = {
        "create": "Domain_Entity_Created",
        "update": "Domain_Entity_Updated",
        "delete": "Domain_Entity_Deleted",
    }.get(op)
    if not event_type:
        raise ValueError(f"lifecycle_operation no soportada: {lifecycle}")

    origin_topology = inputs.get("origin_topology", "core")
    if origin_topology not in ("core", "local"):
        origin_topology = "core"

    if entity_uuid and lifecycle != "delete":
        existing = find_existing_domain_event(repo, entity_uuid, op, event_type)
        if existing and existing.get("event_id"):
            return {
                "success": True,
                "idempotent": True,
                "event_type": event_type,
                "event_id": existing["event_id"],
                "target_path": existing.get("target_path"),
            }

    event_id = _crypto(repo, {"operation": "GENERATE_UUID", "target_payload": None})
    event = {
        "event_id": event_id,
        "event_type": event_type,
        "timestamp": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "emitter_agent": inputs.get("emitter_agent", "entity-manager"),
        "payload": {
            "entity_class": entity_class,
            "lifecycle_operation": op,
            "entity_uuid": entity_uuid,
            "entity_name": entity_name,
            "version": inputs.get("version"),
            "hash_signature_new": inputs.get("hash_signature_new"),
            "hash_signature_old": inputs.get("hash_signature_old"),
            "origin_topology": origin_topology,
            "changes_summary": inputs.get(
                "changes_summary",
                f"{op} {entity_class} {entity_name}",
            ),
        },
        "delivery_state": {},
    }
    seal = _write_pending_event(repo, event)
    return {"success": True, "event_type": event_type, **seal}


def _run_sync_entity_index(repo: Path, inputs: dict[str, Any], action_def: dict[str, Any]) -> dict[str, Any]:
    _ = action_def
    entity_class = inputs.get("entity_class", "")
    entity_name = inputs.get("entity_name", "")
    lifecycle_operation = inputs.get("lifecycle_operation", "")

    if entity_class == "norm":
        return {
            "success": True,
            "target_index_path": None,
            "message": "Indexación ignorada para norm.",
        }

    origin_topology = resolve_origin_topology(dict(inputs))
    if origin_topology == "local":
        return {
            "success": True,
            "target_index_path": None,
            "message": "Indexación canónica omitida (origin_topology=local).",
        }

    rel_path = INDEX_MAP.get(str(entity_class))
    if not rel_path:
        return {
            "success": True,
            "target_index_path": None,
            "message": f"entity_class desconocida ({entity_class}): no-op.",
        }

    if not entity_name:
        raise ValueError("entity_name requerido")

    target_file = repo / rel_path
    if not target_file.is_file():
        return {
            "success": False,
            "target_index_path": rel_path,
            "message": f"Índice inexistente: {rel_path}",
        }

    if lifecycle_operation == "delete":
        result = _invoke_bus_operator(
            repo,
            "sync_entity_index",
            {
                "entity_class": entity_class,
                "entity_name": entity_name,
                "lifecycle_operation": lifecycle_operation,
            },
        )
        return {
            "success": True,
            "target_index_path": rel_path,
            "message": f"Fila purgada para {entity_name}."
            if result.get("rows_removed", 0) or result.get("modified")
            else f"Sin fila para purgar (idempotente): {entity_name}.",
        }

    if lifecycle_operation in ("create", "update"):
        result = _invoke_bus_operator(
            repo,
            "sync_entity_index",
            {
                "entity_class": entity_class,
                "entity_name": entity_name,
                "lifecycle_operation": lifecycle_operation,
            },
        )
        exists = bool(result.get("exists"))
        return {
            "success": exists,
            "target_index_path": rel_path,
            "message": (
                f"Auditoría OK: fila presente para {entity_name}."
                if exists
                else f"ALERTA: Fila no encontrada para {entity_name}."
            ),
        }

    raise ValueError(f"lifecycle_operation no soportada: {lifecycle_operation}")


def _slugify_process_name(name: str) -> str:
    slug = re.sub(r"[^\w\-]+", "-", name.strip().lower())
    slug = re.sub(r"-+", "-", slug).strip("-")
    return slug[:48] or "fracture"


def _fracture_trace_hash(error_trace: str) -> str:
    return hashlib.sha256(error_trace.strip().encode("utf-8")).hexdigest()[:12]


def _fracture_pbi_filename(process_name: str, error_trace: str) -> str:
    slug = _slugify_process_name(process_name)
    return f"[FIX] {slug} — fractura sistémica ({_fracture_trace_hash(error_trace)}).md"


def _fracture_pbi_path(repo: Path, process_name: str, error_trace: str) -> Path:
    return repo / "docs" / "todos" / "pending" / _fracture_pbi_filename(process_name, error_trace)


def _analyze_fracture_kaizen(
    process_name: str,
    error_trace: str,
    attempted_action: str,
    agent_emitter: str,
) -> tuple[str, str, str]:
    """Diagnóstico determinista lab → (veredicto, causa raíz, propuesta markdown)."""
    blob = f"{error_trace}\n{attempted_action}\n{process_name}".lower()
    root_causes: list[str] = []
    proposals: list[tuple[str, str]] = []

    if any(token in blob for token in ("recurs", "pre-push", "hook", "delivery-close", "re-entrada")):
        root_causes.append(
            "Recursión o re-entrada en la cadena hook Git ↔ proceso de cierre (`delivery-close-cycle`)."
        )
        proposals.append(
            (
                "refactor_tool",
                "Implementar guarda `SDDIA_HOOK_DELIVERY_CLOSE` y push interno con `SDDIA_SKIP_HOOKS=1` "
                "acotado al subproceso `git-manager`.",
            )
        )
    if any(token in blob for token in ("gh ", "gh pr", "git push", "git merge", "bypass", "skip_hooks", "curl ")):
        root_causes.append(
            "Violación de jurisdicción delegada: terminal raw usada para evadir cápsula o proceso oficial."
        )
        proposals.append(
            (
                "new_norm",
                "Reforzar `SddIA/norms/obediencia-procesos.md` § Ley de Jurisdicción Delegada; "
                "prohibir bypass silencioso ante fallo.",
            )
        )
    if any(token in blob for token in ("orphan", "ruido de sistema", "eda genómica", "huérfan")):
        root_causes.append(
            "Entidad genómica indexada sin correlato `Domain_Entity_Created` en bus EDA."
        )
        proposals.append(
            (
                "refactor_tool",
                "Ejecutar backfill Fase C (`audit-entity-eda-coverage --emit`) o integrar sello en `entity-manager` create.",
            )
        )
    if any(token in blob for token in ("timeout", "block", "abort", "failed", "colaps")):
        root_causes.append(
            "Bloqueo operativo sin escalado Kintsugi previo al intento de recuperación manual."
        )
        proposals.append(
            (
                "prompt_adjustment",
                "Ajustar instrucción operador IA: detener, emitir `System_Fracture_Detected`, "
                "notificar al Vértice Biológico — no continuar entrega.",
            )
        )

    if not root_causes:
        root_causes.append(
            f"Causa raíz no clasificada automáticamente para `{process_name}`; requiere laudo humano."
        )
        proposals.append(
            (
                "process_fix",
                f"Auditar proceso `{process_name}`, acción `{attempted_action}` y emisor `{agent_emitter}`.",
            )
        )

    verdict_priority = ("new_norm", "refactor_tool", "prompt_adjustment", "process_fix")
    verdict = proposals[0][0]
    for vp in verdict_priority:
        if any(p[0] == vp for p in proposals):
            verdict = vp
            break

    verdict_labels = {
        "new_norm": "Nueva norma o endurecimiento normativo",
        "refactor_tool": "Refactor de herramienta / cápsula / handler lab",
        "prompt_adjustment": "Ajuste de prompt o regla operador IA",
        "process_fix": "Corrección de proceso oficial",
    }

    proposal_md = "\n".join(f"- **{verdict_labels.get(p[0], p[0])}:** {p[1]}" for p in proposals)
    root_md = "\n".join(f"- {c}" for c in root_causes)

    section = f"""## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Mayeuta — Kintsugi async)*

### Diagnóstico de causa raíz

{root_md}

### Veredicto evolutivo

**{verdict_labels.get(verdict, verdict)}** (`{verdict}`)

### Propuestas

{proposal_md}

> Mayeuta transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar."""
    return verdict, root_md, section


def _upsert_fracture_kaizen_section(content: str, section: str) -> str:
    marker = "## Conclusión Analítica y Propuesta Evolutiva"
    placeholder = "_Pendiente de síntesis Mayeuta (Kintsugi async)._"
    if marker in content:
        before, _, after = content.partition(marker)
        if after.strip().startswith(placeholder):
            return before.rstrip() + "\n\n" + section + "\n"
        return re.sub(
            rf"{re.escape(marker)}[\s\S]*?(?=\n## |\Z)",
            section + "\n",
            content,
            count=1,
        )
    return content.rstrip() + "\n\n" + section + "\n"


def _run_materialize_fracture_pbi(
    repo: Path, inputs: dict[str, Any], action_def: dict[str, Any]
) -> dict[str, Any]:
    _ = action_def
    process_name = inputs.get("process_name")
    error_trace = inputs.get("error_trace")
    agent_emitter = inputs.get("agent_emitter")
    attempted_action = inputs.get("attempted_action")
    for key, val in (
        ("process_name", process_name),
        ("error_trace", error_trace),
        ("agent_emitter", agent_emitter),
        ("attempted_action", attempted_action),
    ):
        if not isinstance(val, str) or not val.strip():
            raise ValueError(f"{key} es obligatorio (string)")

    trace_hash = _fracture_trace_hash(error_trace)
    slug = _slugify_process_name(process_name)
    filename = _fracture_pbi_filename(process_name, error_trace)
    pending_dir = repo / "docs" / "todos" / "pending"
    pending_dir.mkdir(parents=True, exist_ok=True)
    target = pending_dir / filename
    rel_path = str(target.relative_to(repo)).replace("\\", "/")

    if target.is_file():
        return {
            "success": True,
            "target_path": rel_path,
            "message": "PBI ya existente (idempotente)",
        }

    persist_ref = inputs.get("persist_ref")
    branch_name = inputs.get("branch_name")
    today = datetime.now(timezone.utc).strftime("%Y-%m-%d")
    related_lines = [
        "  - SddIA/norms/obediencia-procesos.md",
        "  - SddIA/events/system-fracture-detected.md",
    ]
    if isinstance(persist_ref, str) and persist_ref.strip():
        related_lines.append(f"  - {persist_ref.strip()}")
    if isinstance(branch_name, str) and branch_name.strip():
        related_lines.append(f"  - branch: {branch_name.strip()}")

    body = f"""---
document_id: PBI-FIX-FRACTURE-{trace_hash}
title: "[FIX] {process_name.strip()} — fractura sistémica"
format: markdown
version: "1.0.0"
created: "{today}"
status: "abierto"
priority: alta
process: bug-fix
incident_ref: "System_Fracture_Detected — {trace_hash}"
related:
{chr(10).join(related_lines)}
---

# [FIX] {process_name.strip()} — fractura sistémica

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `{process_name.strip()}` |
| Emisor | `{agent_emitter.strip()}` |
| Acción intentada | `{attempted_action.strip()}` |

## Traza de error

```
{error_trace.strip()}
```

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado.

## Conclusión Analítica y Propuesta Evolutiva

_Pendiente de síntesis Mayeuta (Kintsugi async)._

## Criterio de cierre

- [ ] Causa raíz resuelta
- [ ] Argos APTO en `validacion.md` del fix
- [ ] Este TODO movido a `docs/todos/done/`
"""
    target.write_text(body, encoding="utf-8")
    return {
        "success": True,
        "target_path": rel_path,
        "message": "PBI materializado",
        "trace_hash": trace_hash,
    }


def _run_enrich_fracture_pbi_kaizen(
    repo: Path, inputs: dict[str, Any], action_def: dict[str, Any]
) -> dict[str, Any]:
    _ = action_def
    process_name = inputs.get("process_name")
    error_trace = inputs.get("error_trace")
    agent_emitter = inputs.get("agent_emitter")
    attempted_action = inputs.get("attempted_action")
    for key, val in (
        ("process_name", process_name),
        ("error_trace", error_trace),
        ("agent_emitter", agent_emitter),
        ("attempted_action", attempted_action),
    ):
        if not isinstance(val, str) or not val.strip():
            raise ValueError(f"{key} es obligatorio (string)")

    cumulo_path = inputs.get("cumulo_pbi_path")
    if isinstance(cumulo_path, str) and cumulo_path.strip():
        target = repo / Path(cumulo_path.strip())
    else:
        target = _fracture_pbi_path(repo, process_name, error_trace)

    if not target.is_file():
        raise FileNotFoundError(
            f"PBI de Cúmulo no encontrado: {target.relative_to(repo)} — ejecutar materialize-fracture-pbi antes"
        )

    verdict, _, section = _analyze_fracture_kaizen(
        process_name.strip(),
        error_trace.strip(),
        attempted_action.strip(),
        agent_emitter.strip(),
    )
    content = target.read_text(encoding="utf-8")
    target.write_text(_upsert_fracture_kaizen_section(content, section), encoding="utf-8")
    rel_path = str(target.relative_to(repo)).replace("\\", "/")
    return {
        "success": True,
        "target_path": rel_path,
        "message": "PBI enriquecido con síntesis Kaizen",
        "evolution_verdict": verdict,
    }


PHYSICAL_HANDLERS: dict[str, Any] = {
    "sync-entity-index": _run_sync_entity_index,
    "emit-pr-merged-event": _run_emit_pr_merged,
    "emit-pr-presented-event": _run_emit_pr_presented,
    "emit-domain-mutation": _run_emit_domain_mutation,
    "materialize-fracture-pbi": _run_materialize_fracture_pbi,
    "enrich-fracture-pbi-kaizen": _run_enrich_fracture_pbi_kaizen,
}


def run_action(repo: Path, action_name: str, action_inputs: dict[str, Any]) -> dict[str, Any]:
    action_def = _load_action_def(repo, action_name)
    canonical = action_def.get("name") or action_name
    handler = PHYSICAL_HANDLERS.get(canonical)
    if handler is None:
        return {
            "success": True,
            "status_code": 0,
            "data": {
                "action_name": canonical,
                "agent": ACTION_AGENT.get(canonical),
                "note": "contrato cargado; handler físico no implementado (simulado)",
            },
        }

    agent = ACTION_AGENT.get(canonical, "cumulo")
    data = handler(repo, action_inputs, action_def)
    inner_ok = data.get("success", True)
    return {
        "success": True,
        "status_code": 0,
        "data": {
            **data,
            "action_name": canonical,
            "delegated_agent": agent,
            "delegated_skill": "bus-operator" if canonical == "sync-entity-index" else None,
            "delegated_tool": "markdown-table-editor" if canonical == "sync-entity-index" else None,
        },
        "execution_report": {
            "action_name": canonical,
            "agent": agent,
            "business_success": inner_ok,
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser(description="execute-action (laboratorio SddIA)")
    parser.add_argument("--action", required=True, help="Nombre canónico de la acción")
    parser.add_argument("--inputs", help="JSON de inputs de la acción")
    args = parser.parse_args()

    try:
        if args.inputs:
            action_inputs = json.loads(args.inputs)
        else:
            stdin = sys.stdin.read()
            action_inputs = json.loads(stdin) if stdin.strip() else {}
        if not isinstance(action_inputs, dict):
            raise ValueError("inputs deben ser objeto JSON")
    except json.JSONDecodeError as e:
        _emit({"success": False, "error": f"JSON inválido: {e}"}, 1)

    try:
        repo = _repo_root()
        load_hierarchical_env(repo)
        result = run_action(repo, args.action.strip(), action_inputs)
        business = (result.get("data") or {}).get("success", True)
        if result.get("success") and business is False:
            result["status_code"] = 0
        _emit(result, result.get("status_code", 0))
    except Exception as e:
        _emit({"success": False, "status_code": 1, "error": str(e)}, 1)


if __name__ == "__main__":
    main()
