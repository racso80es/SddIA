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
| **1 — Validación de purga** | Prueba de humo sobre `test-cli-skill` vía `execute-process` → `entity-manager` (`lifecycle_operation: delete`). | ✅ |
| **1b — Expansión DLT** | Suscriptor `cumulo` + `iota-immutable-publisher` en `Domain_Entity_Deleted`. | ✅ |
| **2 — Motor de acciones** | `execute-action.py`, `bus-operator`, `markdown-table-editor`, handler `feature`; PR #8 `caab46e`. | ✅ |
| **3a — Hooks `pre-commit`** | Aduana Argos (`pre_commit_gate`). | ✅ PR #12 |
| **3b — Hooks PR** | `pre-push` / `post-merge` (Ola B). | ⏳ Backlog |

## Fuera de alcance inmediato

- Instalación de hooks locales (Hito 3).
- Emisión `Domain_Entity_Created` en forja manual de tools (ver TODO EDA entidades).

## Ley aplicada

- Proceso `feature` v1.2.0; entrega Hito 2 documentada en `docs/features/pbi-005-action-engine/`.
- Git vía `git-manager`; bus SSOT: `cumulo.paths.json` → `eda_bus.pending`.

## Criterio de éxito (Hito 1 — cumplido)

- `test-cli-skill` purgado; evento `Domain_Entity_Deleted` en bus; PR #6 fusionado.
