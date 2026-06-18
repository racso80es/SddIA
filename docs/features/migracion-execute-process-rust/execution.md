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
| **C** Engine (parcial) | ✅ | Handlers satélite nativos (salvo `route-domain-event`); motor genérico P1–P3 |
| **D** Forges | ⏳ | Pendiente — sigue en bridge Python |
| **E** Touchpoints | ✅ | Kalma2, wrappers, watchers, hooks, EDA/route lab, README |
| **F** Poda | ⏳ | Gate P9 parcial (4 casos); `.py` fallback activo |

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
| — | `SddIA/scripts/qa/_execute_process_handler_bridge.py` | creado (handlers satélite P4) |
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
| `cargo test -p execute-process` | ✅ 7 tests |
| Smoke `kalma2-interact` nativo | ✅ envelope JSON válido |
| Smoke `feature` nativo (P1–P3, skips lab) | ✅ 7 fases, `success:true` |
| Golden `kalma2-interact` Rust vs Python | ✅ |
| Golden `feature` (skips lab + `SDDIA_LAB_SKIP_GIT`) | ✅ |
| Golden `telegram-fallback-responder` (filtro + chat_id ausente) | ✅ |
| Golden `telegram-gateway` (TODO + empty text) | ✅ |
| Golden daemon handlers (heartbeat, governance status, kill-switch) | ✅ |
| `HANDLER_BRIDGE` residual | 🔶 solo `route-domain-event` |
| `SDDIA_LAB_SKIP_GIT` (Rust + Python workspace_init) | ✅ |
| Handlers satélite restantes | 🔶 vía `_execute_process_handler_bridge.py` |
| Lanzadores `SddIA/scripts/daemons/*` | ✅ N/A — no invocan `execute-process.py` (grep verificado) |

## 4. Deuda técnica explícita

1. **Handlers satélite:** `_execute_process_handler_bridge.py` (P4 parcial); porte nativo pendiente para `route-domain-event`, telegram, kill-switch, governance.
2. **Motor legacy residual:** procesos no cubiertos por `executor`/`handlers` siguen en `_execute_process_engine_bridge.py`.
3. **`execute-action.py`:** permanece Python subprocess (deuda separada).
4. **Touchpoints pendientes:** lanzadores `SddIA/scripts/daemons/*.{sh,bat}`, `_exec_daemon.py`, `_launch.sh` (P12).
5. **Golden harness:** P8 inicial (`kalma2-interact`); ampliar a `feature`, `route-domain-event`, `entity-manager` (P9).
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
| P4 | Handlers satélite nativos (sacar de `HANDLER_BRIDGE`) | golden por handler |
| P5 | Cápsulas `wasmtime` nativas (`engine::capsules`) | target `wasm32-wasip1` |
| P6/P7 | Forjas Rust (`hash_signature` sha256 paridad + índice idempotente) | vía `entity-manager` (DA-2/DA-3) |
| P9 | Ampliar golden a `feature`, `bug-fix`, `route-domain-event`, `entity-manager`, `delivery-close-cycle` | **gate maestro** que habilita P10–P17 |
| P12 | Lanzadores `SddIA/scripts/daemons/*.{sh,bat}` | ✅ N/A verificado (sin referencias a `.py`) |
| P15 | DA-3 vía canónica en `external-ai-constraints.md` | `entity-manager` (genoma) |
| P17 | Retirar `.py` + bridges | gate duro: P9 + P4 + P5 + P6/P7 + E2E + CA-7/CA-8 verdes |

Orden y gating completos en `implementation.md` §6.6 y §7.6.
