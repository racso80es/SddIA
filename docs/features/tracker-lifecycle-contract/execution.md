---
feature_name: tracker-lifecycle-contract
created: "2026-10-22"
process: feature
items_applied:
  - taxonomy-1.0.9
  - bindings-1.7.0
  - schema-emit-v2
  - validator-test
branch_name: feat/tracker-lifecycle-contract
persist_ref: docs/features/tracker-lifecycle-contract
execution_id: "3e0a8c0f-1b97-4b9c-a467-b149494b3e28"
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-01
---

# Ejecución — tracker-lifecycle-contract

Init acusado. Rama `feat/tracker-lifecycle-contract`. `execution_id` `3e0a8c0f-1b97-4b9c-a467-b149494b3e28`.

## AC-2 y AC-1

`CARGO_TARGET_DIR=target cargo test -p execute-process ac_tracker_lifecycle_emit_v2_body -- --test-threads=1`

Resultado: `ok`. El cuerpo de éxito y el de fallo de `emit_v2` pasan el schema. Un objeto sin `success`/`exitCode` devuelve `SchemaMismatch`. El gate sobre `requires_capability: tracker:lifecycle` sin delegate responde `CapabilityProviderMismatch`, no `CapabilityNotIndexed`.

## AC-3

Catorce `capability_id` en bindings. Cero duplicados. `tracker:lifecycle` aparece una vez. El catálogo contiene un solo `id: "tracker:lifecycle"`. No hay `id: "tracker.lifecycle"`.

## Hash

Hash previo de la taxonomía, igual al frontmatter: `sha256:f03b9a2dbdb03585139ba0122eddff6769af26a060a8f9a20566d2b610cd5629`.

Hash nuevo: `sha256:4cf09806f229ac1d4ca0c3131bb054a2db06c715bce28d549310a1d111e1ceab`.

## Evolution y sello

`sddia-qa evolution-register` operación `alta`. `id_cambio` `2c882ecf-0c03-48a9-9259-8de9a15a2a74`. `hash_integrity` `sha256:e60645c8170c9b4c389195e2528685911ed3c57d0b853022efd6d4b94a48d6c2`.

`emit-domain-mutation` update de `capability-taxonomy` acusó `idempotent: true` sobre el evento ya procesado `5505e7ef-a933-4ad4-ade1-4e75607abf3e` (`Domain_Entity_Updated`). El motor no mintea un segundo update para el mismo `entity_uuid`. La cobertura sí avanzó: `eda-coverage.json` `last_hash` = hash nuevo, `correlation_timestamp` `2026-10-22T17:48:02.437Z`.
