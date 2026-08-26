---
feature_name: email-watcher-imap-account-watermark
created: "2026-08-26"
process: bug-fix
base: main
branch: fix/email-watcher-imap-account-watermark
uuid: 16239778-a5bc-4a55-8996-9301e51a6176
document_id: PBI-FIX-EMAIL-WATCHER-IMAP-ACCOUNT-WATERMARK
friction_ids:
  - F-IMAP-WATERMARK-STALE
  - F-IMAP-ACCOUNT-CHANGE-SILENT
incident_ref: "Auditoría IMAP forja 2026-08-26 — UID 5799 no detectado tras cambio de bóveda"
---

# Spec — email-watcher watermark obsoleto al cambiar cuenta IMAP

## Problema

Tras cambiar `SDDIA_EMAIL_IMAP_USER` en bóveda, el state conservaba `last_uid` del buzón anterior. Poll incremental (`uid > last_uid`) devolvía cero candidatos sin error visible.

## Solución

1. Persistir `imap_identity_sha256` = SHA-256 de `{host}|{port}|{mailbox}|{user}` normalizado.
2. Ante mismatch de identidad → bootstrap (`last := 0`) + log stderr.
3. Heurística defensiva: si `last_uid` > max UID del mailbox (misma identidad) → bootstrap + log.
4. State legado sin campo: no bootstrap masivo; persistir identidad en primer poll.

## Touchpoints

- `SddIA/daemons/email-watcher/src/main.rs`
- `SddIA/daemons/email-watcher/Cargo.toml` (`sha2`)
- `SddIA/daemons/email-watcher.md`, `start-sddia.md`
