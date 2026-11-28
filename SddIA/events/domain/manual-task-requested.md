---
uuid: "8e3d6ca1-6fc0-46a8-a3dc-b69b1856f613"
name: "manual-task-requested"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Manual_Task_Requested"
context: "ecosystem-evolution"
capabilities:
  - "manual_task_requested"
hash_signature: "sha256:11ee237fcd58f44505fee9fc7766ecff9f687b1eb05e2e1f3233e574a6fb9b6d"
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
