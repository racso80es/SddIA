---
uuid: "b7c8d9e0-f1a2-4b3c-d4e5-f6a7b8c9d0e1"
name: "kaizen-idea-captured"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Kaizen_Idea_Captured"
context: "ecosystem-evolution"
capabilities:
  - "kaizen_idea_captured"
hash_signature: "sha256:cd7a16d5a59d3821878f594a61c5f96f9b2527bbc0d19c36c4766fbcec463fb6"
---

# Event: Kaizen_Idea_Captured

Idea Kaizen capturada vía patrón `TODO:` o `IDEA:` en Telegram.

## Payload ECST

### REQUIRED
- `idea_text`
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

Sin suscriptores obligatorios en MVP.
