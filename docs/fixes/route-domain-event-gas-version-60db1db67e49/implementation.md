---
feature_name: route-domain-event-gas-version-60db1db67e49
created: "2026-09-09"
process: bug-fix
branch_name: fix/route-domain-event-gas-version-60db1db67e49
persist_ref: docs/fixes/route-domain-event-gas-version-60db1db67e49
pbi_ref: docs/todos/done/[FIX] route-domain-event — fractura sistémica (60db1db67e49).md
document_id: PBI-FIX-FRACTURE-60db1db67e49
execution_id: "468e7f10-99aa-4c91-8803-e06e229742a3"
items:
  - relay-serial-queue
  - dlt-gas-predicate
  - mayeuta-catchall-dlt-subtype
  - genome-enrich-1.4.0
---

# Implementación — fractura `60db1db67e49`

## Touchpoints

| Ítem | Path | Cambio |
|------|------|--------|
| CA1 cola | `.SddIA/services/iota-publish-relay/publish-queue.mjs` | `createSerialQueue()`: un `publishImmutableData` a la vez; el siguiente no arranca hasta resolver el precedente. |
| CA1 cable | `.SddIA/services/iota-publish-relay/server.mjs` | `enqueuePublish(() => publishImmutableData(...))`. |
| CA1 test | `.SddIA/services/iota-publish-relay/publish-queue.test.mjs` | Dos trabajos solapados no se construyen en paralelo. |
| CA2 | `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | `dlt_transient_gas_version_trace` (consumo ∧ `current version:`). `dlt_transient_error_trace` = red **o** gas. Guardia en `emit_dlt_batch_fracture`. Wrapper `issues with transaction inputs` sin firma **sí** emite. |
| CA3/CA5/CA6/CA7 | `SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs` | Subtipos DLT (gas / transporte / opaco). Catch-all sin `failed` (`timeout\|block\|abort\|colaps`). Cero `llm:interact`. |
| Genoma | `SddIA/actions/enrich-fracture-pbi-kaizen.md` v1.4.0 | Solo vía `entity-manager` (DA-2). `hash_signature: sha256:ef22da114e0d5d1ef82d10eeb8950bd6014dc3e67fc3ce45f61bccdc7d176ec6`. |

## Fuera

- `PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION` (`docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md`).
- Taxonomía b3a715, cause-propagation a90fad, predicado red 41717.
- Retry/sleep (DA-5). Bypass raw. `SIMULATE=1` como Done.
