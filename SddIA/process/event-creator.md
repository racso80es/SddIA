---
uuid: "b28194d9-62a8-4cbc-9cbd-237e51e44333"
name: "event-creator"
version: "1.1.0"
contract: "process-contract v1.3.0"
context: "ecosystem-evolution"
hash_signature: sha256:ff3baf158327892036d67b5166963ce5716c5271cb7f3b4ee854248402268999
inputs:
  - "event_name": "Identificador kebab-case de la Clase (`{name}` del archivo `{name}.md` en `cumulo.directories.events`)"
  - "event_type": "Identificador ECST PascalCase_Snake (p. ej. PullRequest_Merged); único en catálogo"
  - "event_context": "Contexto RBAC Cerbero válido según `execution-contexts.md`"
  - "event_description": "Descripción operativa y propósito de la Clase de Evento"
  - "payload_required": "Array de nombres de campos obligatorios en `payload` de instancia ECST"
  - "payload_optional": "Array de campos opcionales en `payload`"
  - "payload_forbidden": "Array de campos prohibidos en `payload` (p. ej. hash_signature en eventos Git)"
  - "emitter_agents": "Array de identificadores de acciones/procesos autorizados a emitir instancias"
  - "event_version": "SemVer de la Clase (ej. 1.0.0)"
  - "events_contract_version": "Versión del contrato events a materializar (ej. 1.1.0 según `events-contract.md`)"
  - "event_family": "Familia Trinidad: telemetry | orchestration | domain. Opcional en invocación; si ausente o vacío → default obligatorio `domain` (retrocompat). Ver Kaizen retirar default."
outputs:
  - "artifact_event_md": "Archivo `{paths.directories.events}/{effective_event_family}/{event_name}.md`"
  - "artifact_events_index": "`{paths.directories.events}/{effective_event_family}/index.md` actualizado con fila sincronizada a la cabecera YAML"
  - "handoff_entity_uuid": "UUID v4 de la Clase forjada; consumido por `entity-manager`"
  - "handoff_hash_signature_new": "Sello `sha256:` + hex canónico post-forja; consumido por `entity-manager`"
  - "handoff_hash_signature_old": "Sello previo en update; `null` en create"
  - "handoff_version": "SemVer resultante (`event_version`)"
phases:
  - name: "Normalización de familia"
    intent: "Calcular effective_event_family = event_family trim no vacío, si no domain. Registrar en contexto de fase."
  - name: "Validación de Arquitectura"
    intent: "Verificar effective_event_family en enum Trinidad; event_context en execution-contexts; unicidad de event_type frente al Códice de la familia; kebab-case de event_name; coherencia payload con events-contract v1.1.0; para telemetry rechazar emisores no-CLI."
    delegates_to:
      - "agent:cumulo"
      - "agent:cerbero"
  - name: "Forja del Artefacto"
    intent: "Generar uuid v4 y hash_signature de integridad; YAML (contract, context, event_type, capabilities) y cuerpo con Payload ECST REQUIRED/OPTIONAL/FORBIDDEN, emisores y suscripciones; rutas solo vía cumulo."
    delegates_to:
      - "action:crypto-broker"
      - "skill:filesystem-manager"
  - name: "Gobernanza de Índice"
    intent: "Auditar events/{effective_event_family}/index.md (columna Capabilities obligatoria) e insertar fila idéntica a la cabecera de la Clase creada."
    delegates_to:
      - "agent:cumulo"
      - "skill:filesystem-manager"
phase_invocations:
  - phase_name: "Forja del Artefacto"
    invocations:
      - capsule: "action:crypto-broker"
        stdin_json:
          operation: "GENERATE_UUID"
          target_payload: null
        bind:
          "data.result": "child_event_uuid"
        on_error: abort
      - capsule: "action:crypto-broker"
        stdin_spec:
          operation: "GENERATE_SHA256"
          target_type: "STRING"
          target_payload:
            type: "canonical_json_utf8"
            from_process_inputs:
              - "event_name"
              - "event_type"
              - "event_version"
              - "event_context"
              - "payload_required"
              - "payload_optional"
              - "payload_forbidden"
            json_dumps:
              sort_keys: true
              separators: [",", ":"]
              ensure_ascii: false
        bind:
          "data.result": "child_event_integrity_hex"
        on_error: abort
minteo_maximo: null
porcentaje_de_exito: null
---

# event-creator

## Directriz de ejecución obrera

Antes de ejecutar fases de forja, el runtime IDE **debe** anteponer al contexto de Tekton el prefijo definido en `SddIA/norms/external-ai-constraints.md` § Prefijo creator:

> [EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. DO NOT BYPASS EDA BUS. USE SddIA CLI.]

Prohibido delegar forja manual en el agente cuando exista proceso creator aplicable.

Proceso maestro para instanciar nuevas **Clases de Evento** (genoma ECST) en `SddIA/events/` y mantener el índice del directorio `events`.

Invocable directamente o desde **`entity-manager`** (cuando `entity_class: event` esté en piloto). Tras indexación síncrona, el gestor emite `emit-domain-mutation` con `emitter_agent: entity-manager` usando los outputs de handoff declarados en cabecera YAML.

## Fase 0 — Normalización de familia

1. Leer `event_family` de `process_inputs`.
2. Si está ausente, es solo espacios o cadena vacía → asignar `effective_event_family = "domain"`.
3. En caso contrario → `effective_event_family = event_family.strip()` (minúsculas esperadas en contrato).

## Fase 1 — Validación de Arquitectura

1. Validar `effective_event_family` ∈ `{ telemetry, orchestration, domain }`.
2. Cargar `execution-contexts.md` y comprobar `event_context` válido.
3. Verificar que no exista `{paths.directories.events}/{effective_event_family}/{event_name}.md` en create; `event_name` kebab-case.
4. Comprobar unicidad de `event_type` frente a `events/{effective_event_family}/index.md` y `events-contract.md` v1.1.0.
5. Si `effective_event_family == telemetry`: `emitter_agents` solo procesos/cápsulas CLI indexados (sin agentes ED obreros).
6. Auditar que `payload_required`, `payload_optional` y `payload_forbidden` no se solapen; respetar reglas forenses del contrato.
7. Validar `event_version` y `events_contract_version` frente a `events-contract.md` vigente.

## Fase 2 — Forja del Artefacto

1. Ejecutar `phase_invocations`: `child_event_uuid` y `child_event_integrity_hex` vía `action:crypto-broker`; asignar `hash_signature` como `sha256:` + hex sobre el sujeto canónico definido en `phase_invocations`.
2. Asignar `contract` como `events-contract v{events_contract_version}`, `event_family` igual a `effective_event_family`, `context` igual a `event_context` y `capabilities` obligatorio.
3. Escribir `{paths.directories.events}/{effective_event_family}/{event_name}.md` con secciones: **Payload ECST**, **Emisores autorizados**, **Suscripciones**.
4. No hardcodear rutas absolutas; resolver `directories.events` y `contracts.events` exclusivamente desde Cúmulo.

## Fase 3 — Gobernanza de Índice

1. Abrir `{paths.directories.events}/{effective_event_family}/index.md` y localizar la tabla de catálogo (columna **Capabilities** obligatoria).
2. Insertar o actualizar la fila asociada a `{event_name}.md` copiando `uuid`, `name`, `event_type`, `version`, `contract`, `context` y `capabilities` desde el YAML fuente.
3. Excluir `events-contract.md` e `index.md` del catálogo de Clases.
4. Si el invocante es `entity-manager`, propagar handoff según outputs de cabecera.
