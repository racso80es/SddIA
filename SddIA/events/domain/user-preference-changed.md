---
uuid: "52979b2f-d360-4ff1-ad06-ede046f870d3"
name: "user-preference-changed"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "User_Preference_Changed"
context: "ecosystem-evolution"
capabilities:
  - "user_preference_changed"
hash_signature: "sha256:2629e0fdbbfae3299ca2f7c289c6106f40476c9765c820c289f884a8eeaf6639"
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
