---
feature_name: kalma2-aiua-perceptive-bridge
created: "2026-09-09"
process: feature
branch_name: feat/kalma2-aiua-perceptive-bridge
persist_ref: docs/features/kalma2-aiua-perceptive-bridge
execution_id: "f075b5ea-aac3-47cb-b5ba-a18c5431a7e5"
document_id: PBI-NUCLEO-PUENTE-PERCEPTIVO-KALMA2
items_applied:
  - L1-spawn-param
  - L2-endpoint-aiua
  - L3-wui-pulse
  - L4-tests
---

# Ejecución — kalma2-aiua-perceptive-bridge

## Init

`execution_id` `f075b5ea-aac3-47cb-b5ba-a18c5431a7e5`. Relé IDE (`SDDIA_AGENT_RELAY_IDE=1`). Commit planificación `70289a4`.

## L1–L2 puente

`spawn_orchestrator` parametriza `--process`. `run_orchestrator` conserva `kalma2-interact`. `handle_aiua_interact` spawnea `aiua-stimulus-processing`. Timeout = max(`SDDIA_CLIENT_TIMEOUT_SECONDS` default 120, `SDDIA_GEMINI_HTTP_TIMEOUT_SECS`). Fractura solo ELF/spawn (`aiua_interact`).

## L3 WUI

`#aiua-pulse` + `enviarAiuaStimulus`. `setBusy` deshabilita los cuatro botones. `appendProgressTrace` objeto PTC. `Ctrl+Enter` → `enviarChat`.

## L4 tests

```text
TMPDIR=<home> CARGO_TARGET_DIR=SddIA/target cargo test --manifest-path SddIA/interfaces/kalma2-bridge/Cargo.toml --bin kalma2-bridge
# 32 passed
```

## Evolution

`sddia-qa evolution-register` → `bdd512bd-42ea-4c45-ab97-2e0fa8ea37c8` (`EVOL_OK`, `alta`).

## CA-CI

`PENDIENTE-CI` hasta `run_id` verde del PR. No `global: APTO` ni archivo PBI hasta entonces.
