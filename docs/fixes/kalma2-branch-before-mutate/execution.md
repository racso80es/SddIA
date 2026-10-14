---
feature_name: kalma2-branch-before-mutate
created: "2026-10-14"
process: bug-fix
branch_name: fix/kalma2-branch-before-mutate
persist_ref: docs/fixes/kalma2-branch-before-mutate
---

# Ejecución — kalma2-branch-before-mutate

## Entrega código

PR [#347](https://github.com/racso80es/SddIA/pull/347) → merge `d17200c6060071ce08d04759b7a7344d83f40dd5` en `main`.

## Tests (BR-CA6)

```text
cargo test -p execute-process --lib run_dirty_outside_scope_aborts_without_system_fracture
cargo test -p execute-process --lib run_checkout_failure_stops_before_base_and_persist
cargo test -p execute-process --lib run_clean_tree_checks_out_work_branch_before_base_sync
cargo test -p execute-process --lib workspace_init_failed_skips_design_execution_and_verification
cargo test -p execute-process --lib end_message_includes_agent_verdict short_verdict_prefers_phase_message_then_handoff
cargo test -p kalma2-bridge --bin kalma2-bridge project_status_phase_end_survives_later_awaiting_agents_pec
```

Resultado local: OK (2026-10-14).

## Cierre documental

Este `persist_ref` y PBI en `docs/todos/done/` en el PR de cierre documental asociado.
