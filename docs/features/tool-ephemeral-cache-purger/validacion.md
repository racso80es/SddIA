---
feature_name: tool-ephemeral-cache-purger
created: "2026-09-09"
process: feature
branch: feat/tool-ephemeral-cache-purger
global: APTO
pbi_archived: true
pr_url: "https://github.com/racso80es/SddIA/pull/279"
checks:
  PURGE-CA1: APTO
  PURGE-CA2: APTO
  PURGE-CA3: APTO
  PURGE-CA4: APTO
  PURGE-CA5: APTO
  PURGE-CA6: APTO
  PURGE-CA7: APTO
  PURGE-CA8: APTO
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
  - docs/todos/done/[FEATURE] Tool: ephemeral-cache-purger (Saneamiento Termodinámico).md
---

# Validación — tool-ephemeral-cache-purger

CA1–CA7: tests crate (6) + handler (2).

CA8: workflow `sddia-index-qa` `conclusion: success`. `run_id` `34344716732` (`headSha` `a853c494627faa27cc39876f6f932101785513ab`). Jobs: `sddia-index-integrity`, `wasi-runtime-smoke`, `eda-bus-e2e-smoke`, `eda-iota-smoke-simulate`, `eda-iota-physical`. Run push `34344712457` también verde (e2e/physical skip por evento push).

PBI archivado en `docs/todos/done/` en esta rama. `global: APTO`.
