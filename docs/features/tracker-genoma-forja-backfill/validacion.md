---
feature_name: tracker-genoma-forja-backfill
process: refactorization
branch: fix/linear-tracker-adapter-hash
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[DEUDA] Tracker — backfill forja entity-manager.md
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
git_changes:
  - SddIA/core/eda-coverage.json
  - SddIA/engine/execute-process/src/forges/factory.rs
  - SddIA/engine/execute-process/src/forges/common.rs
  - SddIA/engine/execute-process/src/engine/entity_manager.rs
  - SddIA/evolution/1f389045-ef96-469c-858c-cca1f9d1b259.md
  - SddIA/evolution/4393da04-37b3-44fe-9f95-688774a1b643.md
---

# Validación — tracker-genoma-forja-backfill

Inventario «sellar» con hash canónico en `coverage_matrix`. `--audit-eda-coverage --scan`: `orphan_count: 0`. Sin secretos en diff.
