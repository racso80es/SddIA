---
feature_name: tracker-done-gate
process: feature
branch: fix/linear-tracker-adapter-hash
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[ARQUITECTURA] Tracker — gate Done según spike.md
checks:
  AC-1: APTO
  AC-2: APTO
git_changes:
  - docs/features/tracker-done-gate/
---

# Validación — tracker-done-gate (OSC-11)

- AC-1: `spec.md` declara **no migrar** y describe handler `feature-pbi-archive` (tres ramas en `phase_capsules.rs`).
- AC-2: diff limitado a `docs/features/tracker-done-gate/` y cierre PBI; sin `phase_capsules.rs` ni hooks.
