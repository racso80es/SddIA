---
feature_name: bridge-presented-tracker-ref
created: "2026-10-30"
process: bug-fix
branch_name: fix/bridge-presented-tracker-ref
persist_ref: docs/fixes/bridge-presented-tracker-ref
pbi_ref: AUT-25
tracker_ref: AUT-25
execution_id: "ceb7ecb1-5e05-4aef-b6f9-c58ed2b6540e"
---

# Spec — bridge-presented-tracker-ref

## Comportamiento

1. Antes de `compose_pre_anchor_event`, el puente lee `objectives.md` del commit `head_sha` (`git ls-tree` + `git show`). Si `branch_name` del YAML coincide con la rama del PR, copia `tracker_ref` y `pbi_ref` al payload. Commit ausente, SHA inválido o rama sin esas claves: el evento se publica igual, sin esos campos.
2. `build_bus_event` deja `delivery_state` vacío. `materialize_to_bus` deja que `stamp_published_event` rellene el plan del catálogo. `dlt_anchor_address` sigue en el payload.
3. El runtime de `emit-pr-presented-event` y la fase DCC ya copian `tracker_ref`. Las tablas de `emit-pr-presented-event.md` y de `delivery-close-cycle.md` no se reescribieron en este paso: son genoma con `hash_signature`.

## Fuera de alcance

Fallback dentro de `tracker-stamp`. Lock de `events.redb`. Transición manual de AUT-8.
