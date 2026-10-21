---
uuid: "a6b7c8d9-e0f1-4a2b-c3d4-e5f6a7b8c9d0"
name: "manual-task-requested"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Manual_Task_Requested"
context: "ecosystem-evolution"
capabilities:
  - "manual_task_requested"
hash_signature: "sha256:ca8a097dab1f3455f42645bccd3428acb38b8c1abb88855335be0d31fa4c8c40"
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
