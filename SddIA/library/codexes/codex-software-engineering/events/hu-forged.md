---
uuid: "87a743fa-04f1-4dd0-8bfe-350d8ec899cb"
name: "hu-forged"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "HU_Forged"
context: "knowledge-management"
hash_signature: "sha256:518313f4769269325ebc6913c8fddd30464507e267b1279ad869123a81adb05d"
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
