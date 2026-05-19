---
feature_name: pbi-005-hito2-action-engine
created: "2026-05-19"
process: feature
branch_name: feat/pbi-005-action-engine
persist_ref: docs/features/pbi-005-hito2-action-engine
pbi_ref: PBI-005
---

# Objetivos — PBI-005 Hito 2: Motor de Acciones

## Misión

Forjar el intérprete universal `execute-action.py`, la herramienta `markdown-table-editor` para soberanía tabular de Cúmulo, y desacoplar el demonio EDA del script legacy `sync-entity-index.py`.

## Alcance (Asalto 1)

| Entregable | Descripción |
|------------|-------------|
| `execute-action.py` | Puerta lógica CLI (`--action`, `--inputs` / `--input-file`) que carga contratos `.md` de acciones y orquesta handlers físicos. |
| `tool:markdown-table-editor` | Parseo, actualización, purga de filas y persistencia idempotente de tablas Markdown. |
| `event-watcher.py` | Despacho genérico vía `execute-action.py`; sin imports ni ramas rígidas a scripts ad-hoc. |
| Purga | Eliminar `SddIA/scripts/qa/sync-entity-index.py`. |

## Ley aplicada

- Proceso `feature` v1.2.0; PBI-005 matriz Hito 2 (Ola A).
- Acciones: `actions-contract v1.2.0`; Tools: `tools-contract v1.2.0`.
