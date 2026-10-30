---
feature_name: validacion-diff-seal
created: "2026-10-30"
process: feature
pbi_ref: AUT-39
---

# Clarify — validacion-diff-seal

- Depende de AUT-38 (gate pre-PR) en la misma pila de ramas hasta merge de PR #414.
- `diff_digest` usa merge-base con `target_branch` (default `main`).
- Lab: `SDDIA_LAB_SKIP_DIFF_SEAL`.
