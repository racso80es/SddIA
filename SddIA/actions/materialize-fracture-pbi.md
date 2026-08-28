---
uuid: "b2c3d4e5-f6a7-4890-b123-4567890abcde"
name: "materialize-fracture-pbi"
version: "1.1.0"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "fracture-pbi-materialization"
  - "fracture-pbi-resolver"
  - "delegate-filesystem-manager"
  - "cumulo-debt-ledger"
inputs:
  - "process_name": "string; proceso o cápsula que colapsó"
  - "error_trace": "string; traza o mensaje de error (origen de fracture_hash)"
  - "agent_emitter": "string; agente o runtime que emitió la fractura"
  - "attempted_action": "string; acción que falló (ej. delivery-close-cycle push)"
  - "persist_ref": "string; carpeta de persistencia si aplica (opcional)"
  - "branch_name": "string; rama en contexto si aplica (opcional)"
outputs:
  - "success": "boolean"
  - "target_path": "string; ruta del PBI abierto o materializado en docs/todos/pending/"
  - "message": "string; sinónimo legado de reason"
  - "reason": "string; already_open | deduped_by_process | regression_opened | materialized"
  - "canonical_ref": "string|null; ruta del PBI cerrado en done/ cuando reason=regression_opened"
  - "regression_n": "number|null; ordinal R<n> cuando reason=regression_opened"
  - "trace_hash": "string; 12 hex SHA-256 de error_trace"
hash_signature: "sha256:a7cdc5069f083403fb8a9579755e65d63a56c523d6221399e51cbcda55e6240b"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: materialize-fracture-pbi

## 1. Propósito

Acción canónica del Agente **Cúmulo** ante `System_Fracture_Detected`. Materializa o resuelve un PBI `bug-fix` en `docs/todos/pending/` según el **genoma YAML** (`fracture_hash`, `fracture_process`) en `pending/` y `done/`. El nombre de fichero es presentación; el motor no parsea prefijos `[FIX]`/`[REGRESIÓN]`.

## 2. Orquestación

### Paso 1 — Validación

Campos obligatorios: `process_name`, `error_trace`, `agent_emitter`, `attempted_action`.

### Paso 2 — Resolución (Core `fracture_pbi`)

1. Calcular `trace_hash` = 12 hex SHA-256(`error_trace`).
2. Calcular `fracture_process` = slug del `process_name`.
3. Barrer `paths.todos.pending` y `paths.todos.done` (SSOT `cumulo.paths.json`).
4. Precedencia §4.1:
   - `already_open` — homólogo abierto con mismo `fracture_hash`.
   - `deduped_by_process` — abierto con mismo `fracture_process` y distinto hash.
   - `regression_opened` — cerrado en `done/` con mismo hash → nuevo PBI `[REGRESIÓN]` con `regression_of` al canónico (inmutable).
   - `materialized` — sin homólogo → nuevo `[FIX]` en `pending/`.

### Paso 3 — Persistencia (solo `materialized` / `regression_opened`)

Frontmatter YAML del PBI incluye obligatoriamente: `fracture_hash`, `fracture_process`, `document_id` (`PBI-FIX-FRACTURE-<hash>` o `…-R<n>`), `process: bug-fix`, `status: abierto`, `priority: alta`. Regresiones incluyen `regression_of: <document_id_canónico>`.

### Paso 4 — Cierre (stdout)

Envelope con `success: true`, `target_path`, `reason`, y campos opcionales `canonical_ref`, `regression_n`, `trace_hash`.

## 3. Límites

* No repara la fractura ni autoriza bypass.
* No resuelve por hash de nombre de fichero.
* No mueve archivos del bus (proceso `route-domain-event`).
