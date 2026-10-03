---
feature_name: eda-scan-process-index
process: bug-fix
branch: fix/linear-tracker-adapter-hash
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[DEUDA] Tracker — escáner EDA de procesos.md
checks:
  AC-1: APTO
  AC-2: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/eda_coverage.rs
  - SddIA/evolution/4393da04-37b3-44fe-9f95-688774a1b643.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — eda-scan-process-index

`cargo test -p execute-process parse_index_row_without_backticks`: OK.  
`execute-process --audit-eda-coverage --scan --json`: `orphan_count: 0`.
