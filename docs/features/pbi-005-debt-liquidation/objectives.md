---
feature_name: pbi-005-debt-liquidation
created: "2026-05-19"
process: feature
branch_name: feat/pbi-005-debt-liquidation
persist_ref: docs/features/pbi-005-debt-liquidation
pbi_ref: PBI-005
---

# Objetivos — PBI-005: Liquidación de Pasivos (Ola A)

## Misión

Liquidar pasivos técnicos heredados de la **Ola A** validando el mecanismo destructivo del genoma (purga física + reconciliación de catálogo) y expandiendo la coreografía EDA para anclar en DLT las mutaciones `Domain_Entity_Deleted`, antes del despliegue masivo de la Ola C.

## Alcance por hitos (matriz PBI-005)

| Hito | Faena | Estado |
|------|-------|--------|
| **1 — Validación de purga** | Prueba de humo sobre `test-cli-skill` vía `execute-process` → `entity-manager` (`lifecycle_operation: delete`). Verificar borrado del `.md`, purga de fila en `SddIA/skills/index.md`, evento en `docs/events/pending/`. | 🔄 En curso |
| **1b — Expansión DLT** | Añadir suscriptor `cumulo` + `iota-immutable-publisher` en `Domain_Entity_Deleted` (`event-subscriptions.json`), simétrico a `PullRequest_Merged`. | 🔄 En curso |
| **2 — Motor de acciones** | `execute-action.py` universal; desacoplar `sync-entity-index.py` del acoplamiento rígido en daemon. | ⏳ Backlog |
| **3 — Hooks Git** | Automatización `PullRequest_Presented` / `PullRequest_Merged` en `.git/hooks/`. | ⏳ Backlog |

## Fuera de alcance inmediato

- Forja de `tool:markdown-table-editor` y refactor completo del watcher (Hito 2).
- Instalación de hooks locales (Hito 3).

## Ley aplicada

- Proceso `feature` v1.2.0 (`SddIA/process/feature.md`).
- Norma `features-documentation-pattern` v1.0.0.
- Git exclusivamente vía `git-manager` (`scripts/skills/git-manager.py`).
- SSOT rutas: `SddIA/core/cumulo.paths.json` → `eda_bus.pending` = `docs/events/pending`.

## Criterio de éxito (Hito 1)

- `SddIA/skills/test-cli-skill.md` ausente en disco.
- Fila de `test-cli-skill` purgada de `SddIA/skills/index.md`.
- Instancia ECST `Domain_Entity_Deleted` sellada en `docs/events/pending/*.json`.
- Genoma de suscripciones incluye anclaje DLT para `Domain_Entity_Deleted`.
