---
feature_name: man-37-eda-test-pyramid
created: "2026-11-09"
updated: "2026-11-09"
process: refactorization
branch_name: refactor/man-37-eda-test-pyramid
persist_ref: docs/features/man-37-eda-test-pyramid
pbi_ref: MAN-37
parent_ref: MAN-35
document_id: PBI-94396a84
tracker_ref: MAN-37
execution_id: "08a7564d-06ee-4558-8b37-9666572bcf00"
agent: tekton
phase: ejecucion
status: residual-docs-closed-verify-blocked
spec_ref: "spec.md@1.2.0"
plan_ref: "plan.md@1.2.0"
norm_ref: features-documentation-pattern@1.3.1
---
# Implementación — man-37-eda-test-pyramid

Tekton (Vértice Productivo) — fase **Ejecución** residual sobre blueprint `plan.md` **v1.2.0** / `spec.md` **v1.2.0**. **No** se re-forjaron T0/T4 (ya presentes en working tree).

## T0 — EventManagerTestHarness (forjado; sin reescritura)

| Área | Estado |
|------|--------|
| `SddIA/engine/execute-process/src/engine/eda_port.rs` → `test_harness` | RAII `EventManagerTestHarness` sobre `install_port` / `clear_port` + `ParallelLabGuard` |
| Tests canónicos | `harness_fail_write_*`, `harness_steal_lease_*`, `harness_duplicate_ack_*` |
| `scoped_tests` | `mock_stays_installed_across_ops` vía harness |
| Export | `pub use test_harness::EventManagerTestHarness` bajo `#[cfg(test)]` |

## T2 — Migración unitarios (cierre residual documentado)

| Path | Cambio |
|------|--------|
| `domain_mutation.rs` tests | `_h = EventManagerTestHarness::new()` |
| `ecst_input_loader.rs` tests | harness + asserts sobre mock/puerto |
| `tracker_pbi_meta.rs` tests EDA | harness en `presented_in_store_*` / `presented_event_id_*` |
| Residuales I/O real bajo `#[cfg(test)]` | **T3** allowlist `integration_src` (no migrate forzado) |

## T3 / T4 — Allowlist + gate (forjados; paridad revisada)

| Área | Estado |
|------|--------|
| `SddIA/tools/sddia-qa/src/gate_eda_unit_no_real_backend.rs` | Gate unitario L-STRIP; `path_allowlisted` espejo operativo |
| `SddIA/tools/sddia-qa/allowlists/eda-unit-backend.toml` | Versionada; `integration_src` + tools prefixes |
| `sddia-qa` CLI | `gate-eda-unit-no-real-backend` |
| `.github/workflows/sddia-index-qa.yml` | Paso tras `gate-event-manager-port` |
| Gate MAN-35 | Intactó (`gate_event_manager_port`) |
| **T4.P** | Paridad toml ↔ hardcode `INTEGRATION_SRC` / contains / prefixes **revisada OK** (sin delta de código en esta pasada) |

## T1 / T5 — Docs de esta pasada

| Artefacto | Rol |
|-----------|-----|
| `execution.md` | Inventario machine-readable **sin filas `pending`** |
| `objectives.md` | Refs taxonomía → `spec.md` §5 **v1.2.0** |
| Este fichero | Sello de forja residual Tekton |

## T6 — Verificación

**No ejecutada en este runtime:** MCP `run_check` → `CERBERO_DENY: executable not in whitelist` (`.SddIA/project.md` sin `mcp.allowed_executables`). Binarios locales `SddIA/target/debug/sddia-qa` / workspace cargo **existen**, pero la aduana MCP no permite invocarlos. **No** se declara AC-6/AC-8 verdes sin evidencia. Ver `execution.md` / `validacion.md`.

## Fuera de alcance respetado

- Sin reabrir MAN-35 / MAN-38.
- Sin TODOs bajo `docs/todos/`.
- Sin reescritura de `_agent_handoff.md`.
