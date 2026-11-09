---
feature_name: man-37-eda-test-pyramid
created: "2026-11-09"
process: refactorization
branch_name: refactor/man-37-eda-test-pyramid
persist_ref: docs/features/man-37-eda-test-pyramid
tracker_ref: MAN-37
execution_id: "47b9678f-7ee1-4080-bd3e-12cf0d366069"
---
# Implementación — man-37-eda-test-pyramid

## T0 — EventManagerTestHarness

| Área | Cambio |
|------|--------|
| `execute-process/src/engine/eda_port.rs` | Módulo `test_harness`: RAII sobre `install_port` / `clear_port`; tests `fail_write`, `steal_lease`, `duplicate_ack_noop` |
| `scoped_tests` | Migrado a harness canónico (sustituye `install_port` ad hoc) |

## T2 parcial — migración unitarios

| Área | Cambio |
|------|--------|
| `domain_mutation.rs`, `ecst_input_loader.rs`, `tracker_pbi_meta.rs` (tests) | `_h = EventManagerTestHarness::new()` |

## T4 — Gate CI unitario

| Área | Cambio |
|------|--------|
| `sddia-qa/src/gate_eda_unit_no_real_backend.rs` | Escaneo `#[cfg(test)]` / `#[test]`; veto L-STRIP; allowlist integration |
| `sddia-qa/allowlists/eda-unit-backend.toml` | Integration-in-src + tools (T3 documentado) |
| `sddia-qa/src/main.rs` | Comando `gate-eda-unit-no-real-backend` |
| `.github/workflows/sddia-index-qa.yml` | Paso tras `gate-event-manager-port` |

## Residual T2/T3

Tests integration-in-src listados en allowlist (reclasificación T3); migración completa a `execute-process/tests/**` queda como kaizen futuro sin bloquear AC-1/AC-7 de esta entrega.
