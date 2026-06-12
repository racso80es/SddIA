---
feature_name: telegram-fallback-responder
created: "2026-06-11"
process: feature
base: main
scope: telegram-fallback-responder, telegram-message-received, event-domain-subscriptions, telegram-gateway, send-telegram-notification
master_pbi_ref: docs/todos/done/PBI-TG-001- Implementación del Suscriptor de Triaje Inverso (Telegram Fallback Responder).md
master_pbi_id: PBI-TG-001
---

# Especificación técnica — Telegram Fallback Responder

## 1. Contexto

Extensión del Puente Sensorial Telegram: capa reactiva EDA que responde a entropía conversacional no estructurada sin mutar `telegram-watcher`.

```text
telegram-watcher (Capa 0, inmutable)
       │
       ▼
telegram-gateway ──► TelegramMessage_Received ──► route-domain-event
       │                        │
       │                        ▼
       │              telegram-fallback-responder
       │                        │
       ├──► Manual_Task_Requested / Kaizen_Idea_Captured
       │                        ▼
       │              send-telegram-notification
```

## 2. Evento `TelegramMessage_Received`

| Campo | Valor |
|-------|-------|
| Archivo | `SddIA/events/domain/telegram-message-received.md` |
| Emisor | `telegram-gateway` (fan-out en `telegram_gateway_core`) |
| Payload | `text`, `chat_id`, `source`, `raw_text` |

`chat_id` se resuelve desde `TELEGRAM_ALLOWED_CHAT_ID` en el emisor (sin modificar watcher).

## 3. Proceso `telegram-fallback-responder`

| Fase | Lógica |
|------|--------|
| Filtro C | Abort `success` si `/`, `!`, `TODO:`, `IDEA:` |
| Síntesis | Prompt Mayeuta literal (lab: plantilla determinista ≤2 líneas) |
| Materialización | `send-telegram-notification` (`parse_mode: null`) |

Handler: `telegram_fallback_responder_core.py` + registro en `execute_process_capsules.py`.

## 4. Suscripción EDA

`SddIA/core/event-domain-subscriptions.json`:

```json
"TelegramMessage_Received": [
  {
    "agent": "mayeuta",
    "process": "telegram-fallback-responder",
    "intent": "Red de seguridad sensorial: Triaje inverso para entropía no estructurada."
  }
]
```

Dispatch especial en `route_domain_event_core.py` (payload `text`/`chat_id`, sin `branch`).

## 5. Restricciones

- Prohibido modificar `telegram-watcher.py`, `.bat`, `.sh`.
- Tool eferente permanece ciega al bus.

## 6. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC1 | Proceso forjado en genoma |
| AC2 | Suscripción JSON válida |
| AC3 | Filtro C aborta comandos/reservados |
| AC4 | Fan-out `TelegramMessage_Received` en gateway |
| AC5 | Tests unitarios verdes |
| AC6 | `eda-coverage --scan` orphan 0 |
