---
feature_name: kalma2-branch-before-mutate
created: "2026-10-14"
process: bug-fix
branch_name: fix/kalma2-branch-before-mutate
persist_ref: docs/fixes/kalma2-branch-before-mutate
document_id: PBI-FIX-KALMA2-BRANCH-AND-STATUS
---

# Spec — kalma2-branch-before-mutate

| ID | Criterio | Implementación |
|----|----------|----------------|
| BR-CA1 | Checkout de `branch_name` como primer paso git | `checkout_work_branch` antes de `dirty_paths_outside_scope` |
| BR-CA2 | Checkout fallido → sin mutación posterior en base | `Err` inmediato; sin `persist_ref` si aborta dirty tras rama |
| BR-CA3 | Init `failed` → barrier como agente `failed` | `barrier_from_phase_entry` + `workspace_init_failed` |
| BR-CA4 | Fin `executed`/`blocked` en `/api/status` | `latest_phase_end_in` + proyección sobre PEC `awaiting_agents` |
| BR-CA5 | Veredicto en mensaje operador | `short_verdict` + `emit_progress_trace` en fin de fase |
| BR-CA6 | Tests | `workspace_init`, `executor`, `kalma2-bridge` |
