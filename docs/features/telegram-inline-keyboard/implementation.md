---
feature_name: telegram-inline-keyboard
created: "2026-09-11"
process: feature
items:
  - factory-tool-update
  - send-telegram-markup
  - watcher-callback
  - gateway-xor
  - em-event-tool-process-daemon
---

# Implementación — telegram-inline-keyboard

| Path | Cambio |
|------|--------|
| `forges/factory.rs` | tool update `markdown_body_replacements` |
| `send-telegram-notification/src` | `reply_markup`; Refugio conserva markup |
| `telegram-watcher/src` | `callback_query` + `answerCallbackQuery` |
| `telegram_gateway.rs` + tool | XOR text/callback → `TelegramCallback_Received` |
| `events/domain/telegram-callback-received.md` | EM create uuid `e39a1c83-51ab-4a7c-81cf-5e10334afdee` |
| `event-domain-subscriptions.json` | clave vacía |
