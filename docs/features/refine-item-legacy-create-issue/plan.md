---
feature_name: refine-item-legacy-create-issue
created: "2026-11-01"
process: refactorization
branch_name: aut-35
persist_ref: docs/features/refine-item-legacy-create-issue
tracker_ref: AUT-35
execution_id: "27ab6def-6796-4baa-a3e6-c569f96efd72"
phases:
  - id: T1
    title: Retirar try_create_hu_issue de refine.rs
  - id: T2
    title: Predicado de tracker_ref en refine_item y tests
---

# Plan — AUT-35

Rama real `aut-35` @ `c008493b`. Un commit por tarea. El motor, con git activo, habría creado `refactor/refine-item-legacy-create-issue` desde `main` y habría dejado fuera el `run.rs` de AUT-34. No se hace ese checkout.

La cascada de PR 409 (`docs/features/refine-item-tracker-ref-remote/`) queda como registro APTO. Este plan no la reescribe.

## T1 — Alta legacy

En `SddIA/engine/execute-process/src/engine/handlers/refine.rs`:

- Borrar `try_create_hu_issue`.
- En `run_hu`, quitar el bloque `tracker_ref.is_none() && remote` que crea issue y asigna `tracker_ref`.
- `create_issue_invoked` no pasa a true en ese camino.

En el mismo fichero, `ac_0c_creates_tracker_ref_when_missing`: `create_issue_invoked` es `false`. No se afirma prefijo `OSC-`. El markdown local no gana un `tracker_ref` inventado.

`ac_0c_skips_create_when_tracker_ref_present` no cambia de expectativa.

## T2 — Predicado

En `handlers/refine_item/`:

- Función pura con el criterio ya escrito en `run.rs`: sin `tracker_ref` o vacío → sellar `identifier`; igual ignorando ASCII case → sellar; distinto → `conflict`.
- `run.rs` usa esa función en el `if remote` actual. Misma salida (`conflict` sin escritura, o `tracker_ref = identifier`).
- Tests de la función: tres casos de arriba. Sin Linear.

Comprobación: `cargo test -p execute-process --lib handlers::refine handlers::refine_item`.

Fuera de estas tareas: título Linear, estado Done, `forge-pbi`.
