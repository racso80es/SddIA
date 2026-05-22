---
feature_name: remove-cli-legacy-compat
created: "2026-05-21"
process: feature
items:
  - purge-execute-action-input-file
  - purge-execute-process-legacy-flags
  - purge-execute-process-core-legacy-warnings
  - purge-execute-process-capsules-shim
  - sddia-tools-index-fix
---

# Implementación — touchpoints

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/qa/execute-action.py` | Retirado `--input-file` y docstring legacy |
| `SddIA/scripts/qa/execute-process.py` | Retirados `--input-file`, `--action`, ramas shim |
| `SddIA/scripts/qa/execute_process_core.py` | Eliminados `warn_deprecated_*`; `normalize_request` solo forma estricta |
| `SddIA/scripts/qa/execute_process_capsules.py` | Eliminado `shim_execute_action`; invocación directa a `execute-action.py` en `invoke_capsule_action` |
| `.SddIA/tools/index.md` | Índice local requerido por `verify-tools-index` (CI) |
| `docs/features/remove-cli-legacy-compat/*` | Manifiesto feature + validación |

## Excluido explícitamente

| Área | Motivo |
|------|--------|
| `SddIA/process/*.md` | Commits de `hash_signature` no pertenecen a la deuda CLI; revertidos a `main` |
