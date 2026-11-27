---
uuid: "8b179e84-128a-4214-913a-94bdec2b3246"
name: "user-preference-change-requested"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "User_Preference_Change_Requested"
context: "ecosystem-evolution"
capabilities:
  - "user_preference_change_requested"
hash_signature: "sha256:f5844cfea0b38c29f1927fad80afd21e96fca883169ba40cf44c40eef6c9d0b5"
---

# Event: User_Preference_Change_Requested

Solicitud de propuesta, activación, revocación o purga de preferencia del usuario. Sin cuerpos de mensaje.

## Payload ECST

### REQUIRED

- `operation` — `propose` | `activate` | `revoke` | `purge` | `ignore`
- `channel` — canal origen (`kalma2`, `telegram`, …)

### OPTIONAL

- `utterance_ref` — referencia corta / hash; no texto largo
- `subject_kind`, `subject_key`, `subject_hint`
- `predicate`, `predicate_hint`
- `scope_type`, `scope_id`
- `value`, `priority_level`
- `preference_id` — requerido para `purge`
- `source_event_id`

### FORBIDDEN

- `body`, `snippet`, `raw_email`, utterance completa

## Emisores autorizados

- `kalma2-bridge`
- acción `emit-user-preference-change-requested`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → `User_Preference_Change_Requested`.

## Propósito (complemento)

Solicitud de propuesta, activación, revocación o purga de preferencia del usuario. Sin cuerpos de mensaje.
