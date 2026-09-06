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
hash_signature: "sha256:c5a6a60c3a66b6cc2081e6bd00402df83e8aa54d25fd60d633733102ee4cde8b"
---

# Event: Email_Triaged

Veredicto de triaje de correo. Porta decision_path y thermodynamic_cost para verificar el peaje (G5). `decision_path` ∈ {`deterministic`, `llm`, `preference`} según quien **cerró**. Emisor: email-triage-gateway. v1.1.0 añade identidad táctica `from`/`subject` (nunca `snippet`/`body`) para fan-out humano.

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
