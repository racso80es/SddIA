---
uuid: "6a4b0e9a-42e1-425c-8a16-9344eae4f246"
name: "email-triaged"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Email_Triaged"
context: "ecosystem-evolution"
capabilities:
  - "email_triaged"
hash_signature: "sha256:fe53e3926249d33108a05da94ef9873cd916b5a39d620dd7a240c594d9c83540"
---

# Event: Email_Triaged

Veredicto de triaje de correo. Porta decision_path y thermodynamic_cost para verificar el peaje (G5). Emisor: email-triage-gateway.

## Payload ECST

### REQUIRED
- `message_uid`
- `verdict`
- `decision_path`
- `thermodynamic_cost`

### OPTIONAL
- `matched_rule`
- `agenda_entry_id`

### FORBIDDEN
- `body`
- `snippet`

## Emisores autorizados

- `email-triage-gateway`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Email_Triaged`.
