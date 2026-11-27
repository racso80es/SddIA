---
uuid: "b2c3d4e5-f6a7-4890-b123-456789ab003"
name: "user-preference-changed"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "User_Preference_Changed"
context: "ecosystem-evolution"
capabilities:
  - "user_preference_changed"
hash_signature: "sha256:pending-forge"
---

# Event: User_Preference_Changed

Notificación de revisión persistida. Metadatos no sensibles; prohibido `value`.

## Payload ECST

### REQUIRED

- `preference_id`
- `revision_id`
- `operation`
- `scope_type`
- `status`

### OPTIONAL

- `predicate`
- `sensitivity`

### FORBIDDEN

- `value`, PII, cuerpos de mensaje

## Emisores autorizados

- proceso `user-preference-ingest`

## Suscripciones

MVP: ninguna (sin DLT).
