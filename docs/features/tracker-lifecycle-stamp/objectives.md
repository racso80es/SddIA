---
feature_name: tracker-lifecycle-stamp
created: "2026-10-22"
process: feature
branch_name: feat/tracker-lifecycle-stamp
persist_ref: docs/features/tracker-lifecycle-stamp
pbi_ref: docs/todos/done/[ARQUITECTURA] HU-003 PBI-03 — tracker lifecycle.md
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-03
pbi_uuid: "85caf44d-e5d0-4c30-8303-bcb825610f92"
execution_id: "1d8790f8-abfe-4880-a3a5-ec0c9093812b"
---

# Objetivos — tracker-lifecycle-stamp

## Misión

`tracker-stamp` pide `tracker:lifecycle`. El handler invoca el artefacto de la fila de binding. Con provider `file` no hay cápsula.

## Alcance

| Dentro | Fuera |
|--------|-------|
| `requires_capability` en la fase | Abrir `tracker-operations` en `feature`, `bug-fix` o `refactorization` |
| Nombre de tool leído de la fila | Cambiar el enum de `tracker.provider` en el contrato de proyecto |
| No-op si el manifiesto dice `provider: file` | Operaciones nuevas |

## Criterios

| ID | Criterio |
|----|----------|
| AC-1 | `linear-tracker-adapter` no aparece en el proceso ni en el handler. |
| AC-2 | Con provider `linear`, el sello de estado usa `update_issue_state` y el stdout cumple el schema `tracker.lifecycle`. |
| AC-3 | El contexto del proceso sigue siendo `tracker-operations`. |

## Ley aplicada

- La fila y el `provides` ya están en `main`.
- `process-creator` update sin fases refresca el hash y conserva el cuerpo escrito.
