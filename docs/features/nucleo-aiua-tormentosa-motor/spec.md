---
feature_name: nucleo-aiua-tormentosa-motor
created: "2026-09-08"
process: feature
base: main
scope: core
branch_name: feat/nucleo-aiua-tormentosa-motor
persist_ref: docs/features/nucleo-aiua-tormentosa-motor
execution_id: "21377cd5-079c-484c-b5d1-c8f8429de402"
document_id: PBI-NUCLEO-ARRANQUE-AIUA-TORMENTOSA
---

# Spec — nucleo-aiua-tormentosa-motor

## 1. Topología documental (no DA-2)

| Artefacto | Mutación |
|-----------|----------|
| `SddIA/conscience/aiua_core.md` | `uuid` v4; `version` 1.1.0; Sección 5 (LanceDB MVP + proyección DLT). Agnóstico. |
| `SddIA/conscience/index.md` | Índice Cúmulo `entity_family: conscience`, `directories_key: conscience`. |
| `README.md` | Fila Ontología: Aiúa / `paths.directories.conscience` / no agente. |
| `SddIA/core/cumulo.paths.json` | `"conscience": "SddIA/conscience"`; bump `version`. |
| `SddIA/CONSTITUTION_CORE.md` | Cláusula: interpretación ética → `aiua_core.md` bajo `directories.conscience`. |

## 2. Genoma (DA-2 / entity-manager)

Orden de forja (dependencias):

1. `entity_class: tool` `thought-graph-access` `scope: core` `tool_context: ecosystem-evolution`. Host nativo. Operaciones `search` \| `store`.
2. `entity_class: action` ×3 (`ecosystem-evolution`):
   - `retrieve-active-context` → `tool:thought-graph-access` `operation=search`
   - `invoke-aiua-core` → lee genoma vía `directories.conscience`; no HTTP
   - `persist-thought-record` → tool `operation=store`; no ECST
3. `entity_class: process` `aiua-stimulus-processing` jurisdicción `core`, `process-contract v1.4.0`, `workspace_template` canónico.

`process_context` / `action_context`: valor existente en `execution-contexts.md` (no inventar contexto). Preferir `ecosystem-evolution` si está listado.

## 3. Runtime (engine, no genoma)

`try_invoke_delegates` invoca nombres como **tool-capsule**. Un proceso `delegates_to: action:…` no ejecuta `invoke_action`. Spec de runtime:

- Handler `handlers::aiua_stimulus::run` registrado en `engine/mod.rs` (paridad `kalma2-interact`).
- Acciones: `actions::try_run_native` para las tres (invocan `invoke_tool_capsule_json("thought-graph-access", …)` o FS genoma).
- Tool crate `SddIA/tools/thought-graph-access/` (workspace `tools/*`):
  - stdin capsule-json-io 2.0
  - `request.operation`: `search` \| `store`
  - URI `{paths.vectorStore}/lancedb/` vía Cúmulo
  - `LocalHashingEmbedder` + `LanceDbThoughtRepo`
- Adaptador: tras `upsert`/`store_thought` OK, escribir ECST `Thought_Persisted` en `eda_bus.pending` con `node_id`, `parent_id` ("" si raíz), `status`, `store_path`. Sin `biological_vertex_output`. `emitter_agent`: `lancedb-thought-repo`.

## 4. I/O proceso

Inputs: `prompt` (req), `context_query` (opt=prompt), `model` (opt → `SDDIA_GEMINI_MODEL`).

Outputs: `thought_id` (`node_id`), `response` (`result.text`), `telemetry.{duration_ms,model,tokens?}`.

Lab: `SDDIA_LAB_MOCK_OUTBOUND` / `SDDIA_LAB_MOCK_GEMINI_URL`. Sin `GEMINI_API_KEY`.

## 5. Tests

- Tool: search vacío → `[]`; store → `node_id` no vacío.
- Adapter: `store_thought` deja JSON `Thought_Persisted` en pending.
- Handler proceso (mock Gemini): `memories` vacías no fallan; `durationMs` presente.
- Grep: proceso/acciones no referencian Kalma2 UI.
