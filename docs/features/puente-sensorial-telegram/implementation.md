---
feature_name: puente-sensorial-telegram
created: "2026-05-29"
process: feature
items:
  - id: T1
    touchpoint: SddIA/tools/send-telegram-notification.md, scripts/tools/send-telegram-notification/
    proposal: Tool ciega + Táctica del Refugio (2 POST max)
  - id: T2
    touchpoint: SddIA/events/domain/manual-task-requested.md, kaizen-idea-captured.md
    proposal: Clases ECST dominio ingestión Telegram
  - id: T3
    touchpoint: SddIA/process/telegram-gateway.md, telegram_gateway_core.py, execute_process_capsules.py
    proposal: Proceso transmutación → .events/domain/
  - id: T4
    touchpoint: SddIA/scripts/daemons/telegram-watcher.py, .gitignore
    proposal: Capa 0 long polling + filtro chat_id
  - id: T5
    touchpoint: event-subscriptions.json, event-domain-subscriptions.json, telegram_notify_core.py, route_domain_event_core.py
    proposal: Fan-out PR/Fracture → send-telegram-notification
  - id: T6
    touchpoint: eda-coverage.json, test_*.py
    proposal: Coverage + QA AC7
---

# Implementación — Puente Sensorial Telegram

| Paso | Archivos | Cambio |
|------|----------|--------|
| T1 | `telegram_api.py`, `main.py` | Pipeline MarkdownV2 → refugio plain; envelope `degraded_plain_fallback` |
| T2 | `manual-task-requested.md`, `kaizen-idea-captured.md`, `domain/index.md` | +2 Clases ECST |
| T3 | `telegram-gateway.md`, `telegram_gateway_core.py`, `run_process` branch | Regex TODO / manual task |
| T4 | `telegram-watcher.py` | `getUpdates`, state `last_update_id`, `--once` / `--dry-run` |
| T5 | Suscripciones + `route_domain_event_core` | Handler tool `send-telegram-notification` |
| T6 | `eda-coverage.json`, tests | `orphan_count: 0` |

Módulos nuevos: `telegram_api`, `telegram_gateway_core`, `telegram_notify_core`.
