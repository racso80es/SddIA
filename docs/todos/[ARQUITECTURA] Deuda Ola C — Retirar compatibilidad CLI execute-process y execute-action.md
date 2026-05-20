---
document_id: TODO-OLA-C-CLI-COMPAT
title: "[ARQUITECTURA] Deuda Ola C — Retirar compatibilidad CLI legacy de orquestadores"
format: markdown
version: "1.0.0"
created: "2026-05-20"
status: "abierto"
priority: media
related:
  - SddIA/scripts/qa/execute-process.py
  - SddIA/scripts/qa/execute-action.py
  - docs/features/refactor-execute-process-engine/objectives.md
  - docs/features/refactor-execute-process-engine/validacion.md
  - docs/features/refactor-execute-process-engine/execution.md
---

**Entrega base completada:** intérprete dinámico y registry EDA en `main` (PR #9). Este TODO cubre la **fase 2** de limpieza de shims.
---

# Deuda técnica: eliminación de capas de compatibilidad (Ola C)

Tras la refactorización del intérprete dinámico (`refactor-execute-process-engine`), permanecen **shims temporales** que deben retirarse en un hito dedicado, adecuando todo el entorno al patrón canónico (igual que la migración de lógica embebida a intérprete agnóstico).

## 1. `--input-file` y envelope legacy en `execute-process.py`

| Estado actual | Objetivo |
|---------------|----------|
| `--input-file` + stdin envelope `{"process_name","process_inputs"}` con **warning stderr** | Solo `--process <nombre> --inputs '<json>'` |
| Atajo payload plano `entity-manager` | Documentar payload explícito con `--process entity-manager` |

### Tareas

- [ ] Inventariar referencias en `docs/features/**/execution.md`, TODOs operativos y scripts CI que usen `--input-file`.
- [ ] Migrar cada llamada al formato `--process` / `--inputs`.
- [ ] Eliminar `warn_deprecated_input_file()` y ramas `normalize_request` legacy no esenciales.
- [ ] Actualizar `SddIA/actions/execute-process.md` y guías de laboratorio.

## 2. Flag `--action` shim en `execute-process.py`

| Estado actual | Objetivo |
|---------------|----------|
| `--action X` redirige a `execute-action.py` vía subprocess con warning | Solo `execute-action.py --action X --inputs '...'` |

### Tareas

- [ ] Migrar tuberías que invocan `execute-process.py --action emit-pr-merged-event` (p. ej. `pbi-005-action-engine/execution.md`, `accept-pr`). Presentación PR: proceso `delivery-close-cycle` (no shim `--action` combinado).
- [ ] Eliminar `shim_execute_action()` y argumento `--action` del parser de procesos.
- [x] Handlers físicos en `execute-action.py` verificados (`emit-pr-*`, `emit-domain-mutation`) — ver `refactor-execute-process-engine/validacion.md`.

## Criterio de cierre

Ningún documento ni script del repositorio invoca las rutas deprecadas; los tests de humo del laboratorio usan exclusivamente el CLI canónico de procesos y acciones.
