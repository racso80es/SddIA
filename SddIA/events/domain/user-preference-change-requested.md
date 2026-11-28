---
uuid: "78a7d612-42ef-48e0-ac2b-29f43256a753"
name: "user-preference-change-requested"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "User_Preference_Change_Requested"
context: "ecosystem-evolution"
capabilities:
  - "user_preference_change_requested"
hash_signature: "sha256:620b2f904a0b829e36f411270b5989d3aa000ad37c82c1cedb2fd7f5f078248b"
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
