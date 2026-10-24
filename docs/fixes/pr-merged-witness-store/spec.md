---
feature_name: pr-merged-witness-store
created: "2026-10-24"
process: bug-fix
base: main
scope: alta de PullRequest_Merged, promoción de testigo y tracker_ref de ciclo
tracker_ref: MAN-23
---

# Spec — testigo de merge en el store

## Hecho cerrado

`route_domain_event` ya inserta la fila con `record_ecst` antes del fan-out. Este fix no lo reabre.

## Corte

1. `actions.rs` `write_pending_event` inserta la fila cuando `writes_store`, en el mismo acto que el JSON. Con `SDDIA_EVENT_STORE=filesystem` deja el JSON y no abre el store. Con `redb` no crea el JSON.
2. Si `promote_witness` falla, `delivery_status` conserva el estado de la delegación. El dead-letter de fallback guarda `result_status` y `error_trace`.
3. Un `PullRequest_Merged` de ciclo lleva `tracker_ref`. El sello lo toma del input, del `PullRequest_Presented` (pending, processed o store) o de `tracker_ref` en `objectives.md`. Un valor ya presente no se pisa.
