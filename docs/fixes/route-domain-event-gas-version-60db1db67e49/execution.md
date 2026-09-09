---
feature_name: route-domain-event-gas-version-60db1db67e49
created: "2026-09-09"
process: bug-fix
branch_name: fix/route-domain-event-gas-version-60db1db67e49
persist_ref: docs/fixes/route-domain-event-gas-version-60db1db67e49
pbi_ref: docs/todos/done/[FIX] route-domain-event — fractura sistémica (60db1db67e49).md
document_id: PBI-FIX-FRACTURE-60db1db67e49
execution_id: "468e7f10-99aa-4c91-8803-e06e229742a3"
items_applied:
  - relay-serial-queue
  - dlt-gas-predicate
  - mayeuta-catchall-dlt-subtype
  - genome-enrich-1.4.0
  - e2e-capsule-simulate0
  - evolution-alta
---

# Ejecución — fractura `60db1db67e49`

## Init

```bash
SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_ALLOW_DIRTY=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 \
  ./sddia-run.sh --process bug-fix --inputs-file .tmp/bug-fix-60db1db67e49-init.json
```

`execution_id`: `468e7f10-99aa-4c91-8803-e06e229742a3`. Diseño `simulated`. Commit planificación `84c412b`.

## Tests (CA1–CA3, CA5–CA7)

```text
cd .SddIA/services/iota-publish-relay && node --test publish-queue.test.mjs relay-error.test.mjs
# 7 passed

cd SddIA && cargo test -p execute-process --lib -- dlt_transient emit_dlt_batch_fracture analyze_fracture_kaizen enqueue_dlt_reanchor
# 23 passed; 0 failed
```

## E2E CA4 (`SIMULATE=0`)

```bash
./sddia-run.sh --tool iota-immutable-publisher --prefer-native --inputs-file .tmp/iota-publish-ca4-60db1db.json
```

Cápsula (no curl POST). Relay systemd `sddia-iota-publish-relay@home-racso-Proyectos-SddIA` **active**.

```text
success: true
exitCode: 0
mode: relay
transaction_digest: 5rkFWghseVYgDh5DTQsECyeRS9T1d99hkkBXa7ELoEja
object_id: 0x26ca0b637495bd888c0871de1e4f3e4093c1b03fb6a5a2f7e636d436cc233178
```

## Genoma

`entity-manager` update `enrich-fracture-pbi-kaizen` v1.4.0. `hash_signature: sha256:ef22da114e0d5d1ef82d10eeb8950bd6014dc3e67fc3ce45f61bccdc7d176ec6`. Evento `ab2f5558-16ae-462a-8a1f-ac49811893d2`.

## Evolution

`sddia-qa evolution-register` → `5f09da5c-a96c-4475-9ce6-15d55cc5840c` (`EVOL_OK`, `alta`).

## Diferido

`PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION` — `docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md`. No despachado.
