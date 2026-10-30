---
feature_name: diff-digest-ci
created: "2026-10-30"
process: feature
branch_name: feat/diff-digest-ci
persist_ref: docs/features/diff-digest-ci
pbi_ref: AUT-40
---

# Objetivos — diff-digest-ci

AUT-40: check CI `sddia-diff-digest` que recalcula `diff_digest` en PR y falla si no coincide con `{persist_ref}/validacion.md`.

## Alcance

- `sddia-qa verify-diff-digest` (paridad con sello DCC AUT-39).
- Job `sddia-diff-digest` en `.github/workflows/sddia-index-qa.yml`.
