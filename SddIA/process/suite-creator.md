---
uuid: "f3a1b2c3-d4e5-46f7-8901-234567890abc"
name: "suite-creator"
version: "1.0.0"
contract: "process-contract v1.4.0"
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context: "ecosystem-evolution"
hash_signature: sha256:b932cc22e3839cf704638d3e57b9b476cc4375784ed89a0f5303810c74b994ba
inputs:
  - "suite_name": "Identificador kebab-case de la Suite (`{name}.md` bajo `directories.suites`)"
  - "suite_context": "Contexto RBAC (default `chaos-engineering`)"
  - "execution_strategy": "Enum: `fail_fast` | `run_all`"
  - "atomic_nodes": "Array de nodos `{ process_name, expected_exit_code, timeout_ms? }`"
  - "suite_version": "SemVer (default `1.0.0`)"
  - "suites_contract_version": "Versión del contrato (default `1.0.0`)"
outputs:
  - "artifact_suite_md": "Archivo `{directories.suites}/{suite_name}.md` conforme a `suites-contract.md`"
  - "artifact_suites_index": "`{directories.suites}/index.md` actualizado"
  - "handoff_entity_uuid": "UUID v4 de la Suite forjada"
  - "handoff_hash_signature_new": "Sello post-forja"
  - "handoff_hash_signature_old": "Sello previo en update; `null` en create"
  - "handoff_version": "SemVer resultante"
phases:
  - name: "Validación de Dominio"
    intent: "Cúmulo + Cerbero: unicidad bajo `directories.suites`; `atomic_nodes` no vacío; process_name existentes."
    delegates_to:
      - "agent:cumulo"
      - "agent:cerbero"
  - name: "Clasificación Semántica"
    intent: "Validar execution_strategy; emitir UUID v4 vía crypto-broker."
    delegates_to:
      - "action:crypto-broker"
      - "agent:cumulo"
  - name: "Materialización"
    intent: "Generar Markdown conforme a suites-contract en directories.suites."
    delegates_to:
      - "skill:filesystem-manager"
      - "agent:cumulo"
  - name: "Indexación"
    intent: "Insertar o actualizar fila en suites/index.md."
    delegates_to:
      - "agent:cumulo"
      - "skill:filesystem-manager"
phase_invocations:
  - phase_name: "Clasificación Semántica"
    invocations:
      - capsule: "action:crypto-broker"
        stdin_json:
          operation: "GENERATE_UUID"
          target_payload: null
        bind:
          "data.result": "suite_uuid"
        on_error: abort
minteo_maximo: null
porcentaje_de_exito: null
---

# suite-creator

Proceso maestro para instanciar **Suites** (ED Caos) en `SddIA/suites/` y mantener el índice del genoma. Simetría con `tool-creator` / `norm-creator`. Invocable vía `entity-manager` con `entity_class: suite`.
