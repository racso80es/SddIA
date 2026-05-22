---
document_id: TODO-OLA-C-CLI-COMPAT
title: "[ARQUITECTURA] Deuda Ola C — Retirar compatibilidad CLI legacy de orquestadores"
format: markdown
version: "1.1.0"
created: "2026-05-20"
closed: "2026-05-22"
status: "cerrado"
priority: media
pr_url: "https://github.com/racso80es/SddIA/pull/14"
pr_follow_up: "https://github.com/racso80es/SddIA/pull/18"
feature_ref: docs/features/remove-cli-legacy-compat
merge_commit: "b44402bdd404f51c976abbe83a3fe67ee835560e"
related:
  - SddIA/scripts/qa/execute-process.py
  - SddIA/scripts/qa/execute-action.py
  - SddIA/scripts/qa/execute_process_core.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - docs/features/refactor-execute-process-engine/
  - docs/features/remove-cli-legacy-compat/
  - docs/todos/[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio.md
---

# Deuda Ola C — CLI legacy: CERRADO (núcleo en `main`)

> **Fase 2** de `refactor-execute-process-engine` (PR #9). Entrega núcleo vía feature `remove-cli-legacy-compat` — **PR #14 MERGED** (`b44402bd`, 2026-05-22).  
> Seguimiento documental / Kaizen EDA: **PR #18** abierto (`feat/docs-remove-cli-validacion-evidence`).

## Situación actual

| Ámbito | Estado |
|--------|--------|
| `execute-process.py` | Sin `--input-file`, sin `--action` shim |
| `execute-action.py` | Sin `--input-file` |
| `execute_process_core.py` | Sin `warn_deprecated_*`; `normalize_request` solo forma estricta `--process` + `--inputs` |
| `execute_process_capsules.py` | Sin `shim_execute_action`; `invoke_capsule_action` → CLI canónico de acciones |
| Scripts activos `SddIA/scripts/**` | Sin invocaciones `--input-file` / `execute-process --action` |
| Aduana PR #14 | `pull-request-review` v2 → `verdict: aprobado` (producción) |
| Bus EDA retroactivo | `PullRequest_Presented` `980725c5-…` → `processed/` (`argos` + `cumulo: success`) — ver `validacion.md` |

## Entregas

| Referencia | Descripción |
|------------|-------------|
| [PR #14](https://github.com/racso80es/SddIA/pull/14) | Purga CLI Ola C + manifiesto feature (merge `54bb11a` → `main`) |
| [PR #18](https://github.com/racso80es/SddIA/pull/18) | Evidencia `validacion.md`, `infer_persist_ref_from_branch` (rama Jules), watcher v2 + `merge_already_done` |
| `docs/features/remove-cli-legacy-compat/` | `objectives`, `spec`, `plan`, `implementation`, `execution`, `validacion` |

## Tareas — §1 `--input-file` y envelope legacy

| Tarea | Estado | Notas |
|-------|--------|-------|
| Inventariar referencias en `docs/features/**/execution.md` y CI | [x] | Rutas legacy solo en **evidencias históricas** (features cerradas); no se reescriben por manifiesto `remove-cli-legacy-compat` |
| Migrar llamadas en scripts / tuberías activas | [x] | `SddIA/scripts/**` limpio (A4 validación) |
| Eliminar `warn_deprecated_input_file()` y ramas legacy en `normalize_request` | [x] | PR #14 |
| Actualizar `SddIA/actions/execute-process.md` y guías de laboratorio | [ ] | Pendiente menor; contrato canónico ya en docstrings CLI |

## Tareas — §2 Flag `--action` shim

| Tarea | Estado | Notas |
|-------|--------|-------|
| Migrar tuberías `execute-process --action emit-pr-*` | [x] | Runtime: `delivery-close-cycle` / `accept-pr` + `execute-action.py`; históricos inmutables |
| Eliminar `shim_execute_action()` y `--action` del parser de procesos | [x] | PR #14 |
| Handlers físicos en `execute-action.py` | [x] | Heredado PR #9 — `refactor-execute-process-engine/validacion.md` |

## Criterio de cierre

| Criterio | Cumple |
|----------|--------|
| Ningún **script activo** invoca rutas deprecadas | Sí |
| Smokes laboratorio / aduana usan CLI canónico | Sí (`validacion.md` A1–A5) |
| Evidencias históricas en `execution.md` cerrados | Excepción documentada (inmutables) |

## Deuda residual (no bloqueante)

1. **`SddIA/actions/execute-process.md`** — alinear redacción al CLI post-Ola C (sin `--input-file` / `--action`).
2. **PR #18** — merge de Kaizen watcher (`code_diff` / `tasks_path` / `document_context`, `github_pr_merged`, sufijo Jules en `persist_ref`).
3. **Backlog operativo** — ítem OC.* en `[OPERATIVO] Backlog pendiente post-PR11` puede archivarse o marcarse obsoleto tras merge PR #18.

## Comandos canónicos (referencia)

```powershell
python SddIA/scripts/qa/execute-process.py --process <nombre> --inputs '<json>'
# o --inputs-file <path>
python SddIA/scripts/qa/execute-action.py --action <nombre> --inputs '<json>'
```

Rutas deprecadas responden `unrecognized arguments` (ver `docs/features/remove-cli-legacy-compat/validacion.md`).
