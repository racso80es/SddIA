---
feature_name: migracion-execute-process-rust
created: "2026-06-18"
process: feature
branch_name: feat/migracion-execute-process-rust
persist_ref: docs/features/migracion-execute-process-rust
status: execution
---

# Execution — Orquestador `execute-process` en Rust nativo

Registro de la forja física (Fases A–C parcial + touchpoints E parcial).

## 1. Fases completadas

| Fase | Estado | Entregable |
|------|--------|------------|
| **A** Andamiaje | ✅ | Crate `SddIA/engine/execute-process/`, miembro `engine/*` en workspace |
| **B** Core | ✅ | `core::{parser,resolver,env,env_parse,repo}`, `envelope::OrchestratorEnvelope` |
| **C** Engine (parcial) | ✅ | Handlers satélite nativos (entry `route-domain-event` + core EDA bridge); motor genérico P1–P3 |
| **D** Forges | ⏳ | Pendiente — sigue en bridge Python |
| **E** Touchpoints | ✅ | Kalma2, wrappers, watchers, hooks, EDA/route lab, README |
| **F** Poda | ⏳ | Gate P9 parcial (13 casos); `.py` fallback activo |

## 2. Artefactos forjados

| ID | Ruta | Estado |
|----|------|--------|
| C1 | `SddIA/engine/execute-process/Cargo.toml` | creado |
| C2 | `SddIA/engine/execute-process/src/main.rs` | creado |
| C3 | `SddIA/engine/execute-process/src/core/` | creado |
| C4 | `SddIA/engine/execute-process/src/engine/` | creado (executor, workspace, thermodynamic, delegate_handler) |
| C5 | `SddIA/engine/execute-process/src/forges/` | — pendiente |
| C6 | `SddIA/Cargo.toml` | actualizado (`engine/*`) |
| — | `SddIA/scripts/qa/_execute_process_engine_bridge.py` | creado (bridge motor legacy residual) |
| — | `SddIA/scripts/qa/_execute_process_handler_bridge.py` | legacy (sin referencias desde `mod.rs`) |
| — | `SddIA/scripts/qa/_execute_process_route_bridge.py` | creado (core EDA Python para P4) |
| — | `SddIA/engine/execute-process/src/engine/delivery_close.rs` | creado (P5 delivery-close nativo) |
| — | `SddIA/engine/execute-process/src/engine/phase_capsules.rs` | creado (P5 handlers fase + try_invoke) |
| — | `SddIA/engine/execute-process/src/engine/capsule_paths.rs` | creado (D-P5.3 SSOT `compiled_capsules`) |
| — | `SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs` | creado (D-P6T.1 nativo) |
| — | `SddIA/engine/execute-process/src/engine/materialize_kaizen_alert_doc.rs` | creado (D-P6T.1 nativo) |
| — | `SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs` | creado (D-P6T.1 nativo — cierra inventario acciones físicas) |
| — | `SddIA/engine/execute-process/src/engine/domain_mutation.rs` | creado (`emit-domain-mutation` nativo) |
| — | `SddIA/engine/execute-process/src/engine/ecst_validation.rs` | creado (aduana ECST) |
| — | `SddIA/engine/execute-process/src/engine/eda_bus.rs` | creado (idempotencia bus) |
| — | `SddIA/engine/execute-process/src/engine/eda_coverage.rs` | creado (SSOT eda-coverage) |
| — | `SddIA/engine/execute-process/src/engine/capsule_invoke_smoke.rs` | creado (D-P5.2 golden fase tool) |
| — | `SddIA/process/capsule-invoke-smoke.md` | creado (proceso lab golden) |
| — | `SddIA/core/cumulo.paths.json` | actualizado (`compiled_capsules`) |
| — | `SddIA/scripts/qa/_execute_process_feature_phase_bridge.py` | creado (fases feature PBI/delivery) |
| — | `SddIA/scripts/qa/orchestrator_resolve.py` | creado (SSOT binario vs `.py`) |
| — | `SddIA/scripts/qa/golden_orchestrator_parity.py` | creado (P8 harness inicial) |
| — | `SddIA/engine/execute-process/src/forges/` | creado (P6/P7 forjas nativas) |
| — | `SddIA/scripts/qa/forge_parity.py` | creado (P6/P7 paridad hash) |
| — | `execute-process --forge` | CLI forja nativa |
| T5 | `sddia-run.sh` | actualizado (binario nativo preferente) |
| T6 | `.SddIA/client/sddia-client-bridge.py` | actualizado |
| T1 | `SddIA/daemons/event-watcher/src/main.rs` | actualizado |
| T2 | `SddIA/daemons/telegram-watcher/src/main.rs` | actualizado |
| T4 | `SddIA/scripts/qa/git-hooks/hook_common.py` | actualizado (`orchestrator_resolve`) |
| — | `SddIA/scripts/qa/route_domain_event_core.py` | actualizado (P13) |
| — | `SddIA/scripts/qa/run-eda-e2e-lab.py` | actualizado (P13) |
| D1 | `README.md` | actualizado (P14 parcial) |

## 3. Verificación local

```bash
# Compilar (target en SddIA/target/)
cd SddIA && cargo build -p execute-process

# Tests unitarios
cargo test -p execute-process

# Smoke kalma2-interact (sin PyYAML)
./target/debug/execute-process --process kalma2-interact \
  --inputs '{"prompt":"test rust"}'
# → success:true, response Mayeuta lab

# Wrapper
cd .. && ./sddia-run.sh --process kalma2-interact --inputs '{"prompt":"via wrapper"}'
```

