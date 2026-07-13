---
uuid: "b2c4e6f8-1a3d-4e5b-9c7d-8e1f2a3b4c5d"
name: "markdown-table-editor"
version: "1.0.0"
contract: "tools-contract v1.2.0"
contract_ref: "SddIA/tools/tools-contract.md"
domain_origin: "SddIA"
context: "ecosystem-evolution"
capabilities:
  - "markdown-table-parse"
  - "markdown-table-row-upsert"
  - "markdown-table-row-delete"
  - "markdown-table-persist"
  - "capsule-json-io"
implementation_path_ref: "SddIA/tools/markdown-table-editor"
---

# markdown-table-editor

**Descripción:** Cápsula determinista para parsear, consultar, mutar y persistir filas de tablas Markdown en catálogos `index.md` y artefactos tabulares del ecosistema. Diseñada para delegación de Cúmulo y acciones de dominio (`sync-entity-index`).

## Interface

Entrada por **stdin** (JSON) o `--request-file`. Salida: envelope `tools-contract v1.2.0`.

### Inputs

| Campo | Tipo | Obligatorio | Descripción |
|-------|------|-------------|-------------|
| `file_path` | string | Sí* | Ruta relativa al workspace del archivo `.md` objetivo. |
| `operation` | string | Sí | `parse` \| `row_exists` \| `delete_row` \| `upsert_row` \| `save` |
| `key_column` | string \| number | No | Nombre de columna (cabecera) o índice 0-based. Si se omite en `delete_row`/`row_exists`, usa `match_token` en la fila completa. |
| `row_data` | object | Condicional | Valores de búsqueda o fila a insertar/actualizar. |
| `match_token` | string | No | Alias de `row_data.token` para modo legado (subcadena en fila). |
| `table_index` | number | No | Índice de tabla en el archivo (default `0`). |
| `dry_run` | boolean | No | Si `true`, no escribe en disco (default `false`). |

\* En `save` sin mutación previa en la misma invocación, `file_path` sigue siendo obligatorio.

### Outputs (envelope)

| Campo | Tipo | Descripción |
|-------|------|-------------|
| `success` | boolean | Éxito de la operación. |
| `exitCode` | number | `0` solo si `success` es true. |
| `message` | string | Resumen breve. |
| `error` | string | Causa en fallo (opcional en éxito). |
| `result` | object | Payload específico (`rows`, `row_count`, `modified`, `target_path`, etc.). |

### Operaciones

| `operation` | Comportamiento |
|-------------|----------------|
| `parse` | Devuelve cabeceras y filas de datos de la tabla objetivo sin mutar. |
| `row_exists` | `result.exists: boolean` según `key_column` + `row_data` o `match_token`. |
| `delete_row` | Elimina filas coincidentes; idempotente si ya no existen (`modified: false`). |
| `upsert_row` | Inserta o reemplaza fila por clave; requiere `row_data` con celdas completas o parciales. |
| `save` | Persiste buffer interno (uso avanzado; las mutaciones persisten por defecto al finalizar). |

## Security

- Sin secretos en stdin/stdout.
- Rutas resueltas solo bajo la raíz del workspace (rechazo de path traversal).
- Escritura atómica: archivo temporal + `replace`.

## Delegación

Invocada por `./sddia-run.sh --action sync-entity-index` (acción `sync-entity-index`, agente Cúmulo) y por agentes en runtime IDE vía orquestación documentada en la acción.
