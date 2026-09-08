---
feature_name: nucleo-aiua-tormentosa-motor
created: "2026-09-08"
process: feature
phases:
  - docs-topology
  - forge-genome-em
  - runtime-tool-adapter
  - runtime-actions-process
  - tests-lab-mock
  - docs-execution-dcc-ci-accept
branch_name: feat/nucleo-aiua-tormentosa-motor
persist_ref: docs/features/nucleo-aiua-tormentosa-motor
pbi_ref: docs/todos/pending/PBI_Arranque_Aiua.md
document_id: PBI-NUCLEO-ARRANQUE-AIUA-TORMENTOSA
uuid: "2a4e6c88-1f3b-4d0e-9a2f-7e4b5401a892"
execution_id: "21377cd5-079c-484c-b5d1-c8f8429de402"
---

# Plan — nucleo-aiua-tormentosa-motor

Corte diseño: clarify + objectives + spec + plan. **Commit planificación** antes de mutar genoma/runtime.

## L0 — Diseño (esta parada)

Artefactos bajo `persist_ref`. PBI v1.2.0.

## L1 — Topología documental

`aiua_core.md`, `conscience/index.md`, `README.md`, `cumulo.paths.json`, `CONSTITUTION_CORE.md`. No entity-manager.

## L2 — Forja genoma

`./sddia-run.sh --process entity-manager` (relay IDE si hace falta):

1. tool `thought-graph-access`
2. actions ×3
3. process `aiua-stimulus-processing`

Si EM aborta por revoked: **stop** (DA-2). No Write sobre `SddIA/tools|actions|process`.

## L3 — Runtime tool + adaptador

Crate `thought-graph-access` (si tool-creator no dejó impl). `store_thought` emite ECST. Tests crate + adapter.

## L4 — Runtime acciones + proceso

`try_run_native` + `handlers::aiua_stimulus`. Registro en `mod.rs`. Tests lab-mock.

## L5 — Docs de ejecución + PR

`implementation.md` / `execution.md` / `validacion.md` (`CA-CI: PENDIENTE-CI` hasta verde). Mover PBI a `done/` al archivar. DCC o `gh` solo si DCC sigue revocado **y** el Vértice no da otra vía: intentar DCC primero.

## L6 — CI → accept-pr

`global: APTO` solo con `run_id` verde. Entonces `accept-pr`.

## Fuera

Kalma2. IOTA live. Agente Tormentosa.
