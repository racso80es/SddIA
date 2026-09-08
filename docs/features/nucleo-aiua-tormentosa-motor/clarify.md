---
feature_name: nucleo-aiua-tormentosa-motor
created: "2026-09-08"
process: feature
purpose: Estabilización Filtro A PBI v1.2.0; latido CLI Aiúa
version_clarify: "1.0.0"
execution_id: "21377cd5-079c-484c-b5d1-c8f8429de402"
pbi_ref: docs/todos/pending/PBI_Arranque_Aiua.md
document_id: PBI-NUCLEO-ARRANQUE-AIUA-TORMENTOSA
pbi_uuid: "2a4e6c88-1f3b-4d0e-9a2f-7e4b5401a892"
pbi_version: "1.2.0"
---

# Clarificación — nucleo-aiua-tormentosa-motor

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `21377cd5-079c-484c-b5d1-c8f8429de402`.

## Decisiones

| ID | Laudo |
|----|-------|
| L-NO-AGENTE | Tormentosa no entra en `directories.agents`. Dominio `directories.conscience`. |
| L-NO-KALMA2 | MVP solo CLI. `kalma2-interact` intacto. |
| L-ONE-BURN | `invoke-aiua-core` ensambla; única inferencia = `tool:gemini-http-infer`. |
| L-TOOL-HOST | `thought-graph-access` host nativo (excepción WASI = LanceDB). Envuelve el adaptador; no es Skill. |
| L-ACTION-RUNTIME | `try_invoke_delegates` trata `action:` como tool-capsule. Hace falta handler nativo del proceso + `try_run_native` de las 3 acciones. Genoma `.md` vía `entity-manager`. |
| L-ECST | Emite `lancedb-thought-repo` en `store_thought`. Acción no emisora. |
| L-EMBED | Tool embebe `query_text` con `LocalHashingEmbedder`. Puerto no recibe texto. |
| L-THOUGHT-ID | `node_id` SHA-256 de `ThoughtNode::new`, no UUID v4. |
| L-EM-REVOKED | `entity-manager` ∈ revoked instancia. Forja con relay lab; si Cerbero aborta → no mutar genoma a mano (DA-2). |
| L-CI | `validacion.md` no `global: APTO` hasta `run_id` verde. `accept-pr` solo entonces. |

## Filtro A (no reintroducir)

- `SddIA/README.md` no existe; ontología en `README.md` raíz.
- `gemini-http-infer` no tiene `system_prompt`/`user_prompt`.
- Tokens no son campo de primera clase.
- `thought-persisted` emisores: `thought-triage-service`, `lancedb-thought-repo`.
- Proceso sin `agent:` en `delegates_to`.
