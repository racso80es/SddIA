---
uuid: "e4f5a6b7-c8d9-4e0f-a1b2-c3d4e5f6a7b8"
name: "send-telegram-notification"
version: "1.0.0"
contract: "tools-contract v1.2.0"
contract_ref: "SddIA/tools/tools-contract.md"
domain_origin: "SddIA"
context: "ecosystem-evolution"
capabilities:
  - "send-telegram-notification"
  - "telegram-send-message"
  - "capsule-json-io"
implementation_path_ref: "SddIA/tools/send-telegram-notification"
---

# send-telegram-notification

Tool inerte de salida hacia Telegram (`/sendMessage`). **Ceguera espacial:** no conoce el motivo del aviso ni el bus EDA.

## Táctica del Refugio (obligatoria)

Si Telegram rechaza el envío por error de **parsing** (`400`), la cápsula reintenta **una vez** con el mismo `message` y **sin** `parse_mode`. La entrega táctica al dispositivo prevalece sobre el formato.

## Interface

stdin JSON:

| Campo | Obligatorio | Default |
|-------|:-----------:|---------|
| `message` | Sí | — |
| `parse_mode` | No | `MarkdownV2` (`null` = plain) |

Salida: envelope con `success`, `message_id`, `attempt`, `degraded_plain_fallback`, `parse_mode_requested`, `error`.

## Entorno

- `TELEGRAM_BOT_TOKEN`
- `TELEGRAM_ALLOWED_CHAT_ID`

Cargados vía jerarquía de bóvedas (`env_loader`).
