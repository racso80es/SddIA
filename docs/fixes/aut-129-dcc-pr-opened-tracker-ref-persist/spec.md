---
feature_name: aut-129-dcc-pr-opened-tracker-ref-persist
created: "2026-11-28"
process: bug-fix
branch_name: fix/aut-129-dcc-pr-opened-tracker-ref-persist
persist_ref: docs/fixes/aut-129-dcc-pr-opened-tracker-ref-persist
pbi_ref: AUT-129
tracker_ref: AUT-129
execution_id: "f491f6fc-fa55-488a-8299-742b816a254c"
---

# Spec — AUT-129

## Problema

`apply_pr_opened_fail_soft` usa `build_cycle_payload`, que solo resolvía `tracker_ref` desde `pbi_ref`. El sello `emit-pr-presented-event` ya enriquecía vía `attach_tracker_ref_from_persist`, pero el milestone `PrOpened` recibía inputs sin `pbi_ref` (pre-push solo inyecta `persist_ref`), produciendo `tracker_ref ausente` y Linear sin transición a In Review.

## Solución

Tras la resolución por `pbi_ref`, si `tracker_ref` sigue ausente, resolver desde `persist_ref` con `resolve_tracker_ref_for_lifecycle` sobre un payload acotado `{ persist_ref }` (misma semántica que objectives / related_todo).

## Criterios de aceptación

| CA | Verificación |
| --- | --- |
| AC1 | `build_cycle_payload` + solo `persist_ref` + `objectives.md` con `tracker_ref` → payload con `tracker_ref` |
| AC2 | `apply_pr_opened_fail_soft` con provider Linear mueve issue a In Review (entorno integración) |
| AC3 | Paridad con `attach_tracker_ref_from_persist` / `resolve_tracker_ref_for_lifecycle` |
| AC4 | Test `build_cycle_payload_resolves_tracker_ref_from_persist_objectives_only` |