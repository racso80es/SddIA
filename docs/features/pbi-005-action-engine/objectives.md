---
feature_name: pbi-005-hito2-action-engine
created: "2026-05-20"
process: feature
branch_name: feat/pbi-005-action-engine
persist_ref: docs/features/pbi-005-action-engine
pbi_ref: PBI-005
---

# Objetivos — PBI-005 Hito 2: Motor de Acciones y Anatomía de Capas

## Misión

Materializar el **motor universal de acciones** (`execute-action.py`), blindar la separación **Skill (dominio) / Tool (mecánica)** mediante `skill:bus-operator`, y dejar el demonio `event-watcher` ciego respecto a scripts legacy (`sync-entity-index.py` purgado).

## Alcance (Asalto 1)

| Fase | Entregable | Estado |
|------|------------|--------|
| 1 | Proceso `feature` iniciado (rama + `persist_ref`) | ✅ |
| 2 | `tool:markdown-table-editor` (contrato + cápsula) | ✅ (preexistente en main) |
| 3 | `skill:bus-operator` + micro-tools del bus | ✅ |
| 4 | `execute-action.py` enlazado a capas | ✅ |
| 5 | Watcher → `execute-action` (sin import legacy) | ✅ (preexistente) |
| 6 | Auditoría Argos / `validacion.md` | ✅ |

## Fuera de alcance

- Hooks Git locales (Hito 3 PBI-005).
- Runtime IDE completo (Mayeuta/Dedalo/Tekton automáticos).

## Ley aplicada

- SSOT rutas: `cumulo.paths.json`
- Idempotencia en tablas y tránsitos de bus
- Git exclusivamente vía `git-manager`

## Criterio de éxito

Daemon en modo `--once` procesa `Domain_Entity_Created` y reconcilia índice vía `execute-action` → `bus-operator` → `markdown-table-editor` sin corrupción de cabeceras.
