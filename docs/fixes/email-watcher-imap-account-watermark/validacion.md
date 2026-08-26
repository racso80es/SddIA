---
feature_name: email-watcher-imap-account-watermark
created: "2026-08-26"
process: bug-fix
branch: fix/email-watcher-imap-account-watermark
persist_ref: docs/fixes/email-watcher-imap-account-watermark
global: APTO
pbi_archived: true
document_id: PBI-FIX-EMAIL-WATCHER-IMAP-ACCOUNT-WATERMARK
uuid: 16239778-a5bc-4a55-8996-9301e51a6176
checks:
  CA_identity_mismatch_bootstrap: APTO
  CA_secret_only_no_reset: APTO
  CA_legacy_no_mass_bootstrap: APTO
  CA_ceiling_heuristic: APTO
  CA_cargo_test: APTO
  CA_docs_operator: APTO
---

# Validación — email-watcher-imap-account-watermark

**global: APTO** — `pbi_archived: true`.

## Criterios

| ID | Estado | Evidencia |
|----|--------|-----------|
| Identity mismatch → bootstrap | APTO | `resolve_watermark` + test `resolve_watermark_identity_mismatch_resets` |
| Solo secret sin reset | APTO | identidad no incluye secret; mismo user/host → mismo hash |
| Legado sin campo | APTO | `resolve_watermark_legacy_skips_identity_check`; persist en poll sin cambio de last |
| Ceiling heuristic | APTO | test `resolve_watermark_ceiling_resets_stale_high_uid` |
| Tests | APTO | `cargo test -p email-watcher` → 19/19 OK |
| Docs operador | APTO | `start-sddia.md` + `email-watcher.md` |

## Dictamen

```json
{
  "global": "APTO",
  "pbi_archived": true,
  "branch": "fix/email-watcher-imap-account-watermark"
}
```
