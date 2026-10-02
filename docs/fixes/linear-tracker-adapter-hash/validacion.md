---
feature_name: linear-tracker-adapter-hash
process: bug-fix
branch: fix/linear-tracker-adapter-hash
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[DEUDA] Tracker — hash de linear-tracker-adapter.md
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
git_changes:
  - SddIA/tools/linear-tracker-adapter.md
  - SddIA/core/eda-coverage.json
---

# Validación — linear-tracker-adapter-hash

`entity-manager` → `tool-creator` con `hash_refresh_only: true`. `handoff_hash_signature_new` = `sha256:8722fcf349c933f3a7af8eb00f4fb1912591f48cef8c1944a787c3ccac3c46d7`. `cargo test -p linear-tracker-adapter`: 6 tests verdes.
