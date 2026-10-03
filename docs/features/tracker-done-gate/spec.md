---
feature_name: tracker-done-gate
created: "2026-10-02"
process: feature
base: docs/features/spike-linear-done-gate/informe.md
---

# Especificación — laudo «no migrar» gate Done

## Decisión (laudo)

**No se migra** el criterio de Done del motor SddIA a Linear en esta HU.

Se adopta la recomendación del informe de spike (`docs/features/spike-linear-done-gate/informe.md`):

- **SSOT documental Done** permanece: `validacion.md` con `global: APTO` y `pbi_archived: true`, más PBI en `docs/todos/done/` en el mismo PR que el código (D1, `features-documentation-pattern` v1.2.x, `task-closure-documental`).
- **Linear** refleja estado operativo (transiciones/comentarios) vía `tracker-stamp`; no sustituye el gate local hasta un laudo futuro y trabajo de producto explícito.
- **Riesgo evitado:** desalinear pre-push / `feature-pbi-archive` con el estado en Linear si Done viviera solo en el tracker externo.

Este PBI **no implementa** migración de gate ni cambios en hooks.

## Puntero Q-5 (`docs_layout` vs handler)

La deuda de layout no está en `fracture_pbi::resolve_todos_done_rel` (sí resuelve rutas de todos). Está en el handler **`feature-pbi-archive`** (`SddIA/engine/execute-process/src/engine/phase_capsules.rs`), que cablea rutas fijas:

1. **Ya en `done/`** — si `rel.starts_with("docs/todos/done/")` → `already_archived: true` (sin mover fichero).
2. **Fuera de `pending/`** — si la ruta no contiene `docs/todos/pending/` → `skipped: true` (`PBI fuera de pending/`).
3. **Archivo en `pending/`** — `done_dir = repo.join("docs/todos/done")`; si el destino ya existe → `already_archived: true`; si no → `rename` a `done/` y `archived: true`.

Cualquier futura migración del gate o lectura de `docs_layout.todos_done` debe tratar estas tres ramas; **este laudo no modifica** `phase_capsules.rs` ni hooks de pre-push.

## HU futura de migración

Queda **bloqueada** hasta el E2E mock del ciclo completo (**AC-8**), entregable de `PBI-DEUDA-TRACKER-STAMP-PARIDAD` (OSC-8). No se forja HU de migración en OSC-11.
