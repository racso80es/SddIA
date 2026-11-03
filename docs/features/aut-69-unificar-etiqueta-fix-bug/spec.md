---
feature_name: aut-69-unificar-etiqueta-fix-bug
created: "2026-11-03"
base: refactor/man-26-ciclo-vida-software
scope: tracker.labels.fix
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-69-unificar-etiqueta-fix-bug
pbi_ref: AUT-69
execution_id: "plan-aut-69-20261103"
---

# Spec — AUT-69

## Mapa

| Clave | Valor actual | Valor destino |
|---|---|---|
| `tracker.labels.fix` | `Fix` | `Bug` |

Las demás claves del bloque `labels` no se modifican.

## Consumidores

Leen `tracker.labels.fix` del manifiesto. Tras el cambio envían `Bug`:

- `forge-pbi` cuando `process` es `bug-fix` (`tipo_label_key` → clave `fix`).
- `materialize_fracture_pbi::run_remote` (`tracker_label_for_inputs(..., "fix")`).

No hay fallback que deba reescribirse: con la clave presente, `unwrap_or("fix")` no entra.

## Linear

| Label | Ámbito | Acción |
|---|---|---|
| `Fix` | equipo AUT | quitar de los 25 issues y borrar la label |
| `Bug` | workspace | añadir a esos 25 issues |

La cápsula no tiene operación de borrado de label ni de actualización masiva de `labelIds`. La migración es GraphQL `issueUpdate` con la lista de ids resultante (ids actuales menos `Fix` más `Bug`), fuera del contrato de `linear-tracker-adapter`. El borrado de la label de equipo es el paso final, cuando `issues(filter: labels name eq Fix)` del equipo AUT devuelve cero.

## Invariantes

- Identificador de issue lo asigna Linear. No se renombran AUT-n.
- `Bug` no sustituye a `Pbi`. Un defecto sigue llevando `Pbi` y la label de tipo.
- Tests de laboratorio que declaran `fix: fix` contra equipos `LAB` u `OSC` no se reescriben.
