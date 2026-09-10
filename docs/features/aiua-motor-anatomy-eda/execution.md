---
feature_name: aiua-motor-anatomy-eda
created: "2026-09-10"
process: feature
branch_name: feat/aiua-motor-anatomy-eda
persist_ref: docs/features/aiua-motor-anatomy-eda
execution_id: "8658220b-47ff-4513-a02e-2a3ed3e1adbe"
document_id: PBI-NUCLEO-AIUA-ANATOMIA-MOTORA-EDA
items_applied:
  - forge-event-em
  - forge-action-em
  - forge-process-em
  - genome-conscience-subs
  - handler-parser-dispatch
  - route-subscriber
  - tests-lab
---

# Ejecución — aiua-motor-anatomy-eda

## Init

`execution_id` `8658220b-47ff-4513-a02e-2a3ed3e1adbe`. Relé IDE. Commit planificación `0f0bf71`.

## L1 genoma (entity-manager)

| Entidad | EM execution_id | Sello | UUID |
|---------|-----------------|-------|------|
| event `aiua-process-requested` | `3f553142-396d-40b9-9bcc-974b8a247b1d` | Domain_Entity_Created `36570a8c-…` | `c9a6db76-…` |
| action `dispatch-aiua-intent` | `b46f7703-3702-450c-b796-5f910770aad2` | Domain_Entity_Created `032671aa-…` | `a1086194-…` |
| process 1.2.0 phases | `bd3d0854-f24c-4078-941f-0d034ff87fda` | Domain_Entity_Updated `caeadda9-…` | `6c595785-…` inmutable |
| process body | `4b6363c5-ed0a-4d5f-be66-df91f010e53c` | idempotente hash fases | inmutable |

## L2 handler

`aiua_intent.rs` + `infer` post-parse. Overlay lab. `dispatch_subscriber` conjunto `{Kalma2, Aiua}_Process_Requested`.

## Tests

```text
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test --offline -p execute-process --lib -- aiua_intent aiua_stimulus aiua_process_requested_maps
# 16 passed
```

`sddia-qa evolution-register` → `60b81a68-39ac-4ccd-b920-6ab759b474d8` (`EVOL_OK`).

Crates HTTP/CLI. IOTA. `pending/`. PEC. Fractura `route-domain-event`.
