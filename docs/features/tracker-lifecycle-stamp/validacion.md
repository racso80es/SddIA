---
feature_name: tracker-lifecycle-stamp
created: "2026-10-22"
process: feature
branch: feat/tracker-lifecycle-stamp
global: NO_APTO
pbi_archived: false
execution_id: "1d8790f8-abfe-4880-a3a5-ec0c9093812b"
pbi_ref: docs/todos/pending/[ARQUITECTURA] HU-003 PBI-03 — tracker lifecycle.md
checks:
  AC-1:
    status: PENDIENTE-CAPSULA
    note: "Inspección estática Grep IDE: 0 hits de linear-tracker-adapter en tracker-stamp.md y tracker_stamp.rs. Sin rg vía cápsula/Shell (runtime bloqueado)."
  AC-2:
    status: BLOQUEADO
    note: "Sin Shell: no se pudo ejecutar cargo test ac2_state_stamp_uses_live_op_and_schema ni sello live con provider linear + validate_output_payload."
  AC-3:
    status: PENDIENTE-CAPSULA
    note: "Lectura estática: tracker-stamp.md declara context: [tracker-operations]. Sin evidencia de cápsula/git-manager."
  runtime_integrity:
    status: FAIL
    note: "Kalma2 inyectó process/phase/persist_ref/branch vacíos (?). Shell IDE rechazado; git-manager y execute-process no invocables."
git_changes:
  - SddIA/process/tracker-stamp.md
  - SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs
  - docs/features/tracker-lifecycle-stamp/objectives.md
  - docs/features/tracker-lifecycle-stamp/validacion.md
  - docs/todos/pending/[ARQUITECTURA] HU-003 PBI-03 — tracker lifecycle.md
approval_status: rejected
integrity_failure: true
---

# validacion.md — tracker-lifecycle-stamp

## Veredicto

**NO_APTO** — fallo de integridad Argos (L-ARGOS-SYNTHESIS / falla controlada).

## Evidencia

| Fuente | Resultado |
|--------|-----------|
| Shell / `./sddia-run.sh` | Rechazado por runtime; sin stdout de cápsulas |
| `skill:git-manager` | No invocado (depende de Shell/CLI) |
| Grep IDE `linear-tracker-adapter` | 0 matches en los dos ficheros (no sustituye AC-1 capsule) |
| Lectura `tracker-stamp.md` | `requires_capability: tracker:lifecycle`; `context: tracker-operations` |
| Cascada documental | Solo `objectives.md` presente pre-auditoría; faltan clarify/spec/plan/implementation/execution |

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
    intent: AC-1 rg; AC-2 cargo test ac2_state_stamp_uses_live_op_and_schema; AC-3 context.
    delegates_to:
      - agent:argos
```

## Bloqueo

No se declara APTO sin evidencia determinista de cápsulas. Reinyectar handoff Kalma2 con `process: feature`, `phase: Verificación`, `persist_ref`, `branch_name` y Shell operativo.
