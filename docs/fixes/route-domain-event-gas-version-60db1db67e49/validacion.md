---
feature_name: route-domain-event-gas-version-60db1db67e49
created: "2026-09-09"
process: bug-fix
branch: fix/route-domain-event-gas-version-60db1db67e49
branch_name: fix/route-domain-event-gas-version-60db1db67e49
persist_ref: docs/fixes/route-domain-event-gas-version-60db1db67e49
pbi_ref: docs/todos/done/[FIX] route-domain-event — fractura sistémica (60db1db67e49).md
document_id: PBI-FIX-FRACTURE-60db1db67e49
execution_id: "468e7f10-99aa-4c91-8803-e06e229742a3"
global: APTO
pbi_archived: true
pr_url: "https://github.com/racso80es/SddIA/pull/277"
ci_run_id: "34332282861"
ci_run_url: "https://github.com/racso80es/SddIA/actions/runs/34332282861"
checks:
  DLT-GAS-CA1: APTO
  DLT-GAS-CA2: APTO
  DLT-GAS-CA3: APTO
  DLT-GAS-CA4: APTO
  MAYEUTA-CA5: APTO
  MAYEUTA-CA6: APTO
  MAYEUTA-CA7: APTO
  CA-CI: APTO
git_changes:
  - .SddIA/services/iota-publish-relay/publish-queue.mjs
  - .SddIA/services/iota-publish-relay/server.mjs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/actions/enrich-fracture-pbi-kaizen.md
  - docs/fixes/route-domain-event-gas-version-60db1db67e49/
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (60db1db67e49).md
  - SddIA/evolution/5f09da5c-a96c-4475-9ce6-15d55cc5840c.md
  - SddIA/evolution/11409c1e-0254-4eea-8d5a-c1c2f5b1b4bb.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — fractura `60db1db67e49`

## Veredicto

**APTO** — gates locales + CA-CI run [34332282861](https://github.com/racso80es/SddIA/actions/runs/34332282861) `pull_request` SUCCESS (`sddia-index-integrity`, `wasi-runtime-smoke`, `eda-bus-e2e-smoke`, `eda-iota-physical`, `eda-iota-smoke-simulate`). Head `f2faea1`.

Canal LLM asíncrono = `PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION` (`docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md`). Fuera.

## Checks

| Check | Estado | Evidencia |
|-------|--------|-----------|
| DLT-GAS-CA1 | APTO | `publish-queue.test.mjs` (cola no solapa) |
| DLT-GAS-CA2 | APTO | `dlt_transient_gas_version_trace` + `emit_dlt_batch_fracture` (wrapper sin firma sí emite) |
| DLT-GAS-CA3 | APTO | `analyze_fracture_kaizen_dlt_gas_version_not_transport` |
| DLT-GAS-CA4 | APTO | digest `5rkFWghseVYgDh5DTQsECyeRS9T1d99hkkBXa7ELoEja` (`SIMULATE=0`, cápsula) |
| MAYEUTA-CA5 | APTO | `analyze_fracture_kaizen_generic_failed_not_prompt` |
| MAYEUTA-CA6 | APTO | `analyze_fracture_kaizen_dlt_publish_error_not_prompt` |
| MAYEUTA-CA7 | APTO | grep módulo: cero `llm:interact` / `mayeuta-llm` |
| CA-CI | APTO | run [34332282861](https://github.com/racso80es/SddIA/actions/runs/34332282861) SUCCESS |
