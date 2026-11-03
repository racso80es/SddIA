---
feature_name: aut-50-backlog-alta
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-50-backlog-alta
tracker_ref: AUT-50
execution_id: "82abfda6-47a2-41f6-855f-bddbc8546bda"
---

# Implementación — AUT-50

| Tarea | Entrega |
|-------|---------|
| T1 | `resolve_parent_ref` (`parent_ref` \| `historia_ref` issue); fallo `PARENT_REF_REQUIRED` con tracker; tests AUT50-CA1/CA2 |
| T2 | `list_issues`: `$projectId: ID!`, `first` default 20, sin `children` en query |
| T3 | `forge-hu` (entity-manager) + handler `forge_hu.rs` + `HU_Forged`; test AUT50-CA5 |
| T4 | `StoreError` Display; open budget 10s; `refine-item` lock → fractura / `refined_event_pending` |
