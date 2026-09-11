---
feature_name: telegram-inline-keyboard
created: "2026-09-11"
process: feature
base: main
scope: telegram-eferente-aferente
---

# Spec — telegram-inline-keyboard

## Eferente

stdin `reply_markup`: objeto JSON con `inline_keyboard`. Serializar a string Bot API. Ausente/null → no campo.

400 + parse_mode Some → un retry sin parse_mode, mismo `reply_markup`.

## Aferente

`chat_id` de callback: `callback_query.message.chat.id`.

Gateway inputs XOR. Evento domain `TelegramCallback_Received` payload `callback_data`, `chat_id`, `source`; opt `message_id`.

Suscripción: array vacío.

## Tests

Tool send markup; extractores watcher; handler XOR y tipo de evento.
