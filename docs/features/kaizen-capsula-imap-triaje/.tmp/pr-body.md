## Summary
- Guante `email-watcher`: aislamiento por UID y envelope `--once` (`capsule-json-io`). Ceguera/RO intactas.
- Fan-out humano de `Email_Triaged` (`verdict=actionable`) vía `send-telegram-notification`; ruido silenciado. Sin clase `Actionable_Email_Detected`.
- WUI Kalma2: inbox + acciones rápidas → `Email_Quick_Action_Requested` → `email-quick-action-ingest` (proof; sin IMAP STORE).

## Test plan
- [x] cargo test -p email-watcher once_envelope
- [x] cargo test -p execute-process email_triaged_ / emit_triaged_copies / persist_archive / gate_skips
- [x] cargo test -p kalma2-bridge email_inbox / email_routes
- [x] validacion.md APTO + pbi_archived
- [ ] Lab IMAP/Telegram vivo (diferido)