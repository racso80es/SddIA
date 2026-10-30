---
feature_name: validacion-diff-seal
created: "2026-10-30"
process: feature
pbi_ref: AUT-39
---

# Especificación

## Motor

- `engine/diff_seal.rs`: `compute_diff_seal`, `validacion_ready_for_seal`, `apply_diff_seal_to_validacion`.
- `capsule_diff_seal_validacion` en `delivery_close.rs` (después de Gate pre-PR QA).

## Frontmatter sellado

- `validated_base_sha`, `validated_head_sha`, `validated_tree_sha`, `diff_digest` (`sha256:…`).

## Envelope

- `status: blocked`, `friction_id: validacion_not_apto` | `diff_seal_commit_failed`.
