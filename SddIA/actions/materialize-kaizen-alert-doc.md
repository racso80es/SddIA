---
uuid: "d7e6f5a4-b3c2-4109-8765-43210abcdef0"
name: "materialize-kaizen-alert-doc"
version: "1.1.0"
contract: "actions-contract v1.2.0"
context: "quality-assurance"
capabilities:
  - "kaizen-alert-doc-materialization"
  - "delegate-filesystem-manager"
  - "cumulo-debt-ledger"
inputs:
  - "review_id": "string; UUID v4 o correlation_id de la aduana"
  - "alert_justification": "string; código máquina + texto breve"
  - "implicated_files": "string[]; rutas repo implicadas (monitored_hits)"
  - "persist_ref": "string; feature bajo revisión (opcional)"
  - "pr_branch": "string; rama del PR (opcional)"
  - "alert_kind": "string; default doc_parity (opcional)"
  - "impacts_doc": "boolean | null; valor DIA resuelto (opcional)"
outputs:
  - "success": "boolean"
  - "target_path": "string; ruta del TODO en docs/todos/pending/"
  - "message": "string; resultado (nuevo | idempotente)"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: materialize-kaizen-alert-doc

## 1. Propósito

Acción canónica del Agente **Cúmulo** ante `Kaizen_Alert_Required`. Materializa la cicatriz Kaizen `PENDING_AUDIT_DOC_{hash8}.md` en `docs/todos/pending/`. Plantilla **polimórfica** según `alert_kind`: `doc_parity` → checklist DIA; `event-bus-audit` (u otro técnico) → checklist de bus, sin prosa de fuga documental.

## 2. Orquestación

### Paso 1 — Validación

Campos obligatorios: `review_id`, `alert_justification`, `implicated_files` (array no vacío).

### Paso 2 — Idempotencia

`hash8 = SHA256(review_id + sorted(implicated_files))[:8]`. Si el TODO ya existe, `success: true` con mensaje idempotente.

### Paso 3 — Persistencia

Markdown con tabla `review_id`, `alert_justification`, `implicated_files`, `persist_ref` (si presente) y checklist según `alert_kind` (DIA o bus). Idempotencia abierta: casilla DIA o de bus sin marcar + misma huella `alert_kind`+files.

### Paso 4 — Cierre (stdout)

Envelope con `success`, `target_path`, `message`.

## 3. Límites

* No altera `delivery_state` de la aduana emisora.
* No referencia artefactos efímeros del sensor (`.tmp/audit-doc-parity-*`).
