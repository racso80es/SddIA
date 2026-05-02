# Contrato: Acciones del ciclo (actions)

**Alcance:** paths.actionsPath (SddIA/actions/). Toda acción del ciclo debe cumplir este contrato.

## Definición por acción

Cada acción tiene una **carpeta** en paths.actionsPath con identificador `<action-id>` (kebab-case). Dentro de la carpeta:

| Artefacto | Propósito |
|-----------|-----------|
| **spec.md** | Especificación legible: propósito, entradas, salidas, flujo de ejecución. es-ES. |
| **spec.json** | Metadatos machine-readable: action_id, name, purpose, inputs, outputs, flow_steps, contract_ref, related_processes. |

## Restricciones

- action_id en kebab-case (spec, clarify, planning, implementation, execution, validate, finalize-process, sddia-difusion).
- Rutas solo vía Cúmulo.

## Alcance de ejecución (norma innegociable)

Una **acción** describe propósito, entradas, salidas y **la política u orden de invocación** de **skills** y **herramientas** registradas en el ecosistema SddIA (Cúmulo: `paths.skillCapsules`, `paths.toolCapsules`, definiciones en `paths.skillsDefinitionPath` / `paths.toolsDefinitionPath`).

- **Prohibido** como requisito normativo de la acción: ejecutar comandos del sistema operativo, invocar scripts (`.ps1`, `.bat`, `.cmd`, shell) o binarios **no** catalogados como skill/tool del proyecto con el fin de cumplir el flujo documentado en `spec.md`.
- **Permitido:** nombrar `skill_id` / `tool_id` y referenciar cápsulas vía Cúmulo; el ejecutor invoca esas cápsulas según `SddIA/norms/commands-via-skills-or-tools.md` (p. ej. variable `GESFER_CAPSULE_REQUEST`, `scripts/skills/run-capsule-from-tekton-request.ps1`).

Los ejemplos de línea de comando en documentación orientada a humanos deben estar claramente acotados como **ilustrativos**, no como paso obligatorio de la acción.

## Consumidores

paths.processPath, SddIA/agents, SddIA/norms (interaction-triggers), .cursor/rules.
