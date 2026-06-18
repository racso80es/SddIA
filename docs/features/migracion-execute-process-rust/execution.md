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
| **F** Poda | ⏳ | Gate P9 parcial (12 casos); `.py` fallback activo |

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
| — | `SddIA/scripts/qa/_execute_process_feature_phase_bridge.py` | creado (fases feature PBI/delivery) |
| — | `SddIA/scripts/qa/orchestrator_resolve.py` | creado (SSOT binario vs `.py`) |
| — | `SddIA/scripts/qa/golden_orchestrator_parity.py` | creado (P8 harness inicial) |
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
| `cargo test -p execute-process` | ✅ 11 tests |
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
| `HANDLER_BRIDGE` | ✅ eliminado — routing directo en `run_process` |
| `SDDIA_LAB_SKIP_GIT` (Rust + Python workspace_init) | ✅ |
| Core EDA `route-domain-event` | 🔶 `_execute_process_route_bridge.py` (deuda porte Rust) |
| Lanzadores `SddIA/scripts/daemons/*` | ✅ N/A — no invocan `execute-process.py` (grep verificado) |

## 4. Deuda técnica explícita

1. **Core EDA route:** entry nativo en Rust; lógica ECST/fan-out sigue en `route_domain_event_core.py` vía `_execute_process_route_bridge.py`.
2. **Motor legacy residual:** procesos no cubiertos por `executor`/`handlers` siguen en `_execute_process_engine_bridge.py`.
3. **`execute-action.py`:** permanece Python subprocess (deuda separada).
4. **Touchpoints pendientes:** lanzadores `SddIA/scripts/daemons/*.{sh,bat}`, `_exec_daemon.py`, `_launch.sh` (P12).
5. **Golden harness:** 12 casos verdes; pendiente `entity-manager` (P9).
6. **`requirements.txt`:** mantener mientras bridges + scripts QA consuman PyYAML (clarify D6).

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
| P5 | Cápsulas `wasmtime` nativas | ✅ delivery-close + try_invoke_delegates |
| P9 | Ampliar golden a `entity-manager` | **gate maestro** que habilita P10–P17 |
| P6/P7 | Forjas Rust (`hash_signature` sha256 paridad + índice idempotente) | vía `entity-manager` (DA-2/DA-3) |
| P12 | Lanzadores `SddIA/scripts/daemons/*.{sh,bat}` | ✅ N/A verificado (sin referencias a `.py`) |
| P15 | DA-3 vía canónica en `external-ai-constraints.md` | `entity-manager` (genoma) |
| P17 | Retirar `.py` + bridges | gate duro: P9 + P4 + P5 + P6/P7 + E2E + CA-7/CA-8 verdes |

Orden y gating completos en `implementation.md` §6.6 y §7.6.
