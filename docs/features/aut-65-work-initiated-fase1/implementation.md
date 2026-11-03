---
feature_name: aut-65-work-initiated-fase1
created: "2026-11-03"
process: refactorization
persist_ref: docs/features/aut-65-work-initiated-fase1
pbi_ref: AUT-65
---

# Implementación — AUT-65

## T1 — Motor

- `workspace_init::run`: emite `emit_work_initiated_fail_soft` al cerrar Fase 1 en `feature` / `bug-fix` / `refactorization`; marca `work_initiated_emitted`.
- `merge_work_initiated_workspace_success`: propaga resultado a `state` y al informe de fase.
- `maybe_emit_work_initiated_on_tekton_entry`: solo propaga `work_initiated` ya emitido; no crea un segundo evento.

## T2 — Contratos

- `feature.md` v1.3.3, `bug-fix.md` v1.4.3, `refactorization.md` v1.2.3: bullet Ignición tracker (AUT-65).
- `docs/audits/ciclo-vida-software-sddia.md`: diagrama y tabla EDA actualizados.

## T3 — Tests

- `workspace_init_emits_work_initiated_after_phase1`
- `workspace_init_respects_lab_skip_work_initiated`
- `maybe_emit_work_initiated_only_propagates_after_phase1`
