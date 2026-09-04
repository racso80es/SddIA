---
feature_name: arquitectura-llm-tiers
created: "2026-09-04"
process: feature
phase: validate
agents: argos
branch: feat/arquitectura-llm-tiers
branch_name: feat/arquitectura-llm-tiers
persist_ref: docs/features/arquitectura-llm-tiers
pbi_ref: "docs/todos/done/[ARQUITECTURA] Inyección de perfiles LLM (Tiers) en contratos de agentes SddIA y Resolución Dinámica.md"
document_id: PBI-ARQUITECTURA-LLM-TIERS
uuid: "8a3b5c7d-4e2f-41a9-9b6e-7c1d3e5f8a2b"
global: APTO
pbi_archived: true
execution_id: "ff0407a0-0458-4461-acc3-1beeb94e1aa0"
checks:
  CA_CONTRACT: APTO
  CA_SEVEN_AGENTS: APTO
  CA_ARGOS_SYNTHESIS: APTO
  CA_PAYLOAD: APTO
  CA_NONE_NO_SPAWN: APTO
  CA_FAIL_SOFT: APTO
  CA_HARNESS: APTO
  CA_STARTER_KIT: APTO
  CA_LLM_INTERACT_INTACT: APTO
  CA_PBI_ARCHIVED: APTO
  CA_EVOLUTION: APTO
git_changes:
  - SddIA/agents/agents-contract.md
  - SddIA/agents/index.md
  - SddIA/agents/argos.md
  - SddIA/agents/cerbero.md
  - SddIA/agents/cumulo.md
  - SddIA/agents/dedalo.md
  - SddIA/agents/mayeuta.md
  - SddIA/agents/radamanto.md
  - SddIA/agents/tekton.md
  - SddIA/engine/execute-process/src/engine/agent_runtime.rs
  - SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
  - .gitignore
  - SddIA/scripts/starter-kit/.dev/.env.example
  - SddIA/scripts/starter-kit/.SddIA/.dev/.env.example
  - SddIA/evolution/2d5df89a-7de9-46a9-bc1c-fda95edcbc2b.md
  - SddIA/evolution/Evolution_log.md
  - docs/features/arquitectura-llm-tiers/
  - docs/todos/done/[ARQUITECTURA] Inyección de perfiles LLM (Tiers) en contratos de agentes SddIA y Resolución Dinámica.md
---

# Validación — arquitectura-llm-tiers

`global: APTO`. PBI archivado en `docs/todos/done/`. Runtime: 3 tests `llm_profile` + selftest harness. `llm:interact` no rebindado.

## Checks

| CA | Veredicto | Evidencia |
|----|-----------|-----------|
| Contrato | APTO | `agents-contract.md` v1.1.0 §5 `llm_profile` |
| Siete agentes | APTO | YAML `tier` según matriz PBI v1.3.0; UUID intactos |
| Argos síntesis | APTO | L-ARGOS-SYNTHESIS en `argos.md` |
| Payload | APTO | `llm_profile_high_is_injected_in_payload` |
| Veto none | APTO | `llm_profile_none_does_not_spawn` |
| Fail-soft | APTO | `llm_profile_missing_agent_md_is_fail_soft` |
| Harness | APTO | `SDDIA_HARNESS_SELFTEST=1` → `resolve_phase_model ok` |
| Starter-kit | APTO | `SDDIA_LLM_TIER_*` comentadas, vacías |
| llm:interact | APTO | `capability-bindings.md` no tocado |
| PBI archivado | APTO | `docs/todos/done/[ARQUITECTURA] Inyección de perfiles LLM (Tiers)…md` |
| Evolution | APTO | `sddia-qa evolution-register` `2d5df89a-7de9-46a9-bc1c-fda95edcbc2b` `EVOL_OK` |
