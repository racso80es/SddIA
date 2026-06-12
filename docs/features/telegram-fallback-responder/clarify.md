---
feature_name: telegram-fallback-responder
created: "2026-06-11"
purpose: Estabilización de requisitos — PBI-TG-001
process: feature
---

# Clarificaciones — Telegram Fallback Responder

## Decisiones pendientes (Dedalo / spec.md)

| ID | Tema | Estado | Notas |
|----|------|--------|-------|
| **D1** | Evento disparador | **Cerrado** | Forjado `TelegramMessage_Received`; fan-out en `telegram_gateway_core` sin tocar watcher |
| **D2** | Payload mínimo | **Propuesto** | `{ "text": string, "chat_id": string }` alineado con Capa 0 existente |
| **D3** | Palabras reservadas Filtro C | **Propuesto** | Prefijos `/`, `!`; tokens `TODO:`, `IDEA:` (competencia gateway Kaizen) |
| **D4** | Esquema suscripción | **Propuesto** | PBI usa notación simplificada; SSOT real es clave de evento → array con `{ agent, process, intent }` en `event-domain-subscriptions.json` |
| **D5** | Agente suscriptor | **Propuesto** | `agent: tekton` o `agent: cumulo` según patrón de procesos domain; validar con `eda-coverage.json` |

## Restricciones heredadas (no negociables)

- **Ceguera espacial del watcher:** cero diffs en `SddIA/scripts/daemons/telegram-watcher.*`.
- **Tool eferente ciega:** `send-telegram-notification` no conoce el bus ni el motivo del mensaje.
- **Prompt Mayeuta literal:** el bloque `[HARD OVERRIDE] Has recibido este estímulo externo…` del PBI debe reproducirse sin parafrasear en la definición del proceso.

## Relación con activos existentes

```text
telegram-watcher (Capa 0, inmutable)
       │
       ▼
telegram-gateway ──► Manual_Task_Requested / Kaizen_Idea_Captured
       │
       ? ──► TelegramMessage_Received (a definir)
       │
       ▼
telegram-fallback-responder ──► Mayeuta ──► send-telegram-notification
```

## AC0 — Arranque feature

| AC | Descripción | Estado |
|----|-------------|--------|
| AC0.1 | Rama `feat/telegram-fallback-responder` creada desde `main` | ✅ |
| AC0.2 | `persist_ref` inicializado con `_init-feature.json` | ✅ |
| AC0.3 | `objectives.md` y `clarify.md` con trazabilidad PBI-TG-001 | ✅ |

## Siguiente fase

**Dedalo** debe resolver **D1** en `spec.md` y proponer blueprint en `plan.md` antes de forja Tekton.
