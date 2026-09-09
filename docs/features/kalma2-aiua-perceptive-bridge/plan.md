---
feature_name: kalma2-aiua-perceptive-bridge
created: "2026-09-09"
process: feature
phases:
  - docs-planning
  - bridge-orchestrator-param
  - bridge-aiua-endpoint
  - wui-pulse
  - tests-check
  - docs-execution-dcc-ci-accept
branch_name: feat/kalma2-aiua-perceptive-bridge
persist_ref: docs/features/kalma2-aiua-perceptive-bridge
pbi_ref: docs/todos/pending/[NÚCLEO] Puente Perceptivo: Interacción Biológica con Tormentosa desde Kalma2 WUI.md
document_id: PBI-NUCLEO-PUENTE-PERCEPTIVO-KALMA2
uuid: "d7192a54-7389-4b68-b3d4-b91c0e35921a"
execution_id: "f075b5ea-aac3-47cb-b5ba-a18c5431a7e5"
---

# Plan — kalma2-aiua-perceptive-bridge

Corte diseño: clarify + objectives + spec + plan. **Commit planificación** antes de mutar puente/WUI.

## L0 — Diseño (esta parada)

Artefactos bajo `persist_ref`. PBI v1.2.0. Init `execution_id` `f075b5ea-aac3-47cb-b5ba-a18c5431a7e5`.

## L1 — Spawn parametrizado + timeout + fractura

`run_orchestrator_inputs`: argumento `process`. `client_timeout_secs`: max cliente/Gemini. `emit_system_fracture`: `attempted_action` parámetro. Callers Mayeuta SSE conservan `sse_chat_stream`.

## L2 — Endpoint Aiúa

`handle_aiua_interact` + ruta en `dispatch`. Flatten envelope. Fractura solo ELF/spawn.

## L3 — WUI

`#aiua-pulse`, `enviarAiuaStimulus`, `setBusy` extendido. PTC objeto. Cero hijack pulso.

## L4 — Tests

`cargo check` + tests crate `kalma2-bridge` (ruta, process name, flatten, timeout, ceguera genoma via grep en test o script).

## L5 — Docs de ejecución + PR

`implementation.md` / `execution.md` / `validacion.md` (`CA-CI: PENDIENTE-CI` hasta verde). Evolution si toca `SddIA/`. DCC. PBI a `done/` al archivar con CI verde.

## L6 — CI → accept-pr

`global: APTO` solo con `run_id` verde. Entonces `accept-pr`.

## Fuera

Genoma latido. README fósil. SSE Aiúa. Anatomía Motora. Poda.
