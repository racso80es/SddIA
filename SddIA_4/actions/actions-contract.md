# Contrato: Acciones del ciclo (actions)

**Alcance:** paths.actionsPath (SddIA/actions/). Toda acción del ciclo debe cumplir este contrato.

## Definición por acción

Cada acción tiene una **carpeta** en paths.actionsPath con identificador `<action-id>` (kebab-case). Dentro de la carpeta:

| Artefacto | Propósito |
|-----------|-----------|
| **spec.md** | Especificación legible: propósito, entradas, salidas, flujo de ejecución. es-ES. |
| **spec.json** | Metadatos machine-readable: action_id, name, purpose, inputs, outputs, flow_steps, contract_ref, related_processes. |

## Orquestación (Ley COMANDOS) — norma innegociable

Las acciones **no** ejecutan código directo del sistema operativo ni scripts (`.ps1`, `.bat`, `.cmd`, `sh`, etc.) como **mecanismo normativo** de cumplimiento. No prescriben comandos literales (`git`, `cargo`, `npm`, …) en el flujo canónico.

**Jurisdicción de la acción:** describir **qué** debe lograrse y **qué** skills o tools **registradas en Cúmulo** (`paths.skillCapsules`, `paths.toolCapsules`, definiciones en `paths.skillsDefinitionPath`, `paths.toolsDefinitionPath`) deben invocarse, en qué orden y bajo qué precondiciones. La ejecución material pasa siempre por esas cápsulas (p. ej. envelope JSON v2 y `SddIA/norms/capsule-json-io.md`, o el flujo Tekton con `.tekton_request.json` y `scripts/skills/run-capsule-from-tekton-request.ps1`).

Los scripts bajo `scripts/` pueden existir como ayuda no canónica o legado; **no** forman parte del contrato de la acción salvo que una skill/tool explícita los exponga como implementación registrada.

## Restricciones

- action_id en kebab-case (spec, clarify, planning, implementation, execution, validate, finalize-process, sddia-difusion).
- Rutas solo vía Cúmulo.

## Consumidores

paths.processPath, SddIA/agents, SddIA/norms (interaction-triggers), .cursor/rules.
