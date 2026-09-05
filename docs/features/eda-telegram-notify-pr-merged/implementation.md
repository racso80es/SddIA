---
feature_name: eda-telegram-notify-pr-merged
created: "2026-09-05"
process: feature
items:
  - json-subscriptions
  - composer-and-tests
  - entity-manager-event
  - evolution-register
document_id: PBI-EDA-TELEGRAM-NOTIFY-PR-MERGED
execution_id: "fccb9d32-8996-4594-8293-71c27926a017"
---

# Implementación — eda-telegram-notify-pr-merged

| Item | Path | Nota |
| :--- | :--- | :--- |
| CA1 | `SddIA/core/event-domain-subscriptions.json` + `event-subscriptions.json` | Argos → `send-telegram-notification`. Intent sin anomalías. IOTA intacto. |
| CA2–CA4 | `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | Rama compositor; `target_branch` desde payload; tests `telegram_message_for_pr_merged_*`. Cero Telegram en `accept_pr.rs`. |
| CA6 | `entity-manager` `46438c47-a671-4b72-933d-bf5991093bd6` | uuid `cfb8ce66-…` inmutable. hash `sha256:6cd7add82268b9d992d8ddc780f11f08f2c25b21f7eb0c09b58f82f80dae1bb5`. Versión Clase 1.0.0. Coverage alineado. |
| Evolution | `c11b4325-3daa-4418-aa87-54438a3b165d` | `EVOL_OK` alta. |
