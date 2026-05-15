---
uuid: "4c448c82-de41-460f-b24f-82a84fa5ed69"
name: "features-documentation-pattern"
version: "1.0.0"
nature: "tactical-norm"
author: "norm-creator"
scope: "agnostic"
category: "workflow"
dependencies: []
---

## Directriz Core

La documentación de tareas (features y fixes) sigue el mismo patrón que skills, tools, actions y process del ecosistema SddIA: **un único archivo `.md` por fase de acción**, con **frontmatter YAML** (metadatos machine-readable) y **cuerpo Markdown** (contenido legible). Los datos estructurados residen en el frontmatter; no se mantienen artefactos `.json` paralelos.

Cada fase del ciclo de tarea produce exclusivamente su `.md` canónico bajo la ruta resuelta por Cúmulo (`directories.documentation` / paths de feature o fix según manifiesto):

| Acción | Archivo | Frontmatter mínimo | Cuerpo |
|--------|---------|-------------------|--------|
| objectives | objectives.md | feature_name, created, process | Objetivo, alcance, ley aplicada |
| spec | spec.md | feature_name, created, base, scope (opc.) | Especificación técnica |
| clarify | clarify.md | feature_name, created, purpose | Clarificaciones y decisiones |
| planning | plan.md | feature_name, created, phases | Plan de implementación |
| implementation | implementation.md | feature_name, created, items | Touchpoints y propuestas |
| execution | execution.md | feature_name, created, items_applied | Registro de ejecución |
| validate | validacion.md | feature_name, branch, global, checks, git_changes | Informe de validación |
| finalize-process | finalize-process.md (opc.) | feature_name, pr_url, timestamp | Resumen de cierre |

La validación opcional `sddia_frontmatter_valid` aplica a los `.md` de tarea cuando el diff los toque. Las tareas existentes con `.json` deben migrarse consolidando el contenido en el frontmatter del `.md` correspondiente y eliminando el `.json`. Las nuevas documentaciones cumplen este patrón desde el inicio.

## Restricciones Duras (Aduana de Fricción)

- Prohibido generar o mantener `spec.json`, `clarify.json`, `plan.json`, `implementation.json`, `execution.json`, `validacion.json` o `finalize-process.json` como fuente de verdad paralela al `.md` de la misma acción.
- Prohibido omitir el bloque frontmatter YAML (`---` … `---`) en cualquier artefacto de tarea.
- Prohibido producir más de un archivo por acción de fase (un `.md` por acción, sin duplicados ni variantes JSON).
- Prohibido almacenar en el cuerpo Markdown datos que deban ser machine-readable si pueden declararse en frontmatter según la tabla canónica.
- Prohibido crear nuevas tareas que violen este patrón; la migración de legado es la única excepción temporal y debe cerrarse eliminando el `.json`.
