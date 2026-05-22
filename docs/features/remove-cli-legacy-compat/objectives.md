---
feature_name: remove-cli-legacy-compat
created: "2026-05-21"
process: feature
branch_name: feat/remove-cli-legacy-compat
persist_ref: docs/features/remove-cli-legacy-compat
pbi_ref: PBI-005
---

# Objetivos — remove-cli-legacy-compat

## Misión

Retirar de manera definitiva la compatibilidad y capas legacy introducidas en la Ola C (scripts `execute-process.py` y `execute-action.py`), purgar los pseudo-argumentos `--input-file` y `--action`, y actualizar la documentación para asegurar el uso exclusivo del contrato canónico (`--process` con `--inputs` y enrutamiento a acciones a través del orquestador).

## Alcance (manifiesto)

1. En `execute-process.py`, eliminar el uso de `--input-file` y el atajo para `--action` que hace shim a `execute-action.py`.
2. En `execute-action.py`, eliminar el uso de `--input-file`.
3. Actualizar la lógica en `execute_process_core.py` (función `normalize_request` y avisos deprecados) para que solo soporte la forma moderna y explícita.
4. Inventariar y migrar tuberías y tests activos que todavía referencien la forma antigua de llamadas.
5. Dejar inmutables las evidencias históricas pasadas en `docs/features/**/execution.md`.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
- Invocaciones CLI seguirán el diseño de contratos estandarizados sin atajos ni compatibilidades obsoletas.
