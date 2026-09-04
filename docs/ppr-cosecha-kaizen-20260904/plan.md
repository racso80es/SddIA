---
feature_name: ppr-cosecha-kaizen-20260904
created: "2026-09-04"
process: bug-fix
uuid: "04af1cd2-c9ee-4cc0-8b6b-af8d6b533ae0"
phases:
  - sync-main
  - persist-ref-sink
  - delivery-close-cycle
---

# Plan

1. `git-manager fetch` + `merge origin/main` (`no_ff: true`).
2. Materializar sink `docs/ppr-cosecha-kaizen-20260904/` (F2 PPR).
3. `delivery-close-cycle` (`source_process: bug-fix`, `branch_name: docs/ppr-cosecha-kaizen-20260904`). Snapshot no incluye `docs/todos/pending/` ajenos.
