---
feature_name: kaizen-capsula-imap-triaje
created: "2026-08-19"
process: feature
branch_name: feat/kaizen-capsula-imap-triaje
persist_ref: docs/features/kaizen-capsula-imap-triaje
uuid: "9c25bb52-57a4-4ede-be43-41388a7576b2"
status: implementing
agent: tekton
document_id: PBI-KAIZEN-CAPSULA-IMAP-TRIAJE
execution_id: "14fff213-bcee-4c26-ad17-53e5e585979b"
items:
  - T1-guante
  - T2-identidad-triaged
  - T3-fanout-telegram
  - T4-evento-retorno
  - T5-ingest
  - T6-wui
---

# Implementación — kaizen-capsula-imap-triaje

| T | Touchpoint | Vía | UUID / sello |
|---|------------|-----|----------------|
| T1 | `SddIA/daemons/email-watcher/src/main.rs` | in-ciclo | aislamiento UID; `--once` JSON-io |
| T2 | `email-triaged.md` v1.1.0 + `email_triage.rs` | quirúrgico UUID-preserve | `6a4b0e9a-…` intacto; OPTIONAL `from`,`subject` |
| T3 | `route_domain_core.rs` + `event-domain-subscriptions.json` | in-ciclo | `Email_Triaged` → `send-telegram-notification` |
| T4 | `email-quick-action-requested` | entity-manager create | `43d84426-7bb6-4179-91a0-aee13581bff6` · `Domain_Entity_Created` `f8f50b5c-…` |
| T5 | `email-quick-action-ingest` | entity-manager create + handler nativo | `e11c4348-29b5-45cd-bac8-f33f40e18a12` · `Domain_Entity_Created` `2d4abf52-…` |
| T6 | `kalma2-bridge` + `interfaces/kalma2/` | in-ciclo | `GET /api/email-inbox` · `POST /api/email-quick-action` |

## Notas

- T2 no usó `entity-manager` update (forge regenera UUID).
- `telegram-watcher` intocado.
- IMAP STORE / SMTP fuera.
