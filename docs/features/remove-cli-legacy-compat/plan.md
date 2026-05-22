---
feature_name: remove-cli-legacy-compat
created: "2026-05-21"
process: feature
branch_name: feat/remove-cli-legacy-compat-12759319319782396173
persist_ref: docs/features/remove-cli-legacy-compat
---

# Plan — remove-cli-legacy-compat

## Fases

1. **Purga `execute-action.py`** — eliminar `--input-file` y referencias en docstring.
2. **Purga `execute-process.py`** — eliminar `--input-file`, shim `--action` y warnings en docstring.
3. **Purga `execute_process_core.py`** — retirar `warn_deprecated_*` y envelope legacy en `normalize_request`.
4. **Purga `execute_process_capsules.py`** — retirar `shim_execute_action`; mantener `invoke_capsule_action` vía CLI canónico de acciones.
5. **Inventario scripts activos** — verificar `SddIA/scripts/**` sin rutas deprecadas (p. ej. `audit-entity-eda-coverage.py`, `event-watcher.py`).
6. **Revertir ruido en `SddIA/process/`** — descartar commits de resincronización de `hash_signature` ajenos al TODO.
7. **Documentación feature** — `spec.md`, `implementation.md`, `validacion.md` con frontmatter atómico.
8. **PR + aduana** — push, `PullRequest_Presented`, `pull-request-review` en perfil producción.

## Ley

- Evidencias históricas en `docs/features/**/execution.md` cerradas permanecen inmutables.
- Git vía `skill:git-manager` / `gh` solo en fases de entrega del proceso `feature`.
