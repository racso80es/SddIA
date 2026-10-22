---
feature_name: tracker-lifecycle-contract
created: "2026-10-22"
process: feature
base: main
scope: contrato tracker:lifecycle
execution_id: "3e0a8c0f-1b97-4b9c-a467-b149494b3e28"
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-01
---

# Spec — tracker-lifecycle-contract

## Taxonomía

`SddIA/library/norms/capability-taxonomy.md` pasa a `1.0.9`. El `catalog` gana:

| Campo | Valor |
|-------|--------|
| id | `tracker:lifecycle` |
| contract | `tracker.lifecycle` |
| version | `1.0.0` |

`hash_signature` se recalcula con el algoritmo del forje: SHA-256 del fichero sin la línea `hash_signature:`. La fila del índice `library/norms/index.md` copia la versión `1.0.9`.

## Binding

`SddIA/core/capability-bindings.md` pasa a `1.7.0`. Una fila:

| capability_id | contract | provider | provider_version |
|---------------|----------|----------|------------------|
| `tracker:lifecycle` | `tracker.lifecycle` | `tool:linear-tracker-adapter` | `>=1.0.0` |

`load_binding_rows` aborta si el `capability_id` se repite.

## Schema

`SddIA/library/norms/capability-contracts/tracker.lifecycle.schema.json`. Draft 2020-12. Required: `success` (boolean), `exitCode` (integer). Propiedades conocidas: `message`, `result`, `meta`, `error`. `additionalProperties: true`.

Cuerpo de éxito que debe pasar:

```json
{
  "meta": {"schemaVersion": "2.0", "entityKind": "tool", "entityId": "linear-tracker-adapter"},
  "success": true,
  "exitCode": 0,
  "message": "linear-tracker-adapter ok",
  "result": {"id": "OSC-1"}
}
```

Un objeto sin `success` o sin `exitCode` es `CONTRACT_OUTPUT_SCHEMA_MISMATCH`.

## Prueba

`ac_tracker_lifecycle_emit_v2_body` en `capability_di_output_validator` usa el schema del repo y el gate real. El id indexado no produce `CAPABILITY_NOT_INDEXED`. Sin `provides` en la tool, el gate responde `CAPABILITY_PROVIDER_MISMATCH`.
