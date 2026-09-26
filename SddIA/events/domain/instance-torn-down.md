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
hash_signature: "sha256:8c34637e1ce5adde8935766d835ae7098618d791aeb8c8d1b02fbfb2d3512e37"
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

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Instance_Torn_Down`.
