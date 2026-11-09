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
agent: tekton
phase: ejecucion
status: inventory-closed-verify-blocked
spec_ref: "spec.md@1.2.0"
plan_ref: "plan.md@1.2.0"
norm_ref: features-documentation-pattern@1.3.1
execution_id: "ceccf5b7-2243-4505-9387-ec50a3af2a04"
---
# Ejecución — man-37-eda-test-pyramid

## Resumen Tekton (`08a7564d-…`)

| Tajo | Resultado |
|------|-----------|
| T0.V | Código harness **presente** (lectura MCP). Smoke `cargo test` **no** ejecutable vía MCP (`run_check` denegado). |
| T1 | Inventario machine-readable **cerrado** — cero `pending` (abajo). |
| T2 | Migrados canónicos verificados; residual I/O → T3 allowlist. |
| T3 | `integration_src` + tools = reclasificación documentada (**L-INVENTORY** D4.7). |
| T4.V / T4.P | Gate+CI presentes; paridad toml↔gate **OK**; smoke CLI **bloqueado** (whitelist MCP vacía). |
| T5 | Taxonomía canónica `spec.md` §5; refs en objectives actualizadas. |
| T6 | **blocked** — sin evidencia gate/tests en este runtime. |

## Inventario T1 (machine-readable) — AC-5

Formato: `path | símbolo/foco | decisión | estado`

| path | símbolo/foco | decisión | estado |
|------|--------------|----------|--------|
| `SddIA/engine/execute-process/src/engine/eda_port.rs` | `EventManagerTestHarness` + faults | migrate | done |
| `SddIA/engine/execute-process/src/engine/eda_port.rs` | `scoped_tests` | migrate | done |
| `SddIA/engine/execute-process/src/engine/eda_port.rs` | `test_lab` FS/Redb helpers | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/eda_port_legacy.rs` | bridge StoreMode | allowlist | allowlisted |
| `SddIA/engine/execute-process/src/engine/domain_mutation.rs` | tests emit | migrate | done |
| `SddIA/engine/execute-process/src/engine/ecst_input_loader.rs` | tests verify mock | migrate | done |
| `SddIA/engine/execute-process/src/engine/tracker_pbi_meta.rs` | tests presented/store | migrate + allowlist | done |
| `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/delivery_close.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/actions.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/accept_pr.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/aiua_intent.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/progress_trace.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/suite_execution_requested.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/workspace_init.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/thermodynamic.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/cli_detach.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/persist_pec_correlation_proof.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/domain_authority.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/capability_di_reactor.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/linear_direct_cycle_e2e.rs` | e2e-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/linear_done_gate_both_e2e.rs` | e2e-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/handlers/system_vitality.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/handlers/tracker_markdown_apply.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/handlers/tracker_sync_replay.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/handlers/kalma2*` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/handlers/user_preference.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/handlers/email_*` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/capability_di_gate.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/capability_di_output_validator.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/cerbero_di_envelope.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/cerbero_di_rbac.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/src/engine/materialize_kaizen_alert_doc.rs` | integration-in-src | reclassify | allowlisted |
| `SddIA/engine/execute-process/tests/**` | matrices I/O | reclassify | allowlisted |
| `SddIA/sddia-core/.../contract_suite.rs` | contract suite | allowlist | allowlisted |
| `SddIA/sddia-event-store/**` | adapters/store | allowlist | allowlisted |
| `SddIA/infrastructure/**` | adapters | allowlist | allowlisted |
| `SddIA/sddia-event-manager-host/**` | host | allowlist | allowlisted |
| `SddIA/tools/sddia-qa/src/gate_eda_unit_no_real_backend.rs` | gate MAN-37 | allowlist | allowlisted |
| `SddIA/tools/sddia-qa/src/gate_event_manager_port.rs` | gate MAN-35 | allowlist | allowlisted |
| `SddIA/interfaces/kalma2-bridge/**` | e2e/tools | reclassify | allowlisted |
| `SddIA/tools/event-bus-audit/**` | e2e/tools | reclassify | allowlisted |
| `SddIA/tools/repo-retention-purger/**` | e2e/tools | reclassify | allowlisted |
| `SddIA/tools/thought-graph-access/**` | e2e/tools | reclassify | allowlisted |

**Filas `pending`: 0.** Fuente allowlist: `SddIA/tools/sddia-qa/allowlists/eda-unit-backend.toml`.

## T4.P — Paridad allowlist ↔ gate

| Vector | Veredicto |
|--------|-----------|
| `path_contains` /tests/, contract_suite, matrices | espejado en gate |
| `path_suffixes` eda_port(_legacy)/gates | espejado |
| `path_prefixes` store/infra/host | espejado |
| `integration_src` exact paths | espejado en `INTEGRATION_SRC` |
| `handlers/kalma2` / `handlers/email_` | gate vía `contains` |
| `integration_tools_prefixes` | espejado |
| Delta código esta pasada | **ninguno** (paridad ya alineada) |

## Comandos de verificación (referencia; **no** corridos aquí)

```bash
cd SddIA && CARGO_TARGET_DIR=target cargo test -p execute-process test_harness --lib
cd SddIA && CARGO_TARGET_DIR=target cargo build -p sddia-qa
SddIA/target/debug/sddia-qa gate-eda-unit-no-real-backend --json
SddIA/target/debug/sddia-qa gate-event-manager-port --json
```

### Evidencia MCP (esta pasada)

| Intento | Resultado |
|---------|-----------|
| `run_check` executable=`cargo` | `CERBERO_DENY: executable not in whitelist` |
| `run_check` executable=`sddia-qa` | `CERBERO_DENY: executable not in whitelist` |
| Causa | `.SddIA/project.md` sin `mcp.allowed_executables` → set vacío en workspace-server |
| Binarios en disco | `SddIA/target/debug/sddia-qa` **presente** (listado MCP); no invocable por aduana |

## AC checklist (honesto)

| AC | Estado |
|----|--------|
| AC-1 harness canónico | código OK; adopción residual cubierta por migrate+allowlist |
| AC-2 FaultInjection | tres canónicos en harness presentes |
| AC-3/AC-4 L-STRIP frontera | gate+allowlist presentes; **verde local no evidenciado** |
| AC-5 inventario | **cerrado** (tabla sin pending) |
| AC-6 gate CI | cableado; **smoke local blocked** |
| AC-7 taxonomía | `spec.md` §5 + refs |
| AC-8 validacion APTO | **no** — falta T6 |

## Orquestador

| Intento | Resultado |
|---------|-----------|
| Diseño Dedalo v1.2.0 | sellado (harness/gate Presentes) |
| Tekton ejecución residual | docs forja + inventario; verify **blocked** por Cerbero MCP |
