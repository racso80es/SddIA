---
feature_name: execute-process-phase-failure-propagation
created: "2026-08-11"
updated: "2026-08-13"
process: bug-fix
branch_name: fix/execute-process-phase-failure-propagation
persist_ref: docs/fixes/execute-process-phase-failure-propagation
pbi_ref: docs/todos/done/[FIX] execute-process — fallo de fase debe fallar ejecución global (EV-AUD-005).md
document_id: 04f8f435-450b-477a-970a-4a05dd0224cb
finding: EV-AUD-005
correlation_id: dcb9efed-2268-4298-8108-7a55cf4db323
items_applied:
  - F1-phase-terminal-helper
  - F2-residual-propagation
  - F3-executor-parity
  - F4-delivery-close-parity
  - F5-capsule-smoke-parity
  - F6-failed-phase-fields
  - F7-thermodynamic-pec-telemetry
  - F8-unit-regression-tests
  - F9-isolate-diff-via-a
  - F10-prune-legacy-persist-ref
  - F11-cargo-test-phase-terminal
---

# Ejecución — EV-AUD-005 phase failure propagation

## Cambios materializados

| Item | Estado | Evidencia |
|------|--------|-----------|
| H1 `phase_terminal.rs` | aplicado | módulo + `#[cfg(test)]` T1–T9 / fail-soft / code priority / regresión `62b201cf` |
| H2 `mod.rs` | aplicado | `pub mod phase_terminal` |
| H3 `residual_runner::run_generic` | aplicado | `state.phase_reports` + helper + `failed_phase*` + peaje con `verdict.success` |
| H4 `executor::run_generic` | aplicado | misma agregación |
| H5 `delivery_close` | aplicado | helper compartido |
| H6 `capsule_invoke_smoke` | aplicado | helper; simulated no inducen fallo |
| H7 `thermodynamic` | aplicado | PEC + telemetría con `failed_phase*` cuando `!success`; **sin** early-PEC Kalma2 (`awaiting_agents`) |
| H8 aislamiento Vía A | aplicado | WIP Kalma2 en `stash@{0}` `via-b-kalma2-ola-wip-post-ev-aud-005`; slug legado eliminado |

## Regresión incidente `62b201cf-0d82-4153-8c7d-8223233cf476`

Fixture `t9_regression_62b201cf_persistencia_oficial`: fase `Persistencia oficial` / `CERBERO_CONFIG_ERROR` → `success:false`, `status_code≠0`, `data.failed_phase*`.

## Verificación local (2026-08-13 · retoma Vía A)

Comando (host, cwd `SddIA/`):

```bash
cd SddIA && env -u CARGO_TARGET_DIR cargo test -p execute-process --lib phase_terminal
```

Stdout físico:

```text
   Compiling execute-process v0.1.0 (/home/racso/Proyectos/SddIA/SddIA/engine/execute-process)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.62s
     Running unittests src/lib.rs (target/debug/deps/execute_process-cb59800795367fb7)

running 13 tests
test engine::phase_terminal::tests::fail_soft_declared_does_not_fail_global ... ok
test engine::phase_terminal::tests::t2_skipped_simulated_awaiting_are_not_failure ... ok
test engine::phase_terminal::tests::code_priority_cerbero_before_di_gate ... ok
test engine::phase_terminal::tests::blocked_status_is_global_failure ... ok
test engine::phase_terminal::tests::t1_mix_executed_and_failed_is_global_failure ... ok
test engine::phase_terminal::tests::t3_cerbero_config_error_code_propagated ... ok
test engine::phase_terminal::tests::t2b_argos_block_without_phase_failure ... ok
test engine::phase_terminal::tests::t5_cerbero_rbac_failed_fixture ... ok
test engine::phase_terminal::tests::t6_capsule_invoke_failed_fixture ... ok
test engine::phase_terminal::tests::t4_di_gate_failed_fixture ... ok
test engine::phase_terminal::tests::t7_agent_runtime_failed_fixture ... ok
test engine::phase_terminal::tests::t9_regression_62b201cf_persistencia_oficial ... ok
test engine::phase_terminal::tests::t8_persistencia_capability_failed ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 144 filtered out; finished in 0.00s
```

| Paso | Resultado |
|------|-----------|
| Materialización código + docs canónicos | **ok** |
| Aislamiento diff (CA8) | **ok** — solo touchpoints EV-AUD-005 + `persist_ref` canónico |
| Poda slug legado (CA9) | **ok** — `docs/fixes/execute-processfallodefasedebefallarejecucinglobalev-aud-005/` eliminado |
| `cargo test -p execute-process --lib phase_terminal` | **ok · 13 passed** (incluye `t9_regression_62b201cf_persistencia_oficial`) |

## Notas

- `persist_ref` único: `docs/fixes/execute-process-phase-failure-propagation`.
- Ola Kalma2 (K1–K6) **no** forma parte de este sellado; WIP en stash `via-b-kalma2-ola-wip-post-ev-aud-005`.
- Git: commit/PR vía `skill:git-manager` / `delivery-close-cycle`.
