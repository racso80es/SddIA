---
uuid: "4c448c82-de41-460f-b24f-82a84fa5ed69"
name: "features-documentation-pattern"
version: "1.2.1"
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
| validate | validacion.md | feature_name, branch, global, checks, git_changes, pbi_archived; opc.: pr_url, merged_pr, merge_commit, closed | Informe de validación |
| finalize-process | finalize-process.md (opc.) | feature_name, pr_url, timestamp | Resumen de cierre |

La validación opcional `sddia_frontmatter_valid` aplica a los `.md` de tarea cuando el diff los toque. Las tareas existentes con `.json` deben migrarse consolidando el contenido en el frontmatter del `.md` correspondiente y eliminando el `.json`. Las nuevas documentaciones cumplen este patrón desde el inicio.

## Validación en fase única — pre-merge (`validacion.md`)

`validacion.md` se completa **una sola vez**, en la rama del PR, **antes** del merge en `main` (features, fixes y refactorizations). El merge en forja cierra la tarea; no se exige un segundo PR ni commit documental a `main`.

| Campo | Obligatorio | Valor típico |
|-------|-------------|--------------|
| `global` | Sí | `APTO` \| `NO_APTO` |
| `branch` | Sí | Rama de trabajo (`fix/*`, `feat/*`, `docs/*`) |
| `checks` | Sí | Mapa de criterios de aceptación |
| `git_changes` | Sí | Lista de paths tocados |
| `pbi_archived` | Sí | `true` — PBI ya en `docs/todos/done/` **en esta rama** |
| `pr_url` | Recomendado | URL tras `delivery-close-cycle` (puede añadirse en el mismo PR antes del merge) |
| `merged_pr` | No | Opcional auditoría; **no** gate de Done |
| `merge_commit` | No | Inferible vía GitHub / `git log` |
| `closed` | No | Opcional |

**Definición de Done (documental):**

```text
Done = un único PR mergeado en main
     + validacion.md APTO en el diff de ese PR (pbi_archived: true)
     + PBI en docs/todos/done/ en esa misma rama
```

**Restricciones:**

- Prohibido exigir `merged_pr` / `merge_commit` obligatorios para declarar la tarea cerrada.
- Prohibido abrir un PR `docs/cerrar-pbi-*` solo para rellenar campos post-merge (Kaizen 2026-05-22).
- Prohibido `pbi_archived: true` si el PBI sigue solo en `docs/todos/pending/`.

### Migración desde v1.1.0 (Fase B)

Los artefactos históricos con `merged_pr` / `merge_commit` en `validacion.md` permanecen válidos como auditoría. Las tareas nuevas aplican v1.2.0 desde el merge de `kaizen-cierre-documental-single-pr`.

### Trazabilidad de merge (fuera del frontmatter obligatorio)

- Número de PR y OID de merge: GitHub API, `gh pr view`, o commit de merge en `main`.
- No duplicar en `validacion.md` salvo necesidad de auditoría explícita del operador.

## Restricciones Duras (Aduana de Fricción)

- Prohibido generar o mantener `spec.json`, `clarify.json`, `plan.json`, `implementation.json`, `execution.json`, `validacion.json` o `finalize-process.json` como fuente de verdad paralela al `.md` de la misma acción.
- Prohibido omitir el bloque frontmatter YAML (`---` … `---`) en cualquier artefacto de tarea.
- Prohibido producir más de un archivo por acción de fase (un `.md` por acción, sin duplicados ni variantes JSON).
- Prohibido almacenar en el cuerpo Markdown datos que deban ser machine-readable si pueden declararse en frontmatter según la tabla canónica.
- Prohibido crear nuevas tareas que violen este patrón; la migración de legado es la única excepción temporal y debe cerrarse eliminando el `.json`.

## Artefactos efímeros (inputs runtime)

Los `.md` de fase siguen siendo la fuente de verdad documental. Los **inputs JSON de runtime** (smokes one-shot, hooks, cierres ad hoc) **no** forman parte del patrón de fase:

| Tipo | Convención | Versionado |
|------|------------|------------|
| Fixture plantilla | `_smoke-<escenario>.json` en `persist_ref` | Sí — plantilla reproducible en el PR de la feature |
| Input operativo | `.tmp/<proceso>-<uuid>.json` vía `tmp_paths.write_ephemeral_json` | No — borrar tras `execute-process` |
| Cierre ciclo ad hoc | Copiar plantilla a `.tmp/`; prohibido `_close-cycle-*.json` suelto en `persist_ref` | No |

Ver `SddIA/norms/git-operations.md` §3 y Kaizen `kaizen-higiene-ficheros-temporales`.

## Ruido de Sistema — Cobertura EDA genómica

Cuando una feature muta entidades bajo `SddIA/` (skills, events, process, agents, tools, actions, norms, codexes):

- Toda entidad indexada debe tener correlato `Domain_Entity_Created` en el bus local (`eda_bus`).
- El gate **Aduana EDA genómica** en `delivery-close-cycle` invoca `audit-entity-eda-coverage.py --scan --json`.
- **Ruido de Sistema (block):** `orphan_count > 0` — artefacto `.md` + fila en `index.md` sin evento ECST correlacionado por `entity_uuid`.
- **Excepción:** backfill Fase C documentado en la feature (`--emit --skip-dlt` + cierre con `--anchor-merkle` y `transaction_digest` registrado).
- Forja directa de `.md` sin pasar por `entity-manager` → huérfana EDA hasta backfill (caso histórico: placeholders forjados fuera del gestor).
