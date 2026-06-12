---
uuid: "e1f2a3b4-c5d6-47e8-f9a0-b1c2d3e4f5a6"
name: "telegram-message-received"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "TelegramMessage_Received"
context: "ecosystem-evolution"
capabilities:
  - "telegram_message_received"
hash_signature: "sha256:pending-anchor-on-merge"
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

## Suscripciones

- Proceso **`telegram-fallback-responder`** — triaje inverso para entropía no estructurada.
