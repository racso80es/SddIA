---
feature_name: telegram-fallback-responder
created: "2026-06-11"
process: feature
phases:
  - "A — Evento TelegramMessage_Received"
  - "B — Proceso + handler core"
  - "C — Fan-out gateway + suscripción EDA"
  - "D — Tests, coverage, cierre documental"
branch_name: feat/telegram-fallback-responder
persist_ref: docs/features/telegram-fallback-responder
---

# Plan — Telegram Fallback Responder

| Paso | Actividad | Touchpoints | Gate |
|------|-----------|-------------|------|
| T1 | Evento ECST | `telegram-message-received.md`, `events/domain/index.md` | AC1 |
| T2 | Proceso + core | `telegram-fallback-responder.md`, `telegram_fallback_responder_core.py`, `process/index.md` | AC3 |
| T3 | Gateway fan-out | `telegram_gateway_core.py`, `transmute.py` | AC4 |
| T4 | Suscripción + route | `event-domain-subscriptions.json`, `route_domain_event_core.py`, `execute_process_capsules.py` | AC2 |
| T5 | Tests + EDA | `test_telegram_fallback_responder.py`, `eda-coverage.json` | AC5, AC6 |
| T6 | Cierre | `validacion.md`, PBI → `done/` | APTO |
