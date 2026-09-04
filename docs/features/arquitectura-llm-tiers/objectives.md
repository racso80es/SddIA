---
feature_name: arquitectura-llm-tiers
created: "2026-09-04"
process: feature
branch_name: feat/arquitectura-llm-tiers
persist_ref: docs/features/arquitectura-llm-tiers
pbi_ref: docs/todos/pending/PBI-ARQUITECTURA-LLM-TIERS.md
execution_id: "ff0407a0-0458-4461-acc3-1beeb94e1aa0"
document_id: PBI-ARQUITECTURA-LLM-TIERS
pbi_uuid: "8a3b5c7d-4e2f-41a9-9b6e-7c1d3e5f8a2b"
pbi_version: "1.3.0"
status: in-progress
---

# Objetivos — arquitectura-llm-tiers

## Misión

Declarar `llm_profile` abstracto en el genoma de agentes; inyectarlo en el payload `AGENT_PHASE`; vetar spawn LLM en fases solo `none`; resolver modelo físico en el harness de forja vía `SDDIA_LLM_TIER_*` con fallback `SDDIA_AGENT_RUNTIME_MODEL`.

## Alcance

- Ciclo `feature` `execution_id` `ff0407a0-0458-4461-acc3-1beeb94e1aa0`.
- `agent_runtime.rs` (crate orquestador; no ED indexada).
- YAML de `SddIA/agents/{name}.md` + `agents-contract.md` (parche quirúrgico: `entity-manager update` de agente regenera UUID).
- Harness `SddIA/scripts/tools/kalma2-agent-runtime-cursor.py`.
- Starter-kit `.env.example`: vars comentadas, vacías.

## Fuera

Rebind `llm:interact`; `kalma2-bridge`; `gemini-http-infer` como router de fases `agent:`; kitchen `PBI-MULTI-LLM-ROUTER`.

## Ley aplicada

- Git vía `skill:git-manager`. Troncal `main`.
- DA-2/DA-4: topología `objectives.md` en rama. UUID de agentes inmutable.
- Agnosticismo de proveedor: cero slugs comerciales en genoma.
