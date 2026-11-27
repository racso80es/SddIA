---
uuid: "edb69685-0fa7-41f3-b898-387f9447de6e"
name: "user-preference-changed"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "User_Preference_Changed"
context: "ecosystem-evolution"
capabilities:
  - "user_preference_changed"
hash_signature: "sha256:a77214d45fcb54279561dea6ef21531dfae16acdcc6b5edc58b14e622c5c1061"
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

## Propósito (complemento)

Notificación de revisión persistida. Metadatos no sensibles; prohibido `value`.
