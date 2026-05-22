---
feature_name: remove-cli-legacy-compat
created: "2026-05-21"
process: feature
branch_name: feat/remove-cli-legacy-compat
executed_at: "2026-05-21"
items_applied:
  - purge-execute-action-input-file
  - purge-execute-process-legacy-flags
  - purge-execute-process-core-legacy-warnings
  - purge-execute-process-capsules-shim
---

# Ejecución — remove-cli-legacy-compat

Registro de entrega: eliminación de deuda técnica introducida en Ola C.

## Tareas realizadas

| # | Tarea | Estado |
|---|--------|--------|
| 1 | Purga de `execute-action.py` (eliminación de `--input-file` y docstring viejo) | ✅ |
| 2 | Purga de `execute-process.py` (eliminación de `--input-file`, shim de `--action` y docstring viejo) | ✅ |
| 3 | Purga de `execute_process_core.py` (eliminación de warnings deprecated y soporte a envelope legacy en `normalize_request`) | ✅ |
| 4 | Purga de `execute_process_capsules.py` (eliminación del método `shim_execute_action` y de la ruta a execute-action.py no usada) | ✅ |
| 5 | Migración de Scripts Activos | ✅ |

## Comandos reproducibles (Nuevo estándar canónico)

### Inicialización feature

```bash
python SddIA/scripts/qa/execute-process.py --process feature --inputs '{"feature_name": "remove-cli-legacy-compat", "base_branch": "main"}'
```

### Ejecución directa a procesos

```bash
python SddIA/scripts/qa/execute-process.py --process process-name --inputs '{"key": "value"}'
```

### Ejecución de acciones

```bash
python SddIA/scripts/qa/execute-action.py --action action-name --inputs '{"key": "value"}'
```
