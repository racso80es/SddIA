---
feature_name: kaizen-feature-lab-init-frictions
created: "2026-08-28"
process: feature
execution_id: "80a3ca0d-80c5-4662-ab12-2afe757478c8"
items:
  - T1-timeout-burial-reentry
  - T2-relay-vault-parity
  - T3-execution-id-trace
  - T4-daemon-circuit
  - T5-hygiene
  - T6-tests-smoke
---

# Implementation — kaizen-feature-lab-init-frictions

## T1 · Timeout + entierro + reentrada

| Touchpoint | Cambio |
|------------|--------|
| `SddIA/engine/execute-process/src/engine/agent_runtime.rs` | `wait` acotado (default 660s); PGID + `kill` grupo; `SDDIA_AGENT_RUNTIME_DEPTH`; tests |
| `SddIA/engine/execute-process/Cargo.toml` | dep `libc` |

## T2 · Relé + paridad bóveda

| Touchpoint | Cambio |
|------------|--------|
| `agent_runtime.rs` | `SDDIA_AGENT_RELAY_IDE` → `is_configured() == false` + log stderr |
| `SddIA/scripts/common/sddia_shell_lib.sh` | `_sddia_load_vault` setdefault; precedencia IOTA |
| `SddIA/engine/execute-process/src/core/env.rs` | `vault_precedence_keys()` export |

Genoma: `external-ai-constraints.md` v1.6.1 — párrafo de relevo IDE en DA-5.

## T3 · Trazabilidad

| Touchpoint | Cambio |
|------------|--------|
| `agent_runtime.rs` | `execution_id` en payload; guard `persist-execution-id-conflict` |
| `SddIA/scripts/tools/kalma2-agent-runtime-cursor.py` | prompt + handoff con `execution_id` |
| `SddIA/engine/execute-process/src/engine/workspace_init.rs` | stub `objectives.md` con `execution_id` |

## T4 · Circuito daemon

| Touchpoint | Cambio |
|------------|--------|
| `entity_manager.rs` | clase `daemon` en piloto |
| `residual_runner.rs` | extirpado fail-soft `daemon-creator` |
| `forges/common.rs` | `sync_daemons_index_census` |
| `forges/factory.rs` | censo tras `run_daemon_forge` |
| `SddIA/daemons/index.md` | pie 6 Centinelas |

Genoma: `entity-manager.md` v1.0.2 forjado vía `./sddia-run.sh --process entity-manager`
(`.tmp/em-update-entity-manager.json`); `process/index.md` y `eda-coverage.json` sellados en consecuencia.

## T5 · Higiene

| Touchpoint | Cambio |
|------------|--------|
| `workspace_init.rs` | gate `dirty-worktree` + `SDDIA_LAB_ALLOW_DIRTY` |
| `phase_capsules.rs` | snapshot preserva `??` `docs/todos/` ajeno |

## T6 · Tests y smokes

| Touchpoint | Cambio |
|------------|--------|
| `agent_runtime.rs` | 12 tests: timeout, PGID, reentrada, conflicto `execution_id`, relevo |
| `phase_capsules.rs` | tests de preservación `??` bajo `docs/todos/` |
| `forges/common.rs` | test de censo del índice de daemons |
| `.tmp/smoke-lab-init.sh` | 9 smokes LAB-CA1…CA11 |

`verify_process_integrity.rs`: el fixture del test creaba un tempdir sin `SddIA/core/cumulo.paths.json`,
por lo que `load_paths_config` abortaba antes de verificar. Fixture completado — defecto preexistente,
ajeno al alcance del PBI, corregido aquí por bloquear la suite verde.
