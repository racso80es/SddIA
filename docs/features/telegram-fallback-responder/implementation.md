---
feature_name: telegram-fallback-responder
created: "2026-06-11"
process: feature
items:
  - event-telegram-message-received
  - process-telegram-fallback-responder
  - core-telegram-fallback-responder
  - gateway-fan-out
  - eda-subscription
  - route-dispatch
---

# Implementación — Telegram Fallback Responder

## Touchpoints aplicados

| Item | Path |
|------|------|
| Evento | `SddIA/events/domain/telegram-message-received.md` |
| Proceso | `SddIA/process/telegram-fallback-responder.md` |
| Handler | `SddIA/scripts/qa/telegram_fallback_responder_core.py` |
| Gateway fan-out | `SddIA/scripts/qa/telegram_gateway_core.py`, `scripts/tools/telegram-gateway/transmute.py` |
| Suscripción | `SddIA/core/event-domain-subscriptions.json` |
| Route dispatch | `SddIA/scripts/qa/route_domain_event_core.py` |
| Intérprete | `SddIA/scripts/qa/execute_process_capsules.py` |
| Índices | `process/index.md`, `events/domain/index.md`, `eda-coverage.json` |
| Tests | `SddIA/scripts/qa/test_telegram_fallback_responder.py` |

## Sin modificar (PBI)

- `SddIA/scripts/daemons/telegram-watcher.py`
- `SddIA/scripts/daemons/telegram-watcher.bat`
- `SddIA/scripts/daemons/telegram-watcher.sh`
