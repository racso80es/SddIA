---
feature_name: tool-ephemeral-cache-purger
created: "2026-09-09"
process: feature
branch_name: feat/tool-ephemeral-cache-purger
persist_ref: docs/features/tool-ephemeral-cache-purger
execution_id: "7f37a724-5d10-41df-9cc9-cf22dc275951"
items_applied:
  - PBI v1.2.0
  - entity-manager tool+action
  - crate jail tests
  - handler nativo
---

# Ejecución — tool-ephemeral-cache-purger

Init `7f37a724-5d10-41df-9cc9-cf22dc275951`. Relé IDE. Commit planificación `ee76dab`.

## Forja

1. EM create tool → UUID `8929eeea-5e5b-402a-aafa-4a9b429acaec`.
2. EM create action → UUID `37454c3a-4590-4dfb-a439-2014876a0138`.
3. EM update action v1.0.1 (cuerpo dos tiempos).

## Código

Crate `ephemeral-cache-purger` (`jail.rs` + `main.rs`). Handler `purge_sandbox_cache`.

## Tests

```
cd SddIA && cargo test -p ephemeral-cache-purger
# 6 passed

cd SddIA && cargo test -p execute-process --lib -- purge_sandbox
# 2 passed
```
