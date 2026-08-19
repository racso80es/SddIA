---
uuid: "6a4b0e9a-42e1-425c-8a16-9344eae4f246"
name: "email-triaged"
version: "1.1.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Email_Triaged"
context: "ecosystem-evolution"
capabilities:
  - "email_triaged"
hash_signature: "sha256:de33eb8edb7e135062bfe63a5edbc850f62f0e69e0649cec50d65258e9fdc620"
---

# Event: Email_Triaged

Veredicto de triaje de correo. Porta decision_path y thermodynamic_cost para verificar el peaje (G5). Emisor: email-triage-gateway. v1.1.0 añade identidad táctica `from`/`subject` (nunca `snippet`/`body`) para fan-out humano.

## Payload ECST

### REQUIRED
- `message_uid`
- `verdict`
- `decision_path`
- `thermodynamic_cost`

### OPTIONAL
- `matched_rule`
- `agenda_entry_id`
- `from`
- `subject`

### FORBIDDEN
- `body`
- `snippet`

## Emisores autorizados

- `email-triage-gateway`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Email_Triaged`.
