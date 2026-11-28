---
uuid: "9ffe78df-728b-4d11-8663-e6aed291720c"
name: "telegram-message-received"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "TelegramMessage_Received"
context: "ecosystem-evolution"
capabilities:
  - "telegram_message_received"
hash_signature: "sha256:7bead5146e93811a95fd831efb3c87aa158167afb8e8d5c806c4badf1ab393cb"
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
