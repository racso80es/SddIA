---
feature_name: tracker-lifecycle-stamp
created: "2026-10-22"
updated: "2026-10-22T20:22:00+02:00"
process: feature
branch: feat/tracker-lifecycle-stamp
global: NO_APTO
pbi_archived: false
execution_id: "1d8790f8-abfe-4880-a3a5-ec0c9093812b"
pbi_ref: docs/todos/done/[ARQUITECTURA] HU-003 PBI-03 — tracker lifecycle.md
checks:
  AC-1:
    status: PENDIENTE-CAPSULA
    note: "Grep IDE: 0 hits linear-tracker-adapter en tracker-stamp.md y tracker_stamp.rs. No sustituye cápsula (L-ARGOS-SYNTHESIS)."
  AC-2:
    status: BLOQUEADO
    note: "Existe stdout ajeno (terminal Tekton 509667: cargo test handlers::tracker_stamp succeeded). Argos no re-ejecutó cápsula → no acredita."
  AC-3:
    status: PENDIENTE-CAPSULA
    note: "Lectura estática: context [tracker-operations]; requires_capability tracker:lifecycle. Sin evidencia de cápsula."
  runtime_integrity:
    status: FAIL
    note: "Handoff Kalma2: process/phase/persist_ref/branch_name/execution_id/pbi_ref/correlation_id = ?. Shell IDE Rejected (probe #4: echo SHELL_PROBE_ARGOS4). ./sddia-run.sh, skill:git-manager y action:execute-process no invocables. Sin tools dinámicos sddia/git-manager. Reintento Argos #4."
git_changes: []
approval_status: rejected
integrity_failure: true
argos_attempt: 4
---

# validacion.md — tracker-lifecycle-stamp

## Veredicto

**NO_APTO** — fallo de integridad Argos (sección 3 / L-ARGOS-SYNTHESIS). Sin `active_norm_pack` inyectado ni runtime de cápsulas. Intento #4: mismo bloqueo.

## Evidencia

| Fuente | Resultado |
|--------|-----------|
| Shell / `./sddia-run.sh` | Rejected (probes #4) |
| `skill:git-manager` | No invocado (sin canal) |
| GetDynamicTools `sddia\|git-manager\|execute-process` | 0 matches |
| Grep IDE `linear-tracker-adapter` | 0 matches (proceso + handler) |
| Lectura `tracker-stamp.md` | `context: tracker-operations`; `requires_capability: tracker:lifecycle`; hash `sha256:2cf44164…` |
| Terminal ajeno `509667.txt` | Tekton: cargo test `handlers::tracker_stamp` succeeded; **no** atribuible a cápsula Argos |
| PBI | `docs/todos/done/...PBI-03...`; cierre documental APTO pendiente |

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

Reinyectar handoff con campos rellenados:

- `process: feature`
- `phase: Verificación`
- `persist_ref: docs/features/tracker-lifecycle-stamp`
- `branch_name: feat/tracker-lifecycle-stamp`
- `execution_id: 1d8790f8-abfe-4880-a3a5-ec0c9093812b`
- `pbi_ref: docs/todos/done/[ARQUITECTURA] HU-003 PBI-03 — tracker lifecycle.md`

Y Shell/CLI operativo. Prohibido declarar APTO sin stdout de cápsulas invocadas por Argos.
