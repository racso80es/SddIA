---
uuid: "c4e5f6a7-8b9c-4d0e-1f2a-3b4c5d6e7f80"
name: "hu-forged"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "HU_Forged"
context: "knowledge-management"
---

# Event: HU_Forged

Sello de HU materializada por `forge-hu` tras `create_issue` en el tracker del proyecto cliente.

## Payload ECST

### REQUIRED
- `tracker_ref`
- `project_slug`

### OPTIONAL
- `document_id`
- `source_process`

### FORBIDDEN
- `hash_signature`

## Emisores autorizados

- `forge-hu`

## Suscripciones

`tekton.tracker-stamp` — confirmación backlog, `tracker_column` y comentario idempotente (paridad `PBI_Forged`).
