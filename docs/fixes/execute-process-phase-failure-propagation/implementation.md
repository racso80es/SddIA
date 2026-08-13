---
feature_name: execute-process-phase-failure-propagation
created: "2026-08-11"
process: bug-fix
branch_name: fix/execute-process-phase-failure-propagation
persist_ref: docs/fixes/execute-process-phase-failure-propagation
pbi_ref: docs/todos/done/[FIX] execute-process — fallo de fase debe fallar ejecución global (EV-AUD-005).md
document_id: 04f8f435-450b-477a-970a-4a05dd0224cb
finding: EV-AUD-005
correlation_id: dcb9efed-2268-4298-8108-7a55cf4db323
items:
  - F1-phase-terminal-helper
  - F2-residual-propagation
  - F3-executor-parity
  - F4-delivery-close-parity
  - F5-capsule-smoke-parity
  - F6-failed-phase-fields
  - F7-thermodynamic-pec-telemetry
  - F8-unit-regression-tests
---

# Implementación — EV-AUD-005 phase failure propagation

## Touchpoints

| # | Artefacto | Cambio |
|---|-----------|--------|
| H1 | `engine/phase_terminal.rs` | Helper `aggregate_execution_terminal` + `apply_failed_phase_fields`; fail-soft solo con `fail_soft: true` en report (forward-compat contractual); tests T1–T9 + fail-soft + prioridad de código |
| H2 | `engine/mod.rs` | `pub mod phase_terminal` |
| H3 | `engine/residual_runner.rs` | Fix primario: `state.phase_reports` antes del peaje; agregación vía helper; envelope `failed_phase*` |
| H4 | `engine/executor.rs` | Paridad: mismo helper + `failed_phase*` |
| H5 | `engine/delivery_close.rs` | Paridad agregador (deja de duplicar semántica ad-hoc) |
| H6 | `engine/capsule_invoke_smoke.rs` | Paridad; `simulated` neutrales (CA2) |
| H7 | `engine/thermodynamic.rs` | PEC `status=failed` + `failed_phase*`; telemetría `Raw_Execution_Finished` espeja causal cuando `!success` |

## Semántica (L1)

- Hard fail: `status ∈ {failed, blocked}` **sin** `fail_soft: true` → `success:false`, `status_code=1`, `exitCode=1`.
- Neutros: `executed` / `skipped` / `simulated` / `awaiting` / `awaiting_agents`.
- `argos_verdict=block` → fallo global (sin `failed_phase`).
- Código causal (orden): `cerbero_di_code` → `cerbero_envelope_di_code` → `di_gate_code` → `di_resolve_code` → `error_code`.
- Envelope `data`: `failed_phase`, `failed_phase_code`, `failed_phase_error`, `failed_phase_handler`.

## Fuera de alcance (sin tocar en esta fase)

- WIP ajeno sin causalidad EV-AUD-005: `kalma2.rs`, `task_queue_manager.rs`, `event-watcher`.
- Genoma / config Cerbero que originó `CERBERO_CONFIG_ERROR`.
- Cierre documental PBI / `validacion.md` (fase Argos + delivery).
