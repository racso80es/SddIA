---
document_id: PBI-TELEGRAM-INLINE-KEYBOARD
uuid: "67ab9395-a0d8-4cd7-8d0d-7ebf3d59377f"
title: "[OPERATIVO] Soporte de Botonera Inline en Telegram (InlineKeyboardMarkup + callback_query)"
format: markdown
version: "1.1.0"
created: "2026-09-06"
updated: "2026-09-11"
status: "propuesta"
refinement_status: refined
priority: media
type: operativo
process: feature
dispatch: false
suggested_branch: feat/telegram-inline-keyboard
persist_ref_suggested: docs/features/telegram-inline-keyboard
depends_on: []
blocks_on:
  - PBI-EMAIL-DIGEST-PREFERENCE-REPLY
spawned_by: PBI-EMAIL-TRIAGE-HEURISTIC
related:
  - SddIA/tools/send-telegram-notification.md
  - SddIA/tools/send-telegram-notification/src/main.rs
  - SddIA/daemons/telegram-watcher.md
  - SddIA/daemons/telegram-watcher/src/main.rs
  - SddIA/process/telegram-gateway.md
  - SddIA/tools/telegram-gateway.md
  - SddIA/engine/execute-process/src/engine/handlers/telegram_gateway.rs
  - SddIA/events/domain/telegram-message-received.md
  - SddIA/events/domain/user-preference-change-requested.md
  - SddIA/daemons/daemons-contract.md
  - docs/todos/pending/[OPERATIVO] Réplica del digest de ruido → preferencias.md
refinement_notes: >-
  Filtro A v1.1.0 (2026-09-11). Residual v1.0.0: (1) watcher no es emisor
  autorizado de User_Preference_Change_Requested; (2) Ceguera Lógica: el
  centinela no interpreta preferencias; (3) telegram-gateway exige `text` y
  transmuta a Kaizen/Manual_Task + TelegramMessage_Received — un callback no
  es utterance; (4) chat_id() solo lee message/edited_message, por eso el
  callback se descarta tras ACK de offset; (5) Táctica del Refugio quita
  parse_mode, no reply_markup; (6) send_form no transporta JSON de markup
  hoy; (7) contrato tool send-telegram default MarkdownV2 vs cápsula null;
  (8) daemons-contract prohíbe execute-process pero el watcher ya lo invoca
  (precedente; no reabrir).
---

### [OPERATIVO] Soporte de Botonera Inline en Telegram

#### 1. Origen

Dependencia de canal identificada en `PBI-EMAIL-TRIAGE-HEURISTIC`. El Core no envía `InlineKeyboardMarkup` ni procesa `callback_query`. Cierra infraestructura; **no** cierra el lazo de preferencias (`PBI-EMAIL-DIGEST-PREFERENCE-REPLY`).

#### 2. Fe de Erratas — Filtro A v1.1.0

| Elemento v1.0.0 | Inexactitud | SSOT | Corrección |
|---|---|---|---|
| Watcher emite `User_Preference_Change_Requested` | Emisores autorizados: `kalma2-bridge` y `emit-user-preference-change-requested` | `events/domain/user-preference-change-requested.md` | Prohibido. Réplica = PBI hijo. |
| Watcher interpreta `callback_data` como preferencia | `daemons-contract` §2: ceguera lógica | `telegram-watcher.md` jurisdiction | Watcher: filtrar chat, `answerCallbackQuery`, delegar al proceso `telegram-gateway`. Cero triaje de negocio. |
| Extender `telegram-gateway` «o» emitir preferencia | Gateway transmuta **texto** a `Kaizen_Idea_Captured` / `Manual_Task_Requested` y sella `TelegramMessage_Received` (dispara `telegram-fallback-responder`) | `handlers/telegram_gateway.rs` | Rama XOR: `callback_data` → evento **`TelegramCallback_Received`**. Sin sello sensorial de mensaje. Sin regex TODO/IDEA. |
| `chat_id` del update | Solo `message` / `edited_message` | `extract_text` / `chat_id` | Callback: `callback_query.message.chat.id`. Sin message → skip (no aborta lote). |
| Refugio descarta markup | 400 + `parse_mode` → reintento sin parse_mode | `send_telegram` | Conservar `reply_markup` en el reintento. |
| `reply_markup` en form | `try_send` solo `chat_id`, `text`, `parse_mode` | `main.rs` | Campo form `reply_markup` = JSON string Bot API. |
| Default MarkdownV2 | Contrato `.md` vs cápsula | digest v1.2.0 | Ausente/`null` → no enviar `parse_mode`. Markup independiente. |

