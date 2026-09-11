---
feature_name: telegram-inline-keyboard
created: "2026-09-11"
process: feature
branch_name: feat/telegram-inline-keyboard
persist_ref: docs/features/telegram-inline-keyboard
pbi_ref: docs/todos/pending/[OPERATIVO] Soporte de Botonera Inline en Telegram (InlineKeyboardMarkup + callback_query).md
execution_id: "9abaf0b6-34d4-4837-88b7-ee8db10b1a07"
---

# Objetivos — telegram-inline-keyboard

## Misión

Infraestructura Telegram: enviar `InlineKeyboardMarkup` y absorber `callback_query` hasta un ECST de callback. Cero mutación de preferencias.

## Alcance

- `send-telegram-notification`: `reply_markup` opcional; Refugio con markup.
- `telegram-watcher`: `callback_query` → `answerCallbackQuery` → gateway XOR.
- `telegram-gateway` + handler: emite `TelegramCallback_Received`.
- Forja EM de event/tool/process/daemon. Factory tool update.

## Fuera de alcance

`PBI-EMAIL-DIGEST-PREFERENCE-REPLY`. Fallback-responder. IMAP.

## Ley aplicada

- DA-2: genoma `.md` vía `entity-manager`.
- Git vía `skill:git-manager` / procesos oficiales.
- `features-documentation-pattern` v1.2.1.
