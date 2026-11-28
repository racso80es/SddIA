---
uuid: "7e4a9c2b-1d3f-4a8e-9b6c-0f1e2d3a4b5c"
name: "emit-domain-mutation"
version: "2.0.2"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "domain-mutation-emission"
  - "event-bus-pending-write"
  - "delegate-filesystem-manager"
  - "delegate-crypto-broker"
  - "domain-event-type-translation"
inputs: []
outputs: []
hash_signature: "sha256:e53a00b779b8a1dca58c8f7a263d021d6a09ba0273b349499353bb2875009337"
minteo_maximo: null
porcentaje_de_exito: null
---

## Propósito

Sello universal EDA tras forja gobernada: publica `Domain_Entity_Created|Updated|Deleted` en `pending`, valida ECST y sincroniza `SddIA/core/eda-coverage.json`.

## Orquestación

Handler `engine/domain_mutation.rs`: exige `entity_class` (piloto), `entity_name`, `lifecycle_operation`, `hash_signature_new` (y `_old` en update); idempotencia por par `(lifecycle, hash)`; `validate_domain_mutation_event` antes de `publish_ecst`. Delegaciones declaradas: `delegate-crypto-broker`, `delegate-filesystem-manager`.

## Límites

No mueve ficheros del bus; no sustituye `route-domain-event`. Consumida por `entity-manager` fase sello y procesos creator.
