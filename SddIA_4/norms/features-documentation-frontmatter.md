# Norma: Documentación de tarea (paths.featurePath, paths.fixPath) — Patrón Frontmatter

**Fuente:** SddIA/norms. Aplicable a toda documentación generada en la carpeta de la tarea (features, fixes).

## Definición

La **documentación de tarea** es el conjunto de artefactos generados por las acciones del ciclo (spec, clarify, planning, implementation, execution, validate, finalize-process) en la carpeta de persistencia de la tarea: **paths.featurePath/<nombre_feature>/** o **paths.fixPath/<nombre_fix>/** (Cúmulo).

## Patrón obligatorio: un solo fichero .md por acción con YAML Frontmatter

Toda nueva documentación de tarea debe seguir el mismo patrón que skills, tools y demás entidades del ecosistema SddIA:

- **Un único archivo `.md` por acción** con metadatos estructurados en **YAML Frontmatter** en la parte superior.
- **No se generan ficheros `.json` separados.** Los metadatos que antes residían en `.json` se integran en el bloque YAML del `.md` correspondiente.

### Formato canónico

```markdown
---
id: "<feature_id>-<action_id>"
action_id: spec
feature_id: nombre-feature
title: "Título"
date: "YYYY-MM-DD"
status: draft | in_progress | done
# ... metadatos según contrato de la acción
---
# Contenido legible en Markdown
...
```

## Artefactos por acción

| Acción | Fichero | Contenido Frontmatter (mínimo) |
| :--- | :--- | :--- |
| **objectives** | objectives.md | id, feature_id, branch, scope, ley_aplicada |
| **spec** | spec.md | id, action_id, feature_id, title, date, status, scope, acceptance_criteria |
| **clarify** | clarify.md | id, action_id, feature_id, decisions, clarify_pending |
| **planning** | plan.md | id, action_id, feature_id, phases, tasks |
| **implementation** | implementation.md | id, action_id, feature_id, touchpoints, items |
| **execution** | execution.md | id, action_id, feature_id, items_applied, status |
| **validate** | validacion.md | id, action_id, feature_id, global, checks, git_changes |
| **finalize-process** | finalize-process.md | id, action_id, feature_id, pr_url, branch, timestamp |

## Obligaciones

1. **Estructura:** Cada artefacto de acción es un único `.md` con YAML Frontmatter.
2. **Sincronidad:** Los metadatos en el frontmatter deben reflejar el estado real del documento; no hay paridad MD/JSON porque no existe `.json` separado.
3. **Validación:** La acción `validate` puede incluir un check que verifique la existencia y formato correcto del frontmatter en los artefactos de la carpeta de la tarea.
4. **Consumidores:** Las acciones que lean metadatos (clarify, planning, implementation, execution, validate, finalize-process) deben obtenerlos del frontmatter del `.md` correspondiente, no de ficheros `.json`.

## Referencias

- **entidades-dominio-ecosistema-sddia.md:** Patrón canónico (spec.md con Frontmatter) aplicado a skills, tools, actions, process, etc.
- **refactorization-arquitectura-frontmatter:** paths.featurePath/refactorization-arquitectura-frontmatter/ — migración dual → frontmatter.
- **C8 (clarify_decisions):** Documentación de proceso ha de respetar patrón frontmatter; process_interface y acciones actualizadas.

---
*Norma canónica para documentación de tarea. Ref: refactorization-arquitectura-frontmatter, entidades-dominio-ecosistema-sddia.*
