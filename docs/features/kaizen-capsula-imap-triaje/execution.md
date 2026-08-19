---
feature_name: kaizen-capsula-imap-triaje
created: "2026-08-19"
process: feature
branch_name: feat/kaizen-capsula-imap-triaje
persist_ref: docs/features/kaizen-capsula-imap-triaje
uuid: "9c25bb52-57a4-4ede-be43-41388a7576b2"
status: executed
agent: tekton
document_id: PBI-KAIZEN-CAPSULA-IMAP-TRIAJE
execution_id: "14fff213-bcee-4c26-ad17-53e5e585979b"
items_applied:
  - T1-guante
  - T2-identidad-triaged
  - T3-fanout-telegram
  - T4-evento-retorno
  - T5-ingest
  - T6-wui
---

# Ejecución — kaizen-capsula-imap-triaje

## CLI genoma

| Acción | Acuse |
|--------|--------|
| `entity-manager` create event `email-quick-action-requested` | `success` `exitCode:0` uuid `43d84426-…` event `f8f50b5c-…` |
| `entity-manager` create process `email-quick-action-ingest` | `success` `exitCode:0` uuid `e11c4348-…` event `2d4abf52-…` jurisdiction domain kalma2 |

## Tests (unidad)

| Crate | Test | Resultado |
|-------|------|-----------|
| email-watcher | `once_envelope_json_io_contract` | APTO |
| execute-process | `email_triaged_noise_yields_no_telegram` | APTO |
| execute-process | `email_triaged_actionable_builds_poke` | APTO |
| execute-process | `emit_triaged_copies_from_subject_not_snippet` | APTO |
| execute-process | `gate_skips_invalid_action` | APTO |
| execute-process | `persist_archive_writes_proof` | APTO |
| kalma2-bridge | `email_inbox_filters_actionable_only` | APTO |
| kalma2-bridge | `email_routes_exist_in_dispatch` | APTO |

## Lab vivo no ejecutado

Poke Telegram (`TELEGRAM_BOT_TOKEN`) e IMAP `--once` contra buzón real: no disparados en esta sesión. Contrato cubierto por unidad.

## G4 spot-check watcher

`execute-process` / IMAP STORE / lectura de `email-triage-matrix` no añadidos al crate del centinela.
