---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:c7e0a688ed06442289d38341078dc0c127341f13dd8d15dce04181f7f0dc1441
inputs:
- daemon_name: Identificador kebab-case del Centinela (`{name}` del archivo `{name}.md` en `cumulo.directories.daemons`)
- daemon_context: Contexto RBAC Cerbero (debe existir en `execution-contexts.md`)
- daemon_description: Descripción operativa del estímulo físico interceptado y límites de Ceguera Lógica
- daemon_capabilities: Array de strings semánticos (enrutamiento; ej. `telegram-long-poll`, `eda-bus-watch`)
- daemon_execution: 'Bloque obligatorio: `entrypoint`, `runtime`, `heartbeat_interval_seconds` (§4 daemons-contract)'
- daemon_jurisdiction: Declaración de aislamiento; default canónico del contrato
- daemon_version: SemVer de la definición (ej. 1.0.0)
- daemons_contract_version: Versión del contrato daemons a materializar (ej. 1.0.0 según `daemons-contract.md`)
minteo_maximo: null
name: daemon-creator
outputs:
- artifact_daemon_md: Archivo `{paths.directories.daemons}/{daemon_name}.md` con cabecera YAML conforme a `paths.contracts.daemons`
- artifact_daemons_index: '`{paths.directories.daemons}/index.md` actualizado con fila sincronizada a la cabecera YAML'
- handoff_entity_uuid: UUID v4 del Centinela forjado (`child_daemon_uuid`); consumido por `entity-manager`
- handoff_hash_signature_new: Sello `sha256:` + hex canónico post-forja; consumido por `entity-manager`
- handoff_hash_signature_old: Sello previo en update; `null` en create
- handoff_version: SemVer resultante (`daemon_version`)
phase_invocations:
- invocations:
  - bind:
      data.result: child_daemon_uuid
    capsule: action:crypto-broker
    on_error: abort
    stdin_json:
      operation: GENERATE_UUID
      target_payload: null
  - bind:
      data.result: child_daemon_integrity_hex
    capsule: action:crypto-broker
    on_error: abort
    stdin_spec:
      operation: GENERATE_SHA256
      target_payload:
        from_process_inputs:
        - daemon_name
        - daemon_version
        - daemon_context
        - daemon_capabilities
        - daemon_execution
        json_dumps:
          ensure_ascii: false
          separators:
          - ','
          - ':'
          sort_keys: true
        type: canonical_json_utf8
      target_type: STRING
  phase_name: Forja del Markdown
phases:
- delegates_to:
  - agent:cumulo
  - agent:cerbero
  intent: Verificar daemon_context en execution-contexts; unicidad y kebab-case de daemon_name bajo directories.daemons; SemVer; bloque execution completo y heartbeat_interval_seconds ≥ 5.
  name: Validación RBAC y topología
- delegates_to:
  - action:crypto-broker
  intent: Generar uuid v4 y hash_signature de integridad según canon §7 daemons-contract; YAML (contract, context, capabilities, execution, jurisdiction) y cuerpo conforme a contracts.daemons; rutas solo vía cumulo.
  name: Forja del Markdown
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
- delegates_to:
  - agent:cumulo
  intent: Auditar daemons/index.md (columna heartbeat_interval_seconds obligatoria) e insertar fila idéntica a la cabecera del Centinela creado.
  name: Indexación
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
porcentaje_de_exito: null
uuid: c172f130-532f-4714-be4e-fcd80b84a5dc
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# daemon-creator

## Directriz de ejecución obrera

Antes de ejecutar fases de forja, el runtime IDE **debe** anteponer al contexto de Tekton el prefijo definido en `SddIA/norms/external-ai-constraints.md` § Prefijo creator:

> [EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. DO NOT BYPASS EDA BUS. USE SddIA CLI.]

Prohibido delegar forja manual en el agente cuando exista proceso creator aplicable.

Proceso maestro para estandarizar y automatizar la creación de nuevos **Centinelas** (definición `{name}.md` bajo `directories.daemons`) en el Core SddIA.

Invocable directamente o desde **`entity-manager`** (piloto v1, `entity_class: process`). Tras indexación síncrona, el gestor emite `emit-domain-mutation` con `emitter_agent: entity-manager` usando los outputs de handoff declarados en cabecera YAML.

## Fase 1 — Validación RBAC y topología

1. Cargar `execution-contexts.md` desde `paths.directories.norms` y comprobar que `daemon_context` coincide con un identificador válido (`ecosystem-evolution`, `peripheral-sensing`, `filesystem-ops` cuando aplique escucha FS).
2. Verificar que no exista `{paths.directories.daemons}/{daemon_name}.md` y que `daemon_name` cumpla kebab-case.
3. Validar `daemon_version` y `daemons_contract_version` frente a `daemons-contract.md` vigente (`cumulo.contracts.daemons`).
4. Auditar `daemon_execution`: presencia obligatoria de `entrypoint`, `runtime`, `heartbeat_interval_seconds` (entero ≥ 5).
5. Validar `daemon_capabilities` como array no vacío de strings kebab-case o snake-case semántico.

## Fase 2 — Forja del Markdown

1. Ejecutar `phase_invocations`: `child_daemon_uuid` y `child_daemon_integrity_hex` vía `action:crypto-broker`; asignar `hash_signature` como `sha256:` + hex sobre canon §7 (`name`, `version`, `context`, `capabilities`, `execution`).
2. Asignar `contract` como `daemons-contract v{daemons_contract_version}`, `context` igual a `daemon_context` validado, `jurisdiction` según input o default del contrato.
3. Escribir `{paths.directories.daemons}/{daemon_name}.md` con secciones de propósito periférico, bloque `execution`, obligaciones `Daemon_Heartbeat` y límites de Ceguera Lógica.
4. No hardcodear rutas absolutas del host; resolver `directories.daemons`, `execution_capsules.daemons` y `daemons_instance` exclusivamente desde el SSOT de cumulo.

## Fase 3 — Indexación

1. Abrir `{paths.directories.daemons}/index.md` y localizar la tabla de catálogo (columna **heartbeat_interval_seconds** obligatoria).
2. Insertar o actualizar la fila asociada a `{daemon_name}.md` copiando `uuid`, `name`, `version`, `contract`, `context`, `capabilities` y `execution.heartbeat_interval_seconds` desde el YAML fuente sin divergencia.
3. Ejecutar verificación cruzada índice ↔ cabecera antes de cerrar la instancia del proceso.
4. Si el invocante es `entity-manager`, propagar `handoff_entity_uuid` ← `child_daemon_uuid`, `handoff_hash_signature_new` ← `sha256:` + `child_daemon_integrity_hex`, `handoff_hash_signature_old` según operación, `handoff_version` ← `daemon_version`.

## Handoff `entity-manager`

Invocable desde **`entity-manager`** con `entity_class: process` para forja del propio creator o extensión futura con `entity_class: daemon` cuando el piloto amplíe la tabla de delegación. Propagar handoff estándar al cierre de Fase 3; `origin_topology=core`.
