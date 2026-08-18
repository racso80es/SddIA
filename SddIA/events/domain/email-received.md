---
uuid: "574fe330-137f-4f3a-b72d-dba189c6c406"
name: "email-received"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Email_Received"
context: "peripheral-sensing"
capabilities:
  - "email_received"
hash_signature: "sha256:3467c04e21573d8d02f11529db3078c822d8aaebeceea5ff143310999ae8f0b9"
---

# Event: Email_Received

Estímulo aferente de correo. Payload ligero: snippet + body_ref; nunca el cuerpo íntegro. Emisor exclusivo: Centinela email-watcher.

## Payload ECST

### REQUIRED
- `message_uid`
- `mailbox`
- `from`
- `subject`
- `received_at`
- `snippet`

### OPTIONAL
- `body_ref`
- `list_headers`

### FORBIDDEN
- `body`
- `attachments`
- `credentials`
- `absolute_host_path`

## Emisores autorizados

- `email-watcher`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Email_Received`.
