---
feature_name: route-domain-event-enetunreach-41717b4bb229
created: "2026-09-08"
process: bug-fix
branch: fix/route-domain-event-enetunreach-41717b4bb229
branch_name: fix/route-domain-event-enetunreach-41717b4bb229
persist_ref: docs/fixes/route-domain-event-enetunreach-41717b4bb229
pbi_ref: docs/todos/done/[FIX] route-domain-event — fractura sistémica (41717b4bb229).md
document_id: PBI-FIX-FRACTURE-41717b4bb229
execution_id: "7065b7fe-c55c-43cd-876c-78dfca1f99b1"
global: APTO
pbi_archived: true
pr_url: "https://github.com/racso80es/SddIA/pull/273"
ci_run_id: "34234850351"
ci_run_url: "https://github.com/racso80es/SddIA/actions/runs/34234850351"
checks:
  DLT-NET-CA1: APTO
  DLT-NET-CA2: APTO
  DLT-NET-CA3: APTO
  DLT-NET-CA4: PENDIENTE_INSTANCIA
  CA-CI: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - docs/fixes/route-domain-event-enetunreach-41717b4bb229/
  - docs/todos/done/[FIX] route-domain-event — fractura sistémica (41717b4bb229).md
  - SddIA/evolution/57e1dcd8-c04c-4818-818c-fb04ecf046e2.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — fractura `41717b4bb229`

## Veredicto

**APTO** — predicado DLT transitorio suprime Kintsugi; cubo Mayeuta `process_fix`; CA-CI run [34234850351](https://github.com/racso80es/SddIA/actions/runs/34234850351) `pull_request` SUCCESS (`sddia-index-integrity`, `wasi-runtime-smoke`, `eda-bus-e2e-smoke`, `eda-iota-physical`, `eda-iota-smoke-simulate`).

CA4: cápsula `SIMULATE=0` reprodujo `cause: ENETUNREACH` (F1 ambiental). No simular. Health ≠ publish.

## Checks

| Check | Estado | Evidencia |
|-------|--------|-----------|
| DLT-NET-CA1 | APTO | `emit_dlt_batch_fracture_suppressed_on_enetunreach` |
| DLT-NET-CA2 | APTO | `emit_dlt_batch_fracture_publish_error_friction` |
| DLT-NET-CA3 | APTO | `analyze_fracture_kaizen_dlt_publish_error_not_prompt` |
| DLT-NET-CA4 | PENDIENTE_INSTANCIA | cápsula exit 1 `ENETUNREACH`; sin digest |
| CA-CI | APTO | run [34234850351](https://github.com/racso80es/SddIA/actions/runs/34234850351) SUCCESS |
