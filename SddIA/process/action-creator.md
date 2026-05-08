---
uuid: d0e1f2a3-b4c5-46d7-e8f9-0a1b2c3d4e5f
name: action-creator
version: 1.0.0
contract: process-contract v1.3.0
context: ecosystem-evolution
hash_signature: sha256:b73c4b13849716087b4033ab1b49d4e43c27f79c4fbc51d0894a686217ca3c7f
inputs:
- action_name: Identificador kebab-case de la acción (`{name}` del archivo `{name}.md`
    en `cumulo.directories.actions`)
- action_context: Contexto RBAC Cerbero válido según `execution-contexts.md`
- action_inputs: Esquema JSON de datos requeridos para iniciar la orquestación
- action_outputs: Esquema JSON del resultado al cierre (éxito/error)
- orchestration_logic: Bucle de ejecución que conecta invocaciones a skills/tools
    resueltas vía cumulo (sin comandos de terminal crudos)
outputs:
- artifact_action_md: Archivo `{paths.directories.actions}/{action_name}.md` conforme
    a `paths.contracts.actions`
- artifact_actions_index: '`{paths.directories.actions}/index.md` creado o actualizado
    con fila sincronizada a la cabecera YAML de la acción'
phases:
- name: Validación de Arquitectura
  intent: Auditar action_context en execution-contexts; orquestación sin shell; delegación
    exclusiva skills/tools vía cumulo; unicidad kebab-case; inputs/outputs; FILTRO
    ANTI-FASES (Cumulo bloquea action_name o propósito que intersequen con la lista
    exhaustiva del SSOT en actions-contract.md § 2bis). Cerbero queda exento de esta
    gobernanza semántica.
  delegates_to:
  - agent:cumulo
  - agent:cerbero
- name: Forja del Artefacto
  intent: Generar uuid v4 y cabecera YAML (contract, context, capabilities, inputs,
    outputs, hash_signature si aplica) según actions-contract; cuerpo con orquestación
    bajo directories.actions.
  delegates_to:
  - action:crypto-broker
  - skill:filesystem-manager
- name: Gobernanza
  intent: Crear o actualizar actions/index.md con columna Capabilities y fila idéntica
    a la cabecera de la acción creada.
  delegates_to:
  - agent:cumulo
  - skill:filesystem-manager
phase_invocations:
- phase_name: Forja del Artefacto
  invocations:
  - capsule: action:crypto-broker
    stdin_json:
      operation: GENERATE_UUID
      target_payload: null
    bind:
      data.result: child_action_uuid
    on_error: abort
minteo_maximo: null
porcentaje_de_exito: null
---

# action-creator

Proceso maestro para instanciar nuevas acciones (orquestaciones lógicas) en el Core SddIA y mantener el índice del directorio `actions`.

## Fase 1 — Validación de Arquitectura

1. Resolver `SddIA/norms/execution-contexts.md` desde `cumulo.paths.json` (`directories.norms`) y validar `action_context` contra la matriz S+.
2. Auditar `orchestration_logic` para prohibir invocaciones directas a terminal, binarios arbitrarios del host o rutas hardcodeadas a dependencias; exigir delegación explícita a skills/tools con resolución SSOT.
3. Comprobar unicidad y kebab-case de `action_name` bajo `cumulo.directories.actions`.
4. Validar exhaustividad de `action_inputs` y `action_outputs` frente al contrato de acciones.
5. **Falla Controlada (candado semántico vía Cumulo):** El proceso abortará inmediatamente si el `action_name` o su propósito intersecan con la lista exhaustiva de términos prohibidos del SSOT (`actions-contract.md § 2bis`). La gobernanza semántica corresponde a **Cumulo** (soberano del catálogo); **Cerbero** queda exento por ser su naturaleza pura intercepción RBAC contra `execution-contexts.md`. La intención debe redirigirse a `process-creator`.

## Fase 2 — Forja del Artefacto

1. Ejecutar `phase_invocations`: obtener `child_action_uuid` vía `action:crypto-broker` (`GENERATE_UUID`). Si el contrato exige `hash_signature`, usar invocaciones adicionales documentadas en el contrato de acciones (mismo broker + `GENERATE_SHA256`).
2. Asignar `contract` alineado a `actions-contract.md` (`cumulo.contracts.actions`) y `capabilities` obligatorio según contrato.
3. Redactar `{paths.directories.actions}/{action_name}.md` con secciones de propósito, grafo o secuencia de orquestación y límites termodinámicos (`minteo_maximo`, `porcentaje_de_exito` si aplican).
4. Documentar explícitamente el uso de skills/tools únicamente como cápsulas referenciadas desde topología cumulo.

## Fase 3 — Gobernanza

1. Abrir o crear `{paths.directories.actions}/index.md` con tabla de catálogo exigida por Gobernanza de Índices del directorio (columna **Capabilities** obligatoria).
2. Insertar o actualizar la fila asociada a `{action_name}.md` copiando metadatos (incl. `capabilities`) desde el YAML fuente sin divergencia.
3. Cerrar con verificación cruzada índice ↔ cabecera.