Resultados (2026-06-18):

| Check | Resultado |
|-------|-----------|
| `cargo build -p execute-process` | ✅ |
| `cargo test -p execute-process` | ✅ 43 tests |
| `forge_parity.py` (P6/P7 hash + idempotencia) | ✅ |
| Smoke `kalma2-interact` nativo | ✅ envelope JSON válido |
| Smoke `feature` nativo (P1–P3, skips lab) | ✅ 7 fases, `success:true` |
| Golden `kalma2-interact` Rust vs Python | ✅ |
| Golden `feature` (skips lab + `SDDIA_LAB_SKIP_GIT`) | ✅ |
| Golden `bug-fix` (skips lab + `SDDIA_LAB_SKIP_GIT`) | ✅ |
| Golden `telegram-fallback-responder` (filtro + chat_id ausente) | ✅ |
| Golden `telegram-gateway` (TODO + empty text) | ✅ |
| Golden daemon handlers (heartbeat, governance status, kill-switch) | ✅ |
| Golden `route-domain-event` (`SDDIA_LAB_ROUTE_SYNC`, fixture ECST) | ✅ |
| Golden `delivery-close-cycle` (skips lab P5) | ✅ |
| Golden `capsule-invoke-smoke` (fase `tool:io-choke` ejecutada, D-P5.2) | ✅ |
| Golden harness total | ✅ **14/14** (incl. **`entity-manager`**) |
| `emit-domain-mutation` nativo (tests unitarios ECST + pending) | ✅ |
| `HANDLER_BRIDGE` | ✅ eliminado — routing directo en `run_process` |
| `SDDIA_LAB_SKIP_GIT` (Rust + Python workspace_init) | ✅ |
| Core EDA `route-domain-event` | 🔶 `_execute_process_route_bridge.py` (deuda porte Rust) |
| Lanzadores `SddIA/scripts/daemons/*` | ✅ N/A — no invocan `execute-process.py` (grep verificado) |
| Touchpoints P10–P13 (`sddia-run`, hooks, bridges, lab, limbo) | ✅ `orchestrator_resolve` SSOT |
| `touchpoint_orchestrator_audit.py` | ✅ audit P10–P13 |
| P14 `README.md` (Aduana Universal / entrypoints) | ✅ |
| P15 `external-ai-constraints.md` v1.2.0 (DA-3 orquestador) | ✅ |
| Smokes E2E CA-7 (`orchestrator_touchpoint_e2e_smoke.py`) | ✅ **8/8** |
| CA-8 parcial (binario nativo sin Python/PyYAML en orquestación) | ✅ smoke `native-without-python`; entrypoint binario-only |
| P17 poda (`execute-process.py`, bridges engine/handler/feature_phase) | ✅ |
| Bridge residual `_execute_process_capsules_bridge.py` | 🔶 creators/telemetry/accept-pr no portados |
| Bridge residual `_execute_process_route_bridge.py` | 🔶 core EDA route en Python |

## 4. Deuda técnica explícita

1. **Core EDA route:** entry nativo en Rust; lógica ECST/fan-out sigue en `route_domain_event_core.py` vía `_execute_process_route_bridge.py`.
2. **Motor legacy residual:** procesos no cubiertos por `executor`/`handlers`/`entity_manager` delegan a `_execute_process_capsules_bridge.py` → `execute_process_capsules.py`.
3. **`execute-action.py` (D-P6T.1):** ✅ inventario `PHYSICAL_HANDLERS` portado a Rust (`try_run_native`). El script permanece como fallback de cápsulas no compiladas.
4. **Touchpoints:** ✅ P10–P13 cerrados (`orchestrator_resolve` binario-only; audit `touchpoint_orchestrator_audit.py`).
5. **Golden harness:** ✅ **14/14** (`entity-manager` nativo P17).
6. **`requirements.txt`:** mantener mientras bridges + scripts QA consuman PyYAML (P16; fuera del entrypoint orquestador).

## 5. Variables de entorno

| Variable | Efecto |
|----------|--------|
| `SDDIA_EXECUTE_PROCESS_BIN` | Ruta absoluta al binario orquestador (override SSOT) |
| `PYTHON` | Intérprete para bridge/delegación legacy |

## 6. Próximo hito

Especificación accionable detallada de los pendientes gated en `implementation.md` §7:

| ID | Pendiente | Gate |
|----|-----------|------|
| P4 | Handlers satélite nativos | ✅ entry nativo; core EDA route en bridge |
| P5 | Cápsulas `wasmtime` nativas + deudas §6.bis | ✅ actions nativas, SSOT, golden capsule-invoke-smoke |
| P9 | Ampliar golden a `entity-manager` | ✅ **14/14** — gate maestro habilita P10–P17 |
| P6/P7 | Forjas Rust (`forges::factory`) | ✅ nativo + `forge_parity.py` |
| P10–P13 | Touchpoints producción → `orchestrator_resolve` | ✅ audit verde |
| P14 | `README.md` — invocación orquestador | ✅ |
| P15 | `external-ai-constraints.md` DA-3 v1.2.0 | ✅ |
| P12 | Lanzadores `SddIA/scripts/daemons/*.{sh,bat}` | ✅ N/A verificado (sin referencias a `.py`) |
| P17 | Retirar `.py` + bridges | gate duro: P9 + P4 + P5 + P6/P7 + E2E + CA-7/CA-8 verdes |

Orden y gating completos en `implementation.md` §6.6 y §7.6.
