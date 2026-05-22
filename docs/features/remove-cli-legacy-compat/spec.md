---
feature_name: remove-cli-legacy-compat
created: "2026-05-21"
process: feature
scope: ola-c-cli-shim-removal
version_spec: "1.0.0"
related_todo: docs/todos/[ARQUITECTURA] Deuda Ola C — Retirar compatibilidad CLI execute-process y execute-action.md
---

# Especificación — Retirada compatibilidad CLI Ola C

## 1. Alcance

Eliminar rutas deprecadas en los orquestadores de laboratorio sin tocar genomas de proceso (`SddIA/process/*.md` salvo evidencia histórica inmutable en features cerradas).

| Componente | Cambio |
|------------|--------|
| `execute-process.py` | Sin `--input-file`, sin `--action` shim |
| `execute-action.py` | Sin `--input-file` |
| `execute_process_core.py` | Sin warnings deprecados; `normalize_request` rechaza envelope legacy |
| `execute_process_capsules.py` | Sin `shim_execute_action`; `invoke_capsule_action` llama `execute-action.py` canónico |

## 2. Contrato CLI canónico (post-entrega)

```text
execute-process.py --process <nombre> [--inputs '<json>' | --inputs-file <path> | stdin]
execute-action.py  --action <nombre>  [--inputs '<json>' | stdin]
```

## 3. Fuera de alcance

- Resincronización masiva de `hash_signature` en `SddIA/process/**` (no es deuda CLI).
- Migración de `docs/features/**/execution.md` históricos (inmutables por manifiesto).
- Retirada de `--inputs-file` en `execute-process.py` (sigue siendo alternativa canónica a `--inputs`).

## 4. Criterios de aceptación

| ID | Criterio |
|----|----------|
| A1 | `execute-process.py --input-file` → error de argparse |
| A2 | `execute-process.py --action X` → error de argparse |
| A3 | `execute-action.py --input-file` → error de argparse |
| A4 | Ningún script activo bajo `SddIA/scripts/**` invoca rutas A1–A3 |
| A5 | Smoke `execute-process --process feature` y acciones EDA siguen operativos |
