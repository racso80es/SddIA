---
feature_name: telegram-fallback-responder
branch: feat/telegram-fallback-responder
global: APTO
pbi_archived: true
created: "2026-06-11"
process: feature
checks:
  AC1_proceso_forjado: "APTO — SddIA/process/telegram-fallback-responder.md"
  AC2_suscripcion_json: "APTO — event-domain-subscriptions.json"
  AC3_filtro_c: "APTO — test_telegram_fallback_responder"
  AC4_fan_out_gateway: "APTO — telegram_gateway_core dual emit"
  AC5_tests: "APTO — 10 tests telegram (gateway + fallback)"
  AC6_eda_scan: "APTO — orphan_count 0"
  AC7_watcher_inmutable: "APTO — sin diffs watcher .py/.bat/.sh"
git_changes:
  - SddIA/process/telegram-fallback-responder.md
  - SddIA/process/index.md
  - SddIA/events/domain/telegram-message-received.md
  - SddIA/events/domain/index.md
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/core/eda-coverage.json
  - SddIA/scripts/qa/telegram_fallback_responder_core.py
  - SddIA/scripts/qa/telegram_gateway_core.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/scripts/qa/route_domain_event_core.py
  - SddIA/scripts/qa/test_telegram_fallback_responder.py
  - SddIA/scripts/tools/telegram-gateway/transmute.py
  - docs/features/telegram-fallback-responder/
  - docs/todos/done/PBI-TG-001- Implementación del Suscriptor de Triaje Inverso (Telegram Fallback Responder).md
---

# Validación — Telegram Fallback Responder

Argos laboratorio: entrega **APTO** en rama `feat/telegram-fallback-responder`. PBI archivado en `docs/todos/done/`. Smoke Telegram real con bóveda local queda como verificación del operador.
