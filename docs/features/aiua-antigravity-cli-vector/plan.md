---
feature_name: aiua-antigravity-cli-vector
created: "2026-09-09"
process: feature
phases:
  - docs-design
  - forge-process-em
  - handler-antigravity
  - bridge-sanitize-timeout
  - starter-kit-env
  - tests-lab-mock
  - docs-execution-dcc-ci-accept
branch_name: feat/aiua-antigravity-cli-vector
persist_ref: docs/features/aiua-antigravity-cli-vector
pbi_ref: docs/todos/pending/[NÚCLEO] Aiúa — combustión Tormentosa vía antigravity-cli.md
document_id: PBI-NUCLEO-AIUA-ANTIGRAVITY-CLI-VECTOR
uuid: "d15e82ed-ccad-4e5a-80ea-c6e808afa086"
execution_id: "e35155cd-fdfa-4612-92a4-121e12b4c3f9"
---

# Plan — aiua-antigravity-cli-vector

Corte diseño: clarify + objectives + spec + plan. **Commit planificación** antes de mutar genoma/handler/bridge.

## L0 — Diseño (esta parada)

Artefactos bajo `persist_ref`. PBI v1.2.0.

## L1 — Forja proceso (DA-2)

`./sddia-run.sh --process entity-manager` (relay IDE) ×2 `update` `aiua-stimulus-processing`:

1. `process_phases` + `process_inputs` + `process_version: 1.1.0`.
2. `markdown_body_replacements`.

Si EM aborta por revoked: **stop** (DA-2). No Write sobre `SddIA/process/`.

## L2 — Handler

Tekton sobre `aiua_stimulus.rs`: `infer_antigravity_cli`, resolución effort/timeout, usage, tests lab-mock.

## L3 — Bridge

`sanitize_bridge_message` + `resolve_client_timeout_secs` 3-techos. Tests herméticos. Cero `app.js`.

## L4 — Starter-kit

Claves AGY comentadas en los dos `.env.example` del kit.

## L5 — Docs + evolution + PR

`implementation.md` / `execution.md` / `validacion.md` (`CA-CI: PENDIENTE-CI` hasta verde). `sddia-qa evolution-register`. DCC. PBI a `done/` en la rama.

## L6 — CI → accept-pr

`global: APTO` solo con `run_id` verde. Entonces `accept-pr`.

## Fuera

Mayeuta. Crates HTTP/CLI. `--print`. Kitchen. Anatomía motora. Skip-permissions.
