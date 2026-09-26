---
uuid: "a71b0d84-9e7b-414e-b935-f054d9539508"
name: "instance-deployed"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Instance_Deployed"
context: "ecosystem-evolution"
capabilities:
  - "instance_deployed"
hash_signature: "sha256:082fbbd6a17e0dc23a2d04565c9a38ebbbdb80802987ceef4632c1edc45c7f88"
---

# Event: Instance_Deployed

Instancia materializada por installer deploy. Payload: root, esc, profile, manifest_created_at, verdict verify.

## Payload ECST

### REQUIRED
- `root`
- `esc`
- `profile`

### OPTIONAL
- `manifest_created_at`
- `verdict`

### FORBIDDEN
- `entity_id`
- `review_id`

## Emisores autorizados

- `tekton`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Instance_Deployed`.
