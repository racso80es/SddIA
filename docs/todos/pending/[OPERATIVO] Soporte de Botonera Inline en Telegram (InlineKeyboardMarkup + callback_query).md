---
document_id: PBI-TELEGRAM-INLINE-KEYBOARD
title: "[OPERATIVO] Soporte de Botonera Inline en Telegram (InlineKeyboardMarkup + callback_query)"
format: markdown
version: "1.0.0"
created: "2026-09-06"
status: "propuesta"
refinement_status: unrefined
priority: media
type: operativo
process: feature
dispatch: false
suggested_branch: feat/telegram-inline-keyboard
persist_ref_suggested: docs/features/telegram-inline-keyboard
depends_on: []
related:
  - SddIA/tools/send-telegram-notification.md
  - SddIA/tools/send-telegram-notification/src/main.rs
  - SddIA/daemons/telegram-watcher/src/main.rs
  - SddIA/daemons/telegram-watcher.md
  - SddIA/events/domain/user-preference-change-requested.md
  - SddIA/actions/emit-user-preference-change-requested.md
spawned_by: PBI-EMAIL-TRIAGE-HEURISTIC
---

### [OPERATIVO] Soporte de Botonera Inline en Telegram (InlineKeyboardMarkup + callback_query)

#### Origen

Dependencia bloqueante identificada durante el refinamiento de `PBI-EMAIL-TRIAGE-HEURISTIC`. El ecosistema SddIA carece de infraestructura para enviar mensajes con botones inline de Telegram y procesar las pulsaciones del usuario.

#### Alcance

**Componente 1 — Eferente (`send-telegram-notification`):**
- Extender el contrato de entrada de la cápsula para aceptar un parámetro opcional `reply_markup` (JSON con estructura `InlineKeyboardMarkup` de la API de Telegram).
- Incorporar `reply_markup` en la llamada a `/sendMessage` cuando esté presente.
- La Táctica del Refugio (reintento sin `parse_mode` ante error 400) debe preservarse; evaluar si `reply_markup` se mantiene o se descarta en el reintento degradado.
- Tests: mock con `reply_markup` presente y ausente.

**Componente 2 — Aferente (`telegram-watcher`):**
- Extender el bucle de long-poll para interceptar `callback_query` (además de `message` / `edited_message`).
- Extraer `callback_data` + `chat_id` + `message_id` del callback.
- Emitir un evento `User_Preference_Change_Requested` al bus EDA con la operación codificada en `callback_data`, o delegar al proceso `telegram-gateway` con un input extendido.
- Responder al callback con `answerCallbackQuery` para cerrar el spinner del botón en el cliente.

#### Restricciones

- No mutar la semántica existente de `send-telegram-notification` cuando `reply_markup` está ausente.
- No mutar el flujo de `telegram-watcher` para mensajes de texto convencionales.
- `callback_data` debe ser un payload compacto (máx 64 bytes por restricción de Telegram API).

#### Criterios de Aceptación (borrador)

- [ ] `send-telegram-notification` acepta `reply_markup` opcional y lo envía correctamente a la API de Telegram.
- [ ] `telegram-watcher` intercepta `callback_query` y emite evento ECST válido al bus.
- [ ] `telegram-watcher` responde con `answerCallbackQuery` tras procesar el callback.
- [ ] Tests unitarios para ambos componentes con y sin la nueva funcionalidad.
