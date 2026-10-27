---
feature_name: gate-evolution-evol-stem-not-v4
created: "2026-10-27"
process: bug-fix
branch_name: fix/gate-evolution-evol-stem-not-v4
persist_ref: docs/fixes/gate-evolution-evol-stem-not-v4
execution_id: "5fb64f21-06f9-4e20-af62-a81d6de226da"
---

# Ejecución

1. `SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_ALLOW_DIRTY=1 ./sddia-run.sh --process bug-fix` — workspace-init, rama `fix/gate-evolution-evol-stem-not-v4`, `pbi_ref: AUT-9`.
2. Parche en `sddia-evolution-register` + contrato evolution.
3. `cargo test -p sddia-evolution-register --lib`.
4. `cargo build --target wasm32-wasip1 -p sddia-evolution-register` (cápsula WASI en `SddIA/target/wasm32-wasip1/debug/`).
5. `sddia-qa evolution-register` alta `03eb13b4-f7d5-419a-99e9-e374f342eed2`.
6. `delivery-close-cycle` `execution_id` `54827c83-19c7-4d17-a0f5-fc5d33281ec5`. Snapshot `f215e0ef`. PR https://github.com/racso80es/SddIA/pull/401. `PullRequest_Presented` `324d1842-adb4-482b-989a-480f125fb54a`.
7. Commit `f215e0ef`: poda ruido colateral (daemons/tools/tracker-outbound) tras primer DCC bloqueado en aduana evolution.
