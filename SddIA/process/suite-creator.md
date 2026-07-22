---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:f40de771b5d499861e05771cdc26935447c11133e71234f8d072618e60beb17d
inputs:
- suite_name: Identificador kebab-case de la Suite (`{name}.md` bajo `directories.suites`)
- suite_context: Contexto RBAC (default `chaos-engineering`)
- execution_strategy: 'Enum: `fail_fast` | `run_all`'
- atomic_nodes: Array de nodos `{ process_name, expected_exit_code, timeout_ms? }`
- suite_version: SemVer (default `1.0.0`)
- suites_contract_version: Versión del contrato (default `1.0.0`)
minteo_maximo: null
name: suite-creator
outputs:
- artifact_suite_md: Archivo `{directories.suites}/{suite_name}.md` conforme a `suites-contract.md`
- artifact_suites_index: '`{directories.suites}/index.md` actualizado'
- handoff_entity_uuid: UUID v4 de la Suite forjada
- handoff_hash_signature_new: Sello post-forja
- handoff_hash_signature_old: Sello previo en update; `null` en create
- handoff_version: SemVer resultante
phase_invocations:
- invocations:
  - bind:
      data.result: suite_uuid
    capsule: action:crypto-broker
    on_error: abort
    stdin_json:
      operation: GENERATE_UUID
      target_payload: null
  phase_name: Clasificación Semántica
phases:
- delegates_to:
  - agent:cumulo
  - agent:cerbero
  intent: 'Cúmulo + Cerbero: unicidad bajo directories.suites; atomic_nodes no vacío; process_name existentes.'
  name: Validación de Dominio
- delegates_to:
  - action:crypto-broker
  - agent:cumulo
  intent: Validar execution_strategy; emitir UUID v4 vía crypto-broker.
  name: Clasificación Semántica
- delegates_to:
  - agent:cumulo
  intent: Generar Markdown conforme a suites-contract en directories.suites.
  name: Materialización
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
- delegates_to:
  - agent:cumulo
  intent: Insertar o actualizar fila en suites/index.md.
  name: Indexación
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
porcentaje_de_exito: null
uuid: f3a1b2c3-d4e5-46f7-8901-234567890abc
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# suite-creator

Proceso maestro para instanciar **Suites** (ED Caos) en `SddIA/suites/` y mantener el índice del genoma. Simetría con `tool-creator` / `norm-creator`. Invocable vía `entity-manager` con `entity_class: suite`.