#### 3. Circuito

```
Eferente:
  send-telegram-notification stdin
    message (req)
    parse_mode (opt; null = plano)
    reply_markup (opt; objeto InlineKeyboardMarkup)
  → POST /sendMessage
  → 400 + parse_mode Some → un reintento sin parse_mode, mismo text y mismo reply_markup

Aferente:
  getUpdates (tipos por defecto incluyen callback_query)
  ACK offset primero (invariante vigente)
  si callback_query:
       chat_id == TELEGRAM_ALLOWED_CHAT_ID
       callback_data no vacío, UTF-8 ≤ 64 bytes
       answerCallbackQuery(callback_query.id)  # HTTP Capa 0; spinner
       execute-process telegram-gateway
         inputs: { callback_data, chat_id, message_id }
  si message/edited_message con text: flujo vigente intacto
```

- `callback_data` y `text` son **XOR**. Ambos o ninguno → error de proceso, cero escritura de dominio.
- Fallo de `answerCallbackQuery` se registra en stderr; **igual** se invoca gateway (el ACK de update ya ocurrió; no reentrega).
- Fallo de gateway: log `rc`; no pánico.

#### 4. Clase ECST `TelegramCallback_Received`

- Familia `domain`. Empaque `SddIA/events/domain/telegram-callback-received.md`.
- Forja: `entity-manager` create clase `event`.
- REQUIRED: `callback_data`, `chat_id`, `source` (`telegram`).
- OPTIONAL: `message_id`.
- FORBIDDEN: `body`, `snippet`, utterance, addr en claro, `reply_markup`.
- Emisor autorizado: proceso/tool `telegram-gateway` (mismo patrón que `TelegramMessage_Received`).
- Suscripciones: clave en `event-domain-subscriptions.json` con array **vacío**. Fan-out = réplica. Prohibido alta a `user-preference-ingest` aquí.
- Cero `TelegramMessage_Received` en esta rama (evita fallback-responder).

#### 5. Contratos a actualizar (DA-2 vía EM)

1. `send-telegram-notification` — input `reply_markup` opcional; Refugio conserva markup. `hash_signature` via patch.
2. `telegram-gateway` (process 1.0.1 + tool) — inputs XOR `text` \| `callback_data` (+ `chat_id`, `message_id` opt).
3. `telegram-watcher` daemon — documentar callback + `answerCallbackQuery`; src bajo `execution_capsules.daemons`.
4. Handler nativo `telegram_gateway.rs` + cápsula `tools/telegram-gateway` alineados.
5. Forja tool: `run_tool_forge` update con `markdown_body_replacements` (hoy solo `hash_refresh_only` o create). Core, no genoma de negocio.

#### 6. Fuera de alcance

- Semántica de botones del digest (`priority:max` / mute).
- `user-preference-ingest`, `email-quick-action-ingest`, `telegram-fallback-responder`.
- Webhook Telegram; se mantiene long-poll.
- Mutar IMAP.

#### 7. Criterios de aceptación

- [ ] **CA-1:** `reply_markup` ausente → idéntico a hoy (form sin ese campo).
- [ ] **CA-2:** `reply_markup` objeto con `inline_keyboard` → se envía; mock/lab incluye el campo.
- [ ] **CA-3:** 400 con `parse_mode` Some → reintento sin `parse_mode`, **con** markup.
- [ ] **CA-4:** `callback_query` de chat permitido → `answerCallbackQuery` + gateway con `callback_data`; `text` ausente.
- [ ] **CA-5:** Se escribe `TelegramCallback_Received`; no `TelegramMessage_Received` / Kaizen / Manual_Task.
- [ ] **CA-6:** Mensaje de texto convencional: mismo evento que antes.
- [ ] **CA-7:** `callback_data` > 64 bytes o XOR violado → error; cero dominio.
- [ ] **CA-8:** Chat no permitido: skip (como texto).
- [ ] **CA-9:** Tests: tool send (markup sí/no); watcher extractores; handler gateway callback vs text.
- [ ] **CA-10:** Genoma `.md` de tool/process/event/daemon solo vía `entity-manager`.
