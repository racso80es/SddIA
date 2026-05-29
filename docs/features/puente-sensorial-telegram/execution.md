---
feature_name: puente-sensorial-telegram
created: "2026-05-29"
process: feature
items_applied:
  - T1-tool-refugio
  - T2-eventos-domain
  - T3-telegram-gateway
  - T4-telegram-watcher
  - T5-suscripciones-notify
  - T6-tests-coverage
branch_name: feat/puente-sensorial-telegram
---

# Ejecución — Puente Sensorial Telegram

## Comandos verificados

```text
python -m unittest SddIA.scripts.qa.test_send_telegram_refugio SddIA.scripts.qa.test_telegram_gateway SddIA.scripts.qa.test_telegram_notify -v
python SddIA/scripts/qa/execute-process.py --process telegram-gateway --inputs "{\"text\":\"TODO: smoke lab\"}"
python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
```

## Resultados

| AC | Evidencia |
|----|-----------|
| AC4/AC5 | `telegram-gateway` → JSON en `.events/domain/` |
| AC7 | `test_send_telegram_refugio` — refugio plain tras 400 parsing |
| AC3 | Tool sin imports bus/subscriptions |
| EDA | `--scan` → `orphan_count: 0` |

## Operador (post-merge)

1. Configurar `.SddIA/.dev/.env` con `TELEGRAM_BOT_TOKEN` y `TELEGRAM_ALLOWED_CHAT_ID`.
2. Smoke real: `python SddIA/scripts/tools/send-telegram-notification/main.py` con JSON message.
3. Arrancar `python SddIA/scripts/daemons/telegram-watcher.py` (o `--once` en lab).

## Deuda explícita

- Contrato ED Centinela (kitchen) no implementado; watcher mínimo alineado al PBI.
- Consultas libres Telegram sin LLM (fuera MVP).
