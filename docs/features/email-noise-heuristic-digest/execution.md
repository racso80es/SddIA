---
feature_name: email-noise-heuristic-digest
created: "2026-09-11"
process: feature
branch_name: feat/email-noise-heuristic-digest
persist_ref: docs/features/email-noise-heuristic-digest
execution_id: "8fa709ed-6c4a-4b71-bac1-a2b69ecb0bf3"
items_applied:
  - handler-aggregation
  - tests-cursor-telegram
  - entity-manager-process
  - entity-manager-codex
  - evolution-register
---

# Ejecución — email-noise-heuristic-digest

## Init

`SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 ./sddia-run.sh --process feature`

`execution_id` `8fa709ed-6c4a-4b71-bac1-a2b69ecb0bf3`. Commit planificación `c7d541e`.

## Código (L1–L2)

Handler `email_noise_digest.rs`. Dispatch en `engine/mod.rs`.

```text
cd SddIA && cargo test -p execute-process --lib -- email_noise_digest
# 7 passed
```

## Genoma (L3)

| Entidad | EM execution_id | Sello | hash_new |
|---------|-----------------|-------|----------|
| process `email-noise-digest` 1.0.0 | `6f6dc071-0e93-4fcd-9ce5-04994c7e6e76` | Domain_Entity_Created `1aca4f9b-434b-4f16-a2ec-95df1505144a` | `sha256:24515d9ea999fbddb282705e42588bee8fe5211847c4c81a4a113023b4961482` |
| códice `codex-kalma2-assistant` 1.0.1 | `63608967-9ce3-4aac-8174-e39d09c6647f` | Domain_Entity_Updated `37efecb7-d526-4e9c-97b3-193467ac01d6` | `sha256:5e7ab8d9ab578785ac1101ac89e34b86fdbccfcb4a81760d96dcea06f47328a7` |

Root: `SddIA/library/codexes/codex-kalma2-assistant/process`. UUID proceso `fc11c0d6-09ba-48e6-972c-561847f8c8ef`.

## Evolution

`sddia-qa evolution-register` → `cf1ddf69-3dc6-4576-8245-e47c9536b000` (`exitCode` 0, `alta`).
