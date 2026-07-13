---
uuid: "a9b8c7d6-e5f4-4321-a987-6543210fedcb"
name: "kaizen-alert-required"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Kaizen_Alert_Required"
context: "quality-assurance"
capabilities:
  - "kaizen_alert_required"
  - "doc_parity_debt"
hash_signature: "sha256:pending-anchor-on-merge"
---

# Event: Kaizen_Alert_Required

Clase ECST para alerta Kaizen de **paridad documental (DIA)** detectada por la Aduana. Dispara materialización asíncrona de cicatriz `PENDING_AUDIT_DOC_*` vía **Cúmulo** — único suscriptor legítimo en v1.

## Payload ECST

### REQUIRED
- `review_id`
- `alert_justification`
- `implicated_files`

### OPTIONAL
- `persist_ref`
- `pr_branch`
- `alert_kind`
- `impacts_doc`

### FORBIDDEN
- `audit_file`
- `sensor_tmp_path`
- `nested_agent_orders`

## Emisores autorizados

- Proceso **`pull-request-review`** (cápsula post-sensor DIA `audit-doc-parity` cuando `alert_required: true`)
- Acción **`emit-kaizen-alert-required-event`** (opcional; laboratorio)

## Suscripciones (fan-out v1)

| Orden | Agente | Acción | Rol |
|-------|--------|--------|-----|
| 1 | **Cúmulo** | `materialize-kaizen-alert-doc` | Materializa TODO `PENDING_AUDIT_DOC_{hash8}.md` |

Ver `SddIA/core/event-subscriptions.json` → clave `Kaizen_Alert_Required`.
