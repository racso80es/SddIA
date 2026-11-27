---
uuid: "fc8f0f9c-7152-4a96-9111-dc6cd0a71d93"
name: "manual-task-requested"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Manual_Task_Requested"
context: "ecosystem-evolution"
capabilities:
  - "manual_task_requested"
hash_signature: "sha256:04d423f3b38464c5495d719595aa92fb355664560b2e912737dafd837885e901"
---

# Event: Manual_Task_Requested

Voluntad capturada desde canal externo (Telegram) e inyectada al bus local.

## Payload ECST

### REQUIRED
- `task_text`
- `source`
- `raw_text`

### OPTIONAL
- *(ninguno)*

### FORBIDDEN
- *(ninguno)*

## Emisores autorizados

- Proceso **`telegram-gateway`**

## Auditoría DLT

`dlt_audit_scope: local_only`.

## Suscripciones

Sin suscriptores obligatorios en MVP; disponible para coreografía Ola C futura.

## Propósito (complemento)

Voluntad capturada desde canal externo (Telegram) e inyectada al bus local.
