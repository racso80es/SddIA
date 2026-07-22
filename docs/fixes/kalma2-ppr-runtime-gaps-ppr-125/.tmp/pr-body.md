## Summary

- Handlers nativos PPR: Prep con `git-manager` (G2) y Triaje técnico formal `verify_process_integrity` (G1).
- G4: coalesce `pr_branch` → `branch_name` en route/agent-runtime/prompt Kalma2.
- G3: prompt KM — seeds `docs/todos/` solo vía Cumulo / `Kaizen_Alert_Required`.

## Test plan

- [x] `cargo test -p execute-process --lib pull_request_review`
- [x] `cargo test -p execute-process --lib branch_name_coalesces`
- [x] Smoke `./sddia-run.sh --process pull-request-review` → Prep+F3 executed