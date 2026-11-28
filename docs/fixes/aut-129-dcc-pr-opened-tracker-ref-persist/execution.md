---
feature_name: aut-129-dcc-pr-opened-tracker-ref-persist
created: "2026-11-28"
process: bug-fix
branch_name: fix/aut-129-dcc-pr-opened-tracker-ref-persist
persist_ref: docs/fixes/aut-129-dcc-pr-opened-tracker-ref-persist
tracker_ref: AUT-129
execution_id: "f491f6fc-fa55-488a-8299-742b816a254c"
---

# Ejecución — AUT-129

1. `SDDIA_ALLOW_STALE_RUNTIME=0 SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_ALLOW_DIRTY=1 ./sddia-run.sh --process bug-fix --inputs-file .tmp/bug-fix-aut-129-init.json` — `execution_id` `f491f6fc-fa55-488a-8299-742b816a254c`, rama `fix/aut-129-dcc-pr-opened-tracker-ref-persist`.
2. Parche en `lifecycle_phase_action.rs` + test AC4.
3. `cargo test -p execute-process --lib lifecycle_phase_action` — 6/6 OK (incl. regresión persist).
4. `cargo build -p execute-process --release` (KA-TOR-01).
5. Evolución `c9d4bfe5-d3be-4c36-b8af-ec0c114e9ec8`.
6. Commit vía cápsula directa `SddIA/target/release/git-manager` + `delivery-close-cycle`.