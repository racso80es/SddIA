---
uuid: "1c6af49c-3091-4648-aa54-bbf6bcb90f82"
name: "patterns-in-planning-implementation-execution"
version: "1.0.0"
nature: "tactical-norm"
author: "norm-creator"
scope: "agnostic"
category: "workflow"
dependencies: []
---

## Directriz Core

Los patrones de diseño (`directories.patterns`, resueltos vía Cúmulo) se aplican en tres fases del ciclo de tarea que tocan diseño y código: **planning**, **implementation** y **execution**. El contrato estructural de patrones es `SddIA/patterns/patterns-contract.md`; cada patrón vive en `directories.patterns/<uuid>/` con `spec.md` (frontmatter YAML conforme al contrato) y metadatos de apoyo según el contrato vigente.

### Planning (`plan.md`)

- En análisis de requisitos, identificar decisiones arquitectónicas que correspondan a patrones existentes (p. ej. persistencia → Repository; legacy → ACL; comandos/eventos → CQRS o Domain Events).
- En el plan, incluir referencias cuando una tarea aplique o respete un patrón: sección **Patrones aplicados** con UUID y título, o etiquetas `[PATTERN-<uuid>]` por tarea/fase.
- El agente o proceso que genera el plan debe resolver el listado de patrones consultando Cúmulo (`directories.patterns`); en planificación el **Arquitecto** selecciona patrones aplicables; **Tekton** los usa en implementación y ejecución según `interested_agents` del patrón.
- Las referencias del plan deben apuntar exclusivamente a patrones existentes con `spec.md` válido; no se inventan patrones en el plan.

### Implementation (`implementation.md`)

- Al derivar touchpoints del plan, si una tarea referencia un patrón (UUID o `[PATTERN-<uuid>]`), el ítem de implementación incluye el campo **Patrón** / `pattern_id` (UUID).
- Los patrones pueden imponer restricciones de ubicación (p. ej. Hexagonal: puertos en dominio, adaptadores en infra); validar coherencia de rutas propuestas contra `spec.md` del patrón.
- Trazabilidad PLAN → IMPL: ítems que materializan tareas con patrón llevan el `pattern_id` correspondiente.

### Execution (`execution.md`)

- Si un ítem tiene `pattern_id`, el ejecutor (Tekton) consulta `directories.patterns/<uuid>/spec.md` antes de aplicar la propuesta.
- Al aplicar Crear/Modificar/Eliminar, respetar restricciones del patrón etiquetado (estructura, capas, convenciones del spec).
- Registrar `pattern_id` en cada entrada de ejecución para trazabilidad y validación posterior (validate / QA Judge).
- El patrón no altera orden ni dependencias del IMPL; gobierna la *forma* de implementar.

Los principios (`directories.principles`) guían *cómo* pensar el diseño; los patrones guían *qué* estructuras y convenciones aplicar en plan, implementación y código.

## Restricciones Duras (Aduana de Fricción)

- Prohibido referenciar en plan, implementation o execution un `pattern_id` que no exista físicamente bajo `directories.patterns/<uuid>/` con `spec.md` conforme a `patterns-contract.md`.
- Prohibido inventar patrones o UUID ficticios en `plan.md`; toda referencia debe ser resoluble vía Cúmulo.
- Prohibido omitir `pattern_id` en ítems de `implementation.md` cuando la tarea del plan incluye etiqueta `[PATTERN-<uuid>]` o entrada en Patrones aplicados.
- Prohibido ejecutar un ítem con `pattern_id` sin haber consultado el `spec.md` del patrón antes de modificar código o estructura de carpetas.
- Prohibido violar restricciones de ubicación o capa declaradas en el spec del patrón cuando el ítem lleva `pattern_id`.
- Prohibido usar `implementation.json` o `execution.json` como artefactos de tarea; la fuente de verdad es `implementation.md` y `execution.md` según `features-documentation-pattern`.
- Prohibido invocar skills Git tácticas legacy (`git-workspace-recon`, `invoke-command`, suite S+ histórica); operaciones Git y PR siguen `git-operations.md`, `git-manager`, `shell-executor` y `pull-request-orchestration.md`.
