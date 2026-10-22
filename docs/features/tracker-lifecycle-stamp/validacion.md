---
feature_name: tracker-lifecycle-stamp
created: "2026-10-22"
updated: "2026-10-22T20:17:00+02:00"
process: feature
branch: feat/tracker-lifecycle-stamp
global: NO_APTO
pbi_archived: false
execution_id: "1d8790f8-abfe-4880-a3a5-ec0c9093812b"
pbi_ref: docs/todos/done/[ARQUITECTURA] HU-003 PBI-03 — tracker lifecycle.md
checks:
  AC-1:
    status: PENDIENTE-CAPSULA
    note: "Grep IDE: 0 hits de linear-tracker-adapter en tracker-stamp.md y tracker_stamp.rs. No sustituye rg/cápsula (L-ARGOS-SYNTHESIS)."
  AC-2:
    status: BLOQUEADO
    note: "Fuente contiene ac2_state_stamp_uses_live_op_and_schema y update_issue_state; Shell rechazado → cargo test / sello live no ejecutables."
  AC-3:
    status: PENDIENTE-CAPSULA
    note: "Lectura estática: tracker-stamp.md context: [tracker-operations]; requires_capability tracker:lifecycle. Sin evidencia de cápsula."
  runtime_integrity:
    status: FAIL
    note: "Handoff Kalma2: process/phase/persist_ref/branch_name/execution_id/pbi_ref/correlation_id vacíos (?). Shell IDE Rejected; ./sddia-run.sh, skill:git-manager y action:execute-process no invocables."
git_changes: []
approval_status: rejected
integrity_failure: true
---

# validacion.md — tracker-lifecycle-stamp

## Veredicto

**NO_APTO** — fallo de integridad Argos (sección 3 / L-ARGOS-SYNTHESIS). Sin `active_norm_pack` inyectado ni cápsulas ejecutables.

## Evidencia

| Fuente | Resultado |
|--------|-----------|
| Shell / `./sddia-run.sh` | Rejected (probe `echo` incluido) |
| `skill:git-manager` | No invocado |
| Grep IDE `linear-tracker-adapter` | 0 matches en proceso + handler |
| Lectura `tracker-stamp.md` | `context: tracker-operations`; `requires_capability: tracker:lifecycle` |
| `execution.md` (claim Tekton) | Afirma tests ok; Argos no re-ejecuta → no acredita AC-2 |
| PBI | `docs/todos/done/...PBI-03...` (`status: done`); cierre documental APTO pendiente |

## correction_blueprint_md

```yaml
phases:
  - name: Restaurar runtime de cápsulas
    intent: Habilitar Shell o inyección nativa execute-process para Argos.
    delegates_to:
      - action:execute-process
  - name: Evidencia git
    intent: Snapshot de cambios vía skill canónico.
    delegates_to:
      - skill:git-manager
  - name: Re-verificación AC
    intent: AC-1 ausencia linear-tracker-adapter; AC-2 cargo test ac2_state_stamp_uses_live_op_and_schema; AC-3 context tracker-operations.
    delegates_to:
      - agent:argos
```

## Bloqueo

Reinyectar handoff con `process: feature`, `phase: Verificación`, `persist_ref: docs/features/tracker-lifecycle-stamp`, `branch_name: feat/tracker-lifecycle-stamp` y Shell/CLI operativo. Prohibido declarar APTO sin stdout de cápsulas.
