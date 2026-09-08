---
feature_name: nucleo-aiua-tormentosa-motor
created: "2026-09-08"
process: feature
branch_name: feat/nucleo-aiua-tormentosa-motor
persist_ref: docs/features/nucleo-aiua-tormentosa-motor
execution_id: "21377cd5-079c-484c-b5d1-c8f8429de402"
document_id: PBI-NUCLEO-ARRANQUE-AIUA-TORMENTOSA
items_applied:
  - L1-topology
  - L2-genome-em
  - L3-tool-adapter
  - L4-handlers
  - L5-tests
---

# Ejecución — nucleo-aiua-tormentosa-motor

## Init

`execution_id` `21377cd5-079c-484c-b5d1-c8f8429de402`. Relé IDE (`SDDIA_AGENT_RELAY_IDE=1`). Commit planificación `e840ce5`.

## L1–L2

Topología `conscience/` + Cúmulo/README/Constitución. Genoma vía `entity-manager`: tool, 3 acciones, proceso.

## L3–L4

Crate `thought-graph-access`. Adaptador `open_with_bus` + emisión `Thought_Persisted`. Handler `aiua_stimulus` registrado antes de `residual_runner`. Acciones en `try_run_native`.

## L5 tests

```text
cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p thought-graph-access -p sddia-infrastructure-lancedb-thought
# thought-graph-access: 3 passed
# lancedb-thought: 6 passed (incl. store_thought_emits_thought_persisted_when_bus_configured)

cd SddIA && CARGO_TARGET_DIR=$PWD/target cargo test -p execute-process --lib -- aiua_stimulus
# 3 passed: invoke_aiua_core_concatenates_genome, lab_mock_empty_memories_yields_duration_and_thought_id, process_genome_has_no_kalma2_ui_coupling
```

Lab-mock: `SDDIA_LAB_MOCK_OUTBOUND=1`. LanceDB vacío → `memories: []`. `thought_id` 64 hex. `telemetry.duration_ms` presente. ECST en pending.

## WASI CI

`thought-graph-access` excluido de `SddIA/scripts/qa/build-wasi-capsules.sh` (host LanceDB; paridad `gemini-http-infer`). Finding: `wasi-runtime-smoke` en run `34222880281`.

## Evolution

`id_cambio` `4f009a69-e575-4d34-af85-e496d0dea370` vía `sddia-qa evolution-register`.

## CA-CI

`PENDIENTE-CI` hasta `run_id` verde del PR. No `global: APTO` ni archivo PBI hasta entonces.
