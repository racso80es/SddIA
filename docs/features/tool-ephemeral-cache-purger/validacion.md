---
feature_name: tool-ephemeral-cache-purger
created: "2026-09-09"
process: feature
branch: feat/tool-ephemeral-cache-purger
global: PENDIENTE-CI
pbi_archived: false
checks:
  PURGE-CA1: APTO
  PURGE-CA2: APTO
  PURGE-CA3: APTO
  PURGE-CA4: APTO
  PURGE-CA5: APTO
  PURGE-CA6: APTO
  PURGE-CA7: APTO
  PURGE-CA8: PENDIENTE-CI
git_changes:
  - SddIA/tools/ephemeral-cache-purger.md
  - SddIA/tools/ephemeral-cache-purger/
  - SddIA/tools/index.md
  - SddIA/actions/purge-sandbox-cache.md
  - SddIA/actions/index.md
  - SddIA/engine/execute-process/src/engine/purge_sandbox_cache.rs
  - SddIA/engine/execute-process/src/engine/actions.rs
  - SddIA/engine/execute-process/src/engine/mod.rs
  - SddIA/core/eda-coverage.json
  - docs/features/tool-ephemeral-cache-purger/
  - docs/todos/pending/[FEATURE] Tool: ephemeral-cache-purger (Saneamiento Termodinámico).md
---

# Validación — tool-ephemeral-cache-purger

CA1–CA7: evidencia tests crate (6) + handler (2). CA8: `PENDIENTE-CI` hasta `run_id` verde. `global` no APTO. PBI permanece en pending hasta CA8.
