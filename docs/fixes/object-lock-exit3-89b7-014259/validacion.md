---
feature_name: object-lock-exit3-89b7-014259
created: "2026-09-25"
process: bug-fix
branch: fix/object-lock-exit3-89b7-014259
branch_name: fix/object-lock-exit3-89b7-014259
persist_ref: docs/fixes/object-lock-exit3-89b7-014259
pbi_ref: docs/todos/done/[FIX] route-domain-event — fractura sistémica (89b7c8b105ec).md
sibling_pbi_ref: docs/todos/done/[FIX] kalma2-bridge — fractura sistémica (0142599491fb).md
document_id: PBI-FIX-FRACTURE-89b7c8b105ec
execution_id: "2b45bcaf-60ea-46bc-ae41-cb2102d92925"
global: APTO
pbi_archived: true
pr_url: "https://github.com/racso80es/SddIA/pull/295"
ci_run_id: "36101108875"
ci_run_url: "https://github.com/racso80es/SddIA/actions/runs/36101108875"
checks:
  DLT-LOCK-CA1: APTO
  DLT-LOCK-CA2: APTO
  MAYEUTA-LOCK-CA3: APTO
  KALMA-EXIT3-CA1: APTO
  MAYEUTA-EXIT3-CA2: APTO
  CA-CI: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/actions/enrich-fracture-pbi-kaizen.md
  - SddIA/actions/index.md
  - SddIA/core/eda-coverage.json
  - SddIA/evolution/c9c0206e-aed4-4e48-a17d-b28d1de43d46.md
  - SddIA/evolution/Evolution_log.md
  - docs/fixes/object-lock-exit3-89b7-014259/
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (89b7c8b105ec).md
  - docs/todos/done/[FIX] kalma2-bridge — fractura sistémica (0142599491fb).md
---

# Validación — sellos `89b7c8b105ec` y `0142599491fb`

## Veredicto

**APTO.** Causas distintas, misma clase de sobre-escalado. CI `pull_request` [36101108875](https://github.com/racso80es/SddIA/actions/runs/36101108875) SUCCESS en head `462bdc0`: `sddia-index-integrity`, `wasi-runtime-smoke`, `eda-bus-e2e-smoke`, `eda-iota-physical`, `eda-iota-smoke-simulate`.

## Checks

| Check | Estado | Evidencia |
|-------|--------|-----------|
| DLT-LOCK-CA1 | APTO | `emit_dlt_batch_fracture_suppressed_on_object_lock` (pending sin fractura; cola `dlt_reanchor` escrita) |
| DLT-LOCK-CA2 | APTO | `emit_dlt_batch_fracture_suppressed_on_enetunreach`, `emit_dlt_batch_fracture_suppressed_on_gas_version_mismatch`, `emit_dlt_batch_fracture_issues_inputs_without_version_still_emits`, `emit_dlt_batch_fracture_publish_error_friction` |
| MAYEUTA-LOCK-CA3 | APTO | `analyze_fracture_kaizen_dlt_object_lock_not_opaque` |
| KALMA-EXIT3-CA1 | APTO | `prosthetic_exit_3_does_not_emit_fracture` |
| MAYEUTA-EXIT3-CA2 | APTO | `analyze_fracture_kaizen_prosthetic_exit3_not_unclassified` |
| CA-CI | APTO | run [36101108875](https://github.com/racso80es/SddIA/actions/runs/36101108875) |

## Fuera

Cola serial del relay. Predicados de gas y de red. ELF ausente `64f37c7f7b34`. Canal LLM asíncrono.
