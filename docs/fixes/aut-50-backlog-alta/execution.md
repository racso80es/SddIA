---
feature_name: aut-50-backlog-alta
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-50-backlog-alta
tracker_ref: AUT-50
execution_id: "82abfda6-47a2-41f6-855f-bddbc8546bda"
---

# Ejecución — AUT-50

```bash
cargo test -p execute-process aut50 --lib -- --test-threads=1
cargo test -p linear-tracker-adapter list_issues_query
./sddia-run.sh --process entity-manager --inputs-file .tmp/entity-manager-forge-hu.json
```

Commits planificados: C1–C4 (forge-pbi, linear-adapter, forge-hu+genoma, refine-item+store).
