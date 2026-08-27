---
feature_name: kaizen-aduana-dlt-relay-supervisado
created: "2026-08-27"
process: feature
branch_name: feat/kaizen-aduana-dlt-relay-supervisado
persist_ref: docs/features/kaizen-aduana-dlt-relay-supervisado
document_id: PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO
uuid: "1243c58b-8e93-4897-ba3e-3efc26564673"
execution_id: "cdd000a0-75d3-4bf9-9a4b-c1d889860ed2"
items_applied:
  - daemon-creator-iota-publish-relay
  - cargo-build-execute-process-iota-publish-relay
  - route-domain-batch-cause-fracture-queue
  - dlt-backfill-rescue-28
agents: tekton
---

# Execution — kaizen-aduana-dlt-relay-supervisado

## Build

```bash
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo build -p execute-process -p iota-publish-relay -p iota-immutable-publisher
```

## Forja daemon (T1)

```bash
SDDIA_AGENT_RUNTIME_COMMAND="" SDDIA_LAB_SKIP_CAPABILITY_DI=1 \
  ./sddia-run.sh --process daemon-creator --inputs '{...}'
```

→ `handoff_entity_uuid` `78e94d53-0445-4394-b399-3e594cabc511`, `native-forge` executed.

## Rescate (T4)

```bash
python3 docs/features/kaizen-aduana-dlt-relay-supervisado/dlt-backfill-rescue.py
```

→ 28 payloads, acta + proofs, DL subscriber → `processed/subscribers/`.

## Smokes (T8)

| CA | Estado |
|----|--------|
| DLT-CA1 | ✅ lock + launcher + índice + bin |
| DLT-CA2 | ✅ `_wait_http` en `start-sddia.sh` |
| DLT-CA3/4 | ✅ tests `batch_anchor_*` + `emit_dlt_batch_fracture` |
| DLT-CA5 | ✅ supervisor restart loop |
| DLT-CA6..9 | ✅ acta 28 + digest on-chain |
| DLT-CA10 | ✅ cola `dlt_reanchor` + drain |

## Cierre (T9)

`validacion.md` APTO · PBI en `docs/todos/done/` · evolution `1243c58b-…` · `delivery-close-cycle` + PR.
