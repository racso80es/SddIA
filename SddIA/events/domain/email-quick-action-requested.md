---
uuid: "43d84426-7bb6-4179-91a0-aee13581bff6"
name: "email-quick-action-requested"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Email_Quick_Action_Requested"
context: "ecosystem-evolution"
capabilities:
  - "email_quick_action_requested"
hash_signature: "sha256:3c53b27f57322fd5eb2cc46974c2d81076286f8666002a68e551b8bb7119cfaf"
---

# Event: Email_Quick_Action_Requested

Intención humana de acción rápida sobre un correo ya triado (archive|draft|delegate). No es detección. No porta cuerpo. No autoriza IMAP STORE.

## Payload ECST

### REQUIRED
- `message_uid`
- `action`

### OPTIONAL
- `source_event_id`
- `channel`

### FORBIDDEN
- `body`
- `snippet`
- `credentials`

## Emisores autorizados

- `kalma2-bridge`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Email_Quick_Action_Requested`.
