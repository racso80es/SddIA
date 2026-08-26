---
document_id: PBI-FIX-EMAIL-WATCHER-IMAP-ACCOUNT-WATERMARK
uuid: "16239778-a5bc-4a55-8996-9301e51a6176"
title: "[FIX] email-watcher — watermark obsoleto al cambiar cuenta IMAP"
format: markdown
version: "1.0.0"
status: done
closed: "2026-08-26"
type: bug-fix
priority: alta
process: bug-fix
persist_ref: docs/fixes/email-watcher-imap-account-watermark
fix_ref: docs/fixes/email-watcher-imap-account-watermark
created: "2026-08-26"
updated: "2026-08-26"
pbi_archived: true
incident_ref: "Auditoría IMAP forja 2026-08-26 — correo UID 5799 no detectado tras cambio de bóveda"
friction_ids:
  - F-IMAP-WATERMARK-STALE
  - F-IMAP-ACCOUNT-CHANGE-SILENT
---

# [FIX] email-watcher — watermark obsoleto al cambiar cuenta IMAP

## Cierre

Implementado `imap_identity_sha256` + heurística ceiling en `email-watcher`. Tests 19/19. Ver `docs/fixes/email-watcher-imap-account-watermark/validacion.md`.

## Criterios de aceptación

- [x] Cambio de `SDDIA_EMAIL_IMAP_USER` → bootstrap automático en primer poll
- [x] Solo `SDDIA_EMAIL_IMAP_SECRET` → watermark preservado
- [x] State legado sin `imap_identity_sha256` → sin bootstrap masivo
- [x] `last_uid` > max UID mailbox → bootstrap + log
- [x] `cargo test -p email-watcher` verdes
- [x] `validacion.md` APTO, PBI en `done/`
