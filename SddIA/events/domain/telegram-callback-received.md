---
uuid: "e39a1c83-51ab-4a7c-81cf-5e10334afdee"
name: "telegram-callback-received"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "TelegramCallback_Received"
context: "ecosystem-evolution"
capabilities:
  - "telegram_callback_received"
hash_signature: "sha256:7f95dc20a257f16f1559f289d4450f6b38c6bbbdeaf143270c51a8ee3ff16b30"
---

# Event: TelegramCallback_Received

Pulsación de botón inline Telegram. Sin utterance. No es preferencia.

## Payload ECST

### REQUIRED
- `callback_data`
- `chat_id`
- `source`

### OPTIONAL
- `message_id`

### FORBIDDEN
- `body`
- `snippet`
- `reply_markup`
- `utterance`

## Emisores autorizados

- `telegram-gateway`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `TelegramCallback_Received`.
