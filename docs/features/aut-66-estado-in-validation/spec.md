---
feature_name: aut-66-estado-in-validation
created: "2026-11-03"
scope: in-validation-post-tekton
process: refactorization
persist_ref: docs/features/aut-66-estado-in-validation
pbi_ref: AUT-66
---

# Especificación — AUT-66

## Rango

| Canónico | Ordinal | Linear (propuesto) |
|---|---|---|
| backlog | 0 | Backlog |
| todo | 1 | Todo |
| in_progress | 2 | In Progress |
| in_validation | 3 | In Validation |
| in_review | 4 | In Review |
| done | 5 | Done |
| cancelled | 100 | Cancelled |

`should_discard_transition` descarta un destino con ordinal menor o igual al actual. De `in_validation` no se vuelve a `in_progress`.

## Evento

`Validation_Started`, emitido una vez al cerrar con éxito la fase cuyo `delegates_to` es `agent:tekton`.

Payload mínimo: `source_process`, `persist_ref`, `branch` si existe, `tracker_ref` o `pbi_ref`.

Suscriptor: `tekton.tracker-stamp` → estado `in_validation` + comentario de inicio de validación Argos. En file, parche de `tracker_column` sin mover el path.

## Continuidad

`PullRequest_Presented` mantiene destino `in_review` (ordinal 4 > 3).
