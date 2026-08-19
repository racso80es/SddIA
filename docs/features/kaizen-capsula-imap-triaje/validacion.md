---
feature_name: kaizen-capsula-imap-triaje
created: "2026-08-19"
process: feature
branch: feat/kaizen-capsula-imap-triaje
persist_ref: docs/features/kaizen-capsula-imap-triaje
pbi_ref: docs/todos/done/PBI-KAIZEN-CAPSULA-IMAP-TRIAJE.md
document_id: PBI-KAIZEN-CAPSULA-IMAP-TRIAJE
uuid: "9c25bb52-57a4-4ede-be43-41388a7576b2"
execution_id: "14fff213-bcee-4c26-ad17-53e5e585979b"
global: APTO
pbi_archived: true
checks:
  DOC_OBJECTIVES: APTO
  DOC_CLARIFY: APTO
  DOC_SPEC: APTO
  DOC_PLAN: APTO
  DOC_IMPLEMENTATION: APTO
  DOC_EXECUTION: APTO
  AC_A_GLOVE: APTO
  AC_B1_SILENCIO_RUIDO: APTO
  AC_B2_ELEVACION: APTO
  AC_B3_RETORNO: APTO
  O3_NO_DUALIDAD: APTO
  G4_CEGUERA: APTO
  PBI_ARCHIVED: APTO
  LAB_TELEGRAM_LIVE: DIFERIDO
  LAB_IMAP_LIVE: DIFERIDO
git_changes:
  - docs/features/kaizen-capsula-imap-triaje/
  - docs/todos/done/PBI-KAIZEN-CAPSULA-IMAP-TRIAJE.md
  - SddIA/daemons/email-watcher/src/main.rs
  - SddIA/events/domain/email-triaged.md
  - SddIA/events/domain/email-quick-action-requested.md
  - SddIA/events/domain/index.md
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/library/codexes/codex-kalma2-assistant/process/email-quick-action-ingest.md
  - SddIA/engine/execute-process/src/engine/handlers/email_triage.rs
  - SddIA/engine/execute-process/src/engine/handlers/email_quick_action.rs
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - interfaces/kalma2/
  - SddIA/evolution/fa0f00e4-20f1-4258-95a9-e4d753f71d71.md
---

# Validación — kaizen-capsula-imap-triaje

**Veredicto global: APTO** (unidad + genoma). Lab IMAP/Telegram vivo diferido.

| AC | Evidencia |
|----|-----------|
| **AC-A** | UID aislado; `--once` envelope `success`⇔`exitCode`; test `once_envelope_json_io_contract` |
| **AC-B1** | `verdict=noise` → `build_telegram_message_from_event` = None |
| **AC-B2** | actionable → poke `from/subject/uid`; inbox WUI filtra solo actionable |
| **AC-B3** | POST `/api/email-quick-action` → `Email_Quick_Action_Requested` → ingest proof; sin IMAP STORE |
| **O3** | Cero clase `Actionable_Email_Detected` |

PBI archivado en `docs/todos/done/` en esta rama.
