---
feature_name: pbi-005-hito2-action-engine
created: "2026-05-20"
process: feature
phases: 6
---

# Plan — Hito 2 PBI-005

## Fase 1 — Inicialización

1. Payload `tmp/feature-pbi005-hito2-init.json` → `execute-process.py`
2. Verificar rama `feat/pbi-005-action-engine`

## Fase 2 — Tool tablas

Contrato y cápsula ya en main; prueba de humo `parse` + `row_exists`.

## Fase 3 — bus-operator

1. `SddIA/skills/bus-operator.md`
2. Micro-tools bajo `SddIA/scripts/tools/{read-event-subscriptions,manage-event-receipt,transit-event-payload}/`
3. Cápsula `scripts/skills/bus-operator.py`
4. Fila en `SddIA/skills/index.md`

## Fase 4 — execute-action

Reemplazar invocación directa a markdown-table-editor por `bus-operator.sync_entity_index`.

## Fase 5 — Watcher

Confirmar `_dispatch_subscriber` usa CLI `execute-action.py` (sin import legacy).

## Fase 6 — Argos

Evento sintético `Domain_Entity_Created` + watcher `--once` + `validacion.md` **APTO**.
