---
feature_name: arquitectura-llm-tiers
created: "2026-09-04"
process: feature
purpose: Estabilización PBI v1.3.0; relevo IDE post-init
version_clarify: "1.0.0"
execution_id: "ff0407a0-0458-4461-acc3-1beeb94e1aa0"
pbi_ref: docs/todos/pending/PBI-ARQUITECTURA-LLM-TIERS.md
document_id: PBI-ARQUITECTURA-LLM-TIERS
pbi_uuid: "8a3b5c7d-4e2f-41a9-9b6e-7c1d3e5f8a2b"
---

# Clarificación — arquitectura-llm-tiers

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery + `SDDIA_LAB_ALLOW_DIRTY=1`. `execution_id` `ff0407a0-0458-4461-acc3-1beeb94e1aa0`. Rama `feat/arquitectura-llm-tiers`. Mayeuta simulated; Dedalo…DCC phase-barrier. Relevo IDE.

## Decisiones (PBI v1.3.0, sin laudo nuevo)

| ID | Laudo |
|----|-------|
| L-TIER-LEVELS | `high` Dedalo/Mayeuta; `medium` Argos síntesis; `low` Tekton; `none` Cerbero/Cúmulo/Radamanto. |
| L-NONE-NO-SPAWN | Fase solo `none` no spawnea `SDDIA_AGENT_RUNTIME_COMMAND`. |
| L-ARGOS-SYNTHESIS | Medium = informe/blueprint tras evidencia de cápsulas. No LLM-linter. |
| L-FORGE-UUID | `agent-creator` update reescribe UUID. Parche YAML preservando `uuid`. |
| L-RESOLVE-SURFACE | Harness Python mapea tier→env. CLI Cursor: sin flags inventados; SDK sí recibe modelo. |
| L-ORTHOGONAL-INTERACT | `llm:interact` / `mayeuta-llm` intactos. |
| L-PAYLOAD-EXTEND | Extender `AGENT_PHASE` vigente; no esquema nuevo. |

## Fuera

`kalma2-bridge`; kitchen multi-proveedor; slugs eternos en examples.
