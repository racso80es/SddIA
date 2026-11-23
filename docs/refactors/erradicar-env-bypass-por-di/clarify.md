---
feature_name: erradicar-env-bypass-por-di
created: "2026-11-23"
process: refactorization
branch_name: refactor/erradicar-env-bypass-por-di
persist_ref: docs/refactors/erradicar-env-bypass-por-di
tracker_ref: AUT-116
---
# Clarificación — erradicar-env-bypass-por-di (`AUT-116`)

- Se conservan intactas las variables legítimas de bóveda `.env` (`TELEGRAM_BOT_TOKEN`, `TELEGRAM_ALLOWED_CHAT_ID`, `LINEAR_API_TOKEN`, `SDDIA_GEMINI_MODEL`, etc.).
- Migración progresiva por fronteras según D-3: frontera Telegram `notify_humanized_pr_merged` y despacho `route_domain_core` migradas a DI con 0 `SDDIA_LAB_SKIP_*`; resto de módulos legacy inventariados en `LEGACY_SKIP_PENDING_DI` (lista decreciente).
