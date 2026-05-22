---
uuid: "4c448c82-de41-460f-b24f-82a84fa5ed69"
name: "features-documentation-pattern"
version: "1.1.0"
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
| validate | validacion.md | feature_name, branch, global, checks, git_changes; post-merge: merged_pr, merge_commit, closed, pbi_archived | Informe de validación |
| finalize-process | finalize-process.md (opc.) | feature_name, pr_url, timestamp | Resumen de cierre |

La validación opcional `sddia_frontmatter_valid` aplica a los `.md` de tarea cuando el diff los toque. Las tareas existentes con `.json` deben migrarse consolidando el contenido en el frontmatter del `.md` correspondiente y eliminando el `.json`. Las nuevas documentaciones cumplen este patrón desde el inicio.

## Validación en dos fases (`validacion.md`)

`validacion.md` se completa en **dos momentos** del ciclo de tarea (features, fixes y refactorizations):

### Fase A — Pre-merge (Argos)

Emitida tras Tekton, **antes** de abrir o mergear el PR.

| Campo | Obligatorio | Valor típico |
|-------|-------------|--------------|
| `global` | Sí | `APTO` \| `NO_APTO` |
| `branch` | Sí | Rama de trabajo (`fix/*`, `feat/*`) |
| `checks` | Sí | Mapa de criterios de aceptación |
| `git_changes` | Sí | Lista de paths tocados |
| `merged_pr` | Reservado | `null` o omitido |
| `merge_commit` | Reservado | `null` o omitido |
| `closed` | Reservado | `null` o omitido |
| `pbi_archived` | Sí | `false` |

### Fase B — Post-merge (cierre documental)

Emitida **después** del merge en `main`. Obligatoria para declarar la tarea cerrada.

| Campo | Obligatorio | Valor |
|-------|-------------|-------|
| `merged_pr` | Sí | Número o URL del PR mergeado |
| `merge_commit` | Sí | OID del commit de merge en `main` |
| `closed` | Sí | Fecha ISO del cierre |
| `pbi_archived` | Sí | `true` cuando exista el PBI en `docs/todos/done/` |

**Restricción:** Prohibido considerar cierre definitivo de la tarea si, con merge ya conocido, `pbi_archived: false` o faltan `merged_pr` / `merge_commit` en `validacion.md`.

**Commit obligatorio:** Los cambios de Fase B (PBI archivado + frontmatter actualizado) deben commitearse y pushearse a `main` en el mismo acto de cierre. Ver `bug-fix` § Cierre documental post-merge.

## Restricciones Duras (Aduana de Fricción)

- Prohibido generar o mantener `spec.json`, `clarify.json`, `plan.json`, `implementation.json`, `execution.json`, `validacion.json` o `finalize-process.json` como fuente de verdad paralela al `.md` de la misma acción.
- Prohibido omitir el bloque frontmatter YAML (`---` … `---`) en cualquier artefacto de tarea.
- Prohibido producir más de un archivo por acción de fase (un `.md` por acción, sin duplicados ni variantes JSON).
- Prohibido almacenar en el cuerpo Markdown datos que deban ser machine-readable si pueden declararse en frontmatter según la tabla canónica.
- Prohibido crear nuevas tareas que violen este patrón; la migración de legado es la única excepción temporal y debe cerrarse eliminando el `.json`.

## Ruido de Sistema — Cobertura EDA genómica

Cuando una feature muta entidades bajo `SddIA/` (skills, events, process, agents, tools, actions, norms, codexes):

- Toda entidad indexada debe tener correlato `Domain_Entity_Created` en el bus local (`eda_bus`).
- El gate **Aduana EDA genómica** en `delivery-close-cycle` invoca `audit-entity-eda-coverage.py --scan --json`.
- **Ruido de Sistema (block):** `orphan_count > 0` — artefacto `.md` + fila en `index.md` sin evento ECST correlacionado por `entity_uuid`.
- **Excepción:** backfill Fase C documentado en la feature (`--emit --skip-dlt` + cierre con `--anchor-merkle` y `transaction_digest` registrado).
- Forja directa de `.md` sin pasar por `entity-manager` → huérfana EDA hasta backfill (caso histórico: placeholders forjados fuera del gestor).
