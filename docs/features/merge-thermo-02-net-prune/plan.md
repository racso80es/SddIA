---

feature_name: merge-thermo-02-net-prune
created: "2026-10-03"
process: feature
branch_name: feat/merge-thermo-02-net-prune
persist_ref: docs/features/merge-thermo-02-net-prune
execution_id: "ddd321e1-0e81-4e0c-9a29-2651c869aa2a"
---
# Plan — merge-thermo-02-net-prune

1. `hook_common.sh`: orden bus→`gh`, `resolve_sddia_qa` F-DEP-07, `invoke_process` + timeout 180 s.
2. `gate_evolution.rs`: omitir `git fetch` si `origin/main` ≤ 3600 s.
3. Tests shell + Rust (AC-6, AC-NET-1).
4. Cierre documental y evolution.
