---
feature_name: dcc-pre-pr-qa-gate
created: "2026-10-30"
process: feature
pbi_ref: AUT-38
---

# Plan

1. Extender `qa_attestation.rs` (worktree + validación HEAD).
2. Fase y handler en `delivery_close.rs`.
3. Bump `delivery-close-cycle` 1.5.0 + hash fases.
4. Tests unitarios + `sync_dcc_process_hash_signature`.
