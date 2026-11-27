---
uuid: "5bfa4f95-c95d-486c-8624-a4eea55abe60"
name: "telegram-message-received"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "TelegramMessage_Received"
context: "ecosystem-evolution"
capabilities:
  - "telegram_message_received"
hash_signature: "sha256:2153ba89df296ae1bf30d0a48f1e17d83cb44353d0d189e596509b92a5a4d326"
---

# Event: TelegramMessage_Received

Estímulo aferente crudo desde Telegram antes del triaje estructurado. Alimenta la red de seguridad sensorial (triaje inverso).

## Payload ECST

### REQUIRED
- `text`
- `chat_id`
- `source`

### OPTIONAL
- `raw_text`

### FORBIDDEN
- *(ninguno)*

## Emisores autorizados

- Proceso **`telegram-gateway`**

## Auditoría DLT

`dlt_audit_scope: local_only`.

## Suscripciones

- Proceso **`telegram-fallback-responder`** — triaje inverso para entropía no estructurada.

## Propósito (complemento)

Estímulo aferente crudo desde Telegram antes del triaje estructurado. Alimenta la red de seguridad sensorial (triaje inverso).
