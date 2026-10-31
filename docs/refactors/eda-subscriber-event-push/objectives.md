---
feature_name: eda-subscriber-event-push
created: "2026-10-31"
process: refactorization
branch_name: Fix_Events_Multiple
persist_ref: docs/refactors/eda-subscriber-event-push
tracker_ref: AUT-21
historia_ref: AUT-20
---

# Objetivos — eda-subscriber-event-push

## Misión

Cerrar la deuda AUT-21: el router entrega el ECST por parámetros, el suscriptor no depende del fichero del bus, y un no-op con tracker remoto no queda como `success`.

## Alcance

Puerto único de carga, despacho de los siete suscriptores con `event` y `event_id`, autenticidad antes de escribir en el tracker remoto, aduana contra la lectura ciega del path, y verificación de la semántica de entrega ya clasificada en AUT-46.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager` cuando haya commit.
- Genoma indexado (`events`, `process`, `norms`, códices) solo vía `entity-manager`.
- El motor en `SddIA/engine/` no es genoma de esas claves.
- `subscriber_delivery` es la fuente de verdad del estado de entrega.
- Rama de trabajo: `Fix_Events_Multiple`.
