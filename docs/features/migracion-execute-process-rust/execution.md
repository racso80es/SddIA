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
| **C** Engine (parcial) | ✅ | Handler nativo `kalma2-interact`; delegación Python vía bridge para el resto |
| **D** Forges | ⏳ | Pendiente — sigue en bridge Python |
| **E** Touchpoints | 🔶 | Kalma2, `sddia-run.sh`, `event-watcher` actualizados; hooks/telegram-watcher pendientes |
| **F** Poda | ⏳ | `execute-process.py` se mantiene como fallback |

## 2. Artefactos forjados

| ID | Ruta | Estado |
|----|------|--------|
| C1 | `SddIA/engine/execute-process/Cargo.toml` | creado |
| C2 | `SddIA/engine/execute-process/src/main.rs` | creado |
| C3 | `SddIA/engine/execute-process/src/core/` | creado |
| C4 | `SddIA/engine/execute-process/src/engine/` | creado (handlers + delegate) |
| C5 | `SddIA/engine/execute-process/src/forges/` | — pendiente |
| C6 | `SddIA/Cargo.toml` | actualizado (`engine/*`) |
| — | `SddIA/scripts/qa/_execute_process_engine_bridge.py` | creado (bridge Fase C) |
| T5 | `sddia-run.sh` | actualizado (binario nativo preferente) |
| T6 | `.SddIA/client/sddia-client-bridge.py` | actualizado |
| T1 | `SddIA/daemons/event-watcher/src/main.rs` | actualizado |

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
| Procesos complejos (`feature`, `route-domain-event`) | 🔶 delegan a bridge Python (requiere PyYAML en PYTHON) |

## 4. Deuda técnica explícita

1. **Motor genérico:** `engine::delegate_python` invoca `_execute_process_engine_bridge.py` → `execute_process_capsules.run_process`. Eliminar cuando Fase C complete porte de `executor`/`capsules`/`forges`.
2. **`execute-action.py`:** permanece Python subprocess (deuda separada).
3. **Touchpoints pendientes:** `telegram-watcher`, git-hooks (`hook_common.py`), lanzadores `.sh/.bat` de daemons.
4. **Golden harness:** no forjado aún (plan Fase A); requerido antes de poda del `.py`.
5. **`requirements.txt`:** mantener mientras bridge + scripts QA consuman PyYAML (clarify D6).

## 5. Variables de entorno

| Variable | Efecto |
|----------|--------|
| `SDDIA_EXECUTE_PROCESS_BIN` | Ruta absoluta al binario orquestador (override SSOT) |
| `PYTHON` | Intérprete para bridge/delegación legacy |

## 6. Próximo hito

Forja Fase C completa: portar `run_workspace_init`, bucle de fases genérico y Peaje Termodinámico a Rust; golden tests contra Python; luego Fase E restante + Fase F poda.
