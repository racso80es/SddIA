---
feature_name: dcc-snapshot-hash-stuck
created: "2026-10-31"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/dcc-snapshot-hash-stuck
tracker_ref: AUT-22
items_applied:
  - T1-snapshot-branch-mismatch
  - T2-snapshot-hash-advanced
  - T3-fracture-signature-corpus
---

# Ejecución — AUT-22

Rama `Fix_Events_Multiple`. Plan T1–T3 materializado en un diff (motor + catálogo + tests + evolution).

Verificación local:

```bash
cd SddIA && cargo test -p execute-process parse_star_branch snapshot_branch snapshot_hash_advanced snapshot_branch_mismatch fracture_corpus_regression -- --nocapture
```

Pendiente Argos: `validacion.md` APTO y AUT-22 en Done (Linear).
