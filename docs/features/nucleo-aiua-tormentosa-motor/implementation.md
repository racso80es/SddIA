---
feature_name: nucleo-aiua-tormentosa-motor
created: "2026-09-08"
process: feature
items:
  - L1-topology
  - L2-genome-em
  - L3-tool-adapter
  - L4-handlers
  - L5-tests
branch_name: feat/nucleo-aiua-tormentosa-motor
persist_ref: docs/features/nucleo-aiua-tormentosa-motor
execution_id: "21377cd5-079c-484c-b5d1-c8f8429de402"
document_id: PBI-NUCLEO-ARRANQUE-AIUA-TORMENTOSA
---

# Implementation — nucleo-aiua-tormentosa-motor

## Touchpoints

| Path | Cambio |
|------|--------|
| `SddIA/conscience/aiua_core.md` | uuid, v1.1.0, Sección 5 LanceDB MVP + proyección DLT |
| `SddIA/conscience/index.md` | Índice Cúmulo `entity_family: conscience` |
| `README.md` | Fila Ontología Aiúa; latido `aiua-stimulus-processing` |
| `SddIA/core/cumulo.paths.json` | `directories.conscience`; version 1.10.0 |
| `SddIA/CONSTITUTION_CORE.md` | Interpretación ética → `paths.directories.conscience` |
| `SddIA/tools/thought-graph-access.md` | Forja EM; host nativo search/store |
| `SddIA/tools/thought-graph-access/` | Crate: `LocalHashingEmbedder` + `LanceDbThoughtRepo::open_with_bus` |
| `SddIA/actions/{retrieve-active-context,invoke-aiua-core,persist-thought-record}.md` | Forja EM |
| `SddIA/process/aiua-stimulus-processing.md` | Forja EM; 4 fases; sin `agent:` |
| `SddIA/infrastructure/adapters/lancedb_thought_repo/` | `open_with_bus`; emite `Thought_Persisted` |
| `SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs` | Handler nativo + `try_action` |
| `SddIA/engine/execute-process/src/engine/{mod.rs,actions.rs,handlers/mod.rs}` | Registro proceso y acciones |

## Runtime

1. `retrieve-active-context` → `thought-graph-access` `search`. `memories: []` no es error.
2. `invoke-aiua-core` lee `aiua_core.md` vía `directories.conscience`. Sin HTTP.
3. Única combustión: `gemini-http-infer` (`request.prompt` + `request.model`).
4. `persist-thought-record` → `store`. El **adaptador** emite ECST (`emitter_agent: lancedb-thought-repo`). `thought_id` = `node_id` SHA-256.

## Fuera

Kalma2. IOTA live. Agente Tormentosa.
