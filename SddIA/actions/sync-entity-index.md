---
uuid: "a3f8c2e1-4b5d-6a7e-8f90-1a2b3c4d5e6f"
name: "sync-entity-index"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "entity-index-reconciliation"
  - "delegate-filesystem-manager"
  - "cumulo-catalog-sync"
inputs:
  - "entity_class": "string; enum: process | agent | skill | tool | action | codex (norm → no-op)"
  - "entity_name": "string; nombre canónico de la entidad"
  - "lifecycle_operation": "string; enum: create | update | delete"
  - "entity_uuid": "string; UUID v4 de la entidad (informativo en auditoría)"
  - "version": "string|null; versión resultante"
  - "hash_signature_new": "string|null; sello post-mutación"
  - "hash_signature_old": "string|null; sello pre-mutación"
outputs:
  - "success": "boolean"
  - "target_index_path": "string; ruta relativa al index.md auditado o purgado"
  - "message": "string; resultado de auditoría o purga"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: sync-entity-index

## 1. Propósito

Acción canónica del Agente **Cúmulo** para reconciliación asíncrona de catálogos (`index.md`) tras eventos `Domain_Entity_*` en el bus EDA.

| Capa | Responsabilidad |
|------|-----------------|
| **Creator (síncrono)** | En `create`/`update`, la fase Indexación del `*-creator` escribe la fila en `index.md` |
| **Esta acción (async)** | `create`/`update`: **auditoría** de idempotencia (fila presente); `delete`: **purga** de fila |

No mueve archivos del bus (jurisdicción del proceso `route-domain-event`). En runtime del demonio, delega la lógica determinista en `SddIA/scripts/qa/execute-action.py --action sync-entity-index`, que orquesta `tool:markdown-table-editor`.

## 2. Orquestación

Gate **Cerbero** por `context: ecosystem-evolution`. Rutas de índices vía `cumulo.paths.json` → `directories.*` + `/index.md`.

### Paso 1 — Triaje

| Condición | Resultado |
|-----------|-----------|
| `entity_class` es `norm` | `success: true`, mensaje de indexación ignorada |
| `entity_class` desconocida | `success: true`, no-op documentado |

### Paso 2 — Resolución de índice

| `entity_class` | Ruta relativa |
|----------------|---------------|
| `process` | `SddIA/process/index.md` |
| `agent` | `SddIA/agents/index.md` |
| `skill` | `SddIA/skills/index.md` |
| `tool` | `SddIA/tools/index.md` |
| `action` | `SddIA/actions/index.md` |
| `codex` | `SddIA/library/codexes/index.md` |

### Paso 3 — Operación por `lifecycle_operation`

**create / update (auditoría):**

1. Invocar `skill:filesystem-manager` con `READ_FILE` sobre el `index.md` objetivo (runtime IDE).
2. Comprobar que existe una fila de tabla que referencia `entity_name` (backticks o celda plain).
3. Si falta la fila: `success: false`, `message`: desincronización creator/bus.
4. Si existe: `success: true`, `target_index_path` relativo al workspace.

**delete (purga):**

1. Leer índice vía `filesystem-manager`.
2. Eliminar filas `|` que contengan el token `entity_name` (excluir separadores `---`).
3. Persistir índice actualizado vía `WRITE_FILE`.
4. En demonio: `execute-action.py` invoca `markdown-table-editor` (`delete_row` / `row_exists`).

### Paso 4 — Cierre (stdout envelope S+)

```json
{
  "success": true,
  "exitCode": 0,
  "data": {
    "success": true,
    "target_index_path": "SddIA/skills/index.md",
    "message": "Auditoría OK: fila presente."
  }
}
```

En fallo de auditoría: `data.success: false`, `exitCode: 0` (el watcher promueve el testigo del suscriptor a `dead-letter/` según `data.success`).

## 3. Límites

* Sin mutación del bus; sin emisión de eventos.
* No sustituye la fase Indexación síncrona del creator en create/update.
* `entity_name in line` puede dar falsos positivos en columna Descripción (v1 aceptado).
