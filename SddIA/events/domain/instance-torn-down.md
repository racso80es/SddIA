---
uuid: "4ea892df-12a1-4c40-9b93-f2f3e559ea21"
name: "instance-torn-down"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Instance_Torn_Down"
context: "ecosystem-evolution"
capabilities:
  - "instance_torn_down"
hash_signature: "sha256:b162d52061bb7f946f2edca657026f8fee1c3b5c9a8a6d5b772e89c383e3672a"
---

# Event: Instance_Torn_Down

Instancia destruida por installer teardown --force. Payload: root, esc, profile.

## Payload ECST

### REQUIRED
- `root`
- `esc`
- `profile`

### OPTIONAL
- `manifest_created_at`

### FORBIDDEN
- `entity_id`
- `review_id`

## Emisores autorizados

- `tekton`

## Auditoría DLT

`dlt_audit_scope: local_only`.

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Instance_Torn_Down`.
