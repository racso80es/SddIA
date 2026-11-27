---
uuid: "ea906c5c-2bec-4bc9-865b-966a10fa8202"
name: "hu-forged"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "HU_Forged"
context: "knowledge-management"
hash_signature: "sha256:509592bc3897c788dfa6108365a4019d99b0ffd462a6d8f06ad07d0eb5079e54"
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

## Propósito (complemento)

Sello de HU materializada por `forge-hu` tras `create_issue` en el tracker del proyecto cliente.
