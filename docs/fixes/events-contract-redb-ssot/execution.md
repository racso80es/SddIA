---
feature_name: events-contract-redb-ssot
created: "2026-10-29"
process: bug-fix
branch_name: fix/events-contract-redb-ssot
pbi_ref: AUT-45
execution_id: "a629489e-2c6c-41fb-89db-26484a96fbd9"
---

# Ejecución

1. `refine-item` AUT-45 (relevo IDE).
2. `bug-fix` workspace-init `a629489e-2c6c-41fb-89db-26484a96fbd9`.
3. Parche contrato + store + test.
4. `cargo test -p execute-process terminal_witness_exists_redb --test event_store_writers -- --test-threads=1`.
