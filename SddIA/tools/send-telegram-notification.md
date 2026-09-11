---
capabilities:
- send-telegram-notification
- telegram-send-message
- capsule-json-io
context: ecosystem-evolution
contract: tools-contract v1.2.0
contract_ref: SddIA/tools/tools-contract.md
domain_origin: SddIA
implementation_path_ref: SddIA/tools/send-telegram-notification
name: send-telegram-notification
source_sha256: sha256:1097dec79cc8e73156a714b47dbb0529f7f676e2dd1da80c0da2b54d6a056c7e
uuid: e4f5a6b7-c8d9-4e0f-a1b2-c3d4e5f6a7b8
version: 1.0.0
hash_signature: "sha256:ee764b1cb22c42cf4132061ee3b91947f6180432ec74bc92e1f039fca303e923"
---

# send-telegram-notification

Tool inerte de salida hacia Telegram (`/sendMessage`). **Ceguera espacial:** no conoce el motivo del aviso ni el bus EDA.

## Táctica del Refugio (obligatoria)

Si Telegram rechaza el envío por error de **parsing** (`400`), la cápsula reintenta **una vez** con el mismo `message` y **sin** `parse_mode`. Si hay `reply_markup`, se conserva en el reintento. La entrega táctica al dispositivo prevalece sobre el formato.

## Interface

stdin JSON:

| Campo | Obligatorio | Default |
|-------|:-----------:|---------|
| `message` | Sí | — |
| `parse_mode` | No | ausente/`null` = plano (no enviar campo) |
| `reply_markup` | No | objeto `InlineKeyboardMarkup` (`inline_keyboard`); se serializa a JSON string Bot API |

Salida: envelope con `success`, `message_id`, `attempt`, `degraded_plain_fallback`, `parse_mode_requested`, `error`.

## Entorno

- `TELEGRAM_BOT_TOKEN`
- `TELEGRAM_ALLOWED_CHAT_ID`

Cargados vía jerarquía de bóvedas (`env_loader`).
