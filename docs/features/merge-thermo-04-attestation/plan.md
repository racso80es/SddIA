---
feature_name: merge-thermo-04-attestation
created: "2026-10-03"
process: feature
phases:
  - qa-attestation-module
  - hooks-read-write
  - accept-pr-skip-invalidate
  - tests-shell-rust
execution_id: "0d6b3391-7ae4-47bc-af51-b4c4f24c0295"
---
# Plan

1. Módulo `qa_attestation.rs` (escritura/validación/borrado).
2. Hooks `hook_common`, `pre_push_gate`, `post_merge_gate`.
3. `accept_pr` atajo genómico + limpieza Fase 4.
4. Tests shell + unitarios Rust.
