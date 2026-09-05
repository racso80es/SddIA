---
feature_name: eda-telegram-notify-pr-merged
created: "2026-09-05"
process: feature
items:
  - json-subscriptions
  - composer-and-tests
  - entity-manager-event
  - evolution-register
  - ola2-humanized-action
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

## Ola 2

| Item | Path | Nota |
| :--- | :--- | :--- |
| CA1 | JSON domain + paridad | `action: notify-humanized-pr-merged`; sin `tool: send-telegram-notification` bajo `PullRequest_Merged`. IOTA intacto. |
| CA2–CA4 / CA7 | `notify_humanized_pr_merged.rs` + despacho `run_from_event` | Prompt ECST + kernel anti-conjetura; fail-soft Gemini; estático reutilizado; un envío. Tests `notify_humanized_*` + `pull_request_merged_subscription_*`. |
| CA6 | EM action `f6017b3e-…` create / `0d03547d-…` update | uuid `1cd7bd40-b72f-4114-ac44-68b912774aa6`. hash `sha256:126a051b2b326109a93a20f2053692117445d06996637bcd2fa99b810e4fa902`. |
| Clase evento | EM `bab599ae-…` replacements | uuid `cfb8ce66-…` inmutable. hash `sha256:e82cf28dd23db23bafa5a860d46ca61ea431a12bbdd27712e0d49bf4e6dd4c20`. Sello idempotente `3027b63d-…`. |
| Evolution | `cef0d9ee-f0d0-4ba2-b3ef-28dd47a6d0d4` | `EVOL_OK` modificacion. |
