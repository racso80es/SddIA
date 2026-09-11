---
feature_name: telegram-inline-keyboard
created: "2026-09-11"
process: feature
purpose: Filtro A PBI v1.1.0; markup eferente + callback XOR gateway
version_clarify: "1.0.0"
execution_id: "9abaf0b6-34d4-4837-88b7-ee8db10b1a07"
pbi_ref: docs/todos/pending/[OPERATIVO] Soporte de Botonera Inline en Telegram (InlineKeyboardMarkup + callback_query).md
document_id: PBI-TELEGRAM-INLINE-KEYBOARD
pbi_uuid: "67ab9395-a0d8-4cd7-8d0d-7ebf3d59377f"
pbi_version: "1.1.0"
---

# Clarificación — telegram-inline-keyboard

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skip archive/DCC. `execution_id` `9abaf0b6-34d4-4837-88b7-ee8db10b1a07`. Rama `feat/telegram-inline-keyboard`.

## Decisiones

1. Preferencias **no** salen del watcher. Clase nueva `TelegramCallback_Received`. Réplica = PBI hijo.
2. XOR `text` vs `callback_data` en `telegram-gateway`.
3. Refugio: quitar `parse_mode`; conservar `reply_markup`.
4. `callback_data` ≤ 64 bytes UTF-8 (tope Bot API).
5. Forja tool update: `markdown_body_replacements` (hueco de factory).
