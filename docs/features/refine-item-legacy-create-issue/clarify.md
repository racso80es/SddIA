---
feature_name: refine-item-legacy-create-issue
created: "2026-11-01"
purpose: contraste del sello 2026-10-29 con HEAD aut-35
process: refactorization
branch_name: aut-35
tracker_ref: AUT-35
execution_id: "27ab6def-6796-4baa-a3e6-c569f96efd72"
---

# Clarificación — AUT-35

La rama `refactor/refine-item-tracker-ref-remote` no está en el clon. Sus commits son ancestros de `main`. No había ref local `aut-35`; esta rama sale de `Fix_Events_Multiple` en `c008493b` para incluir el `refine_item/run.rs` posterior a AUT-34.

El texto Linear del 2026-10-29 narraba el defecto ya corregido en el handler productivo. El residuo medible es `try_create_hu_issue` en `refine.rs` y el test que exige `create_issue_invoked: true`.

`refine-hu` / `refine-pbi` ya responden `PROCESS_RENAMED`. `linear_direct_cycle_e2e` sigue llamando `refine::run_hu` y `refine::run_pbi` en proceso.
