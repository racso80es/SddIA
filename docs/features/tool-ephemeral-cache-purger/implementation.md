---
feature_name: tool-ephemeral-cache-purger
created: "2026-09-09"
process: feature
items:
  - entity-manager-tool
  - entity-manager-action
  - crate-jail
  - action-handler
  - tests
branch_name: feat/tool-ephemeral-cache-purger
persist_ref: docs/features/tool-ephemeral-cache-purger
execution_id: "7f37a724-5d10-41df-9cc9-cf22dc275951"
---

# Implementation — tool-ephemeral-cache-purger

## Touchpoints

| Path | Rol |
|------|-----|
| `SddIA/tools/ephemeral-cache-purger.md` | Spec forjada. UUID `8929eeea-5e5b-402a-aafa-4a9b429acaec` |
| `SddIA/tools/ephemeral-cache-purger/` | Crate nativo jail + I/O `sddia-io` |
| `SddIA/tools/index.md` | Fila catálogo |
| `SddIA/actions/purge-sandbox-cache.md` | Acción v1.0.1. UUID `37454c3a-4590-4dfb-a439-2014876a0138` |
| `SddIA/actions/index.md` | Fila catálogo |
| `SddIA/engine/execute-process/src/engine/purge_sandbox_cache.rs` | Handler dos tiempos |
| `SddIA/engine/execute-process/src/engine/actions.rs` | Brazo `purge-sandbox-cache` |
| `SddIA/core/eda-coverage.json` | Upsert EM |

## Propuestas aplicadas

Jail regex único. Default `cargo-target`. Envelope `sddia-io`. Acción sin Argos.
