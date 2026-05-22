---
uuid: "b2c3d4e5-f6a7-4890-b123-4567890abcde"
name: "materialize-fracture-pbi"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "fracture-pbi-materialization"
  - "delegate-filesystem-manager"
  - "cumulo-debt-ledger"
inputs:
  - "process_name": "string; proceso o cápsula que colapsó"
  - "error_trace": "string; traza o mensaje de error"
  - "agent_emitter": "string; agente o runtime que emitió la fractura"
  - "attempted_action": "string; acción que falló (ej. delivery-close-cycle push)"
  - "persist_ref": "string; carpeta de persistencia si aplica (opcional)"
  - "branch_name": "string; rama en contexto si aplica (opcional)"
outputs:
  - "success": "boolean"
  - "target_path": "string; ruta del PBI generado en docs/todos/pending/"
  - "message": "string; resultado (nuevo | idempotente)"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: materialize-fracture-pbi

## 1. Propósito

Acción canónica del Agente **Cúmulo** (Gestor de Deuda Técnica) ante `System_Fracture_Detected`. Materializa un PBI categorizado `bug-fix` en `docs/todos/pending/` — el **Qué** ha fallado — liberando a la IA ejecutora de redactar la deuda manualmente. Deja placeholder para enriquecimiento **Mayeuta** (`enrich-fracture-pbi-kaizen`).

## 2. Orquestación

### Paso 1 — Validación

Campos obligatorios: `process_name`, `error_trace`, `agent_emitter`, `attempted_action`.

### Paso 2 — Idempotencia

Hash SHA-256 truncado (12 hex) de `error_trace` → sufijo de nombre de archivo. Si el PBI ya existe, `success: true` con mensaje idempotente.

### Paso 3 — Persistencia

Escribir markdown con frontmatter YAML: `process: bug-fix`, `status: abierto`, `priority: alta`, objetivos mínimos y referencia al protocolo Kintsugi.

### Paso 4 — Cierre (stdout)

Envelope con `success`, `target_path`, `message`.

## 3. Límites

* No repara la fractura ni autoriza bypass.
* No mueve archivos del bus (`route-domain-event`).
