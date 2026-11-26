---
feature_name: aut-126-tracker-item-title-composition
created: "2026-11-26"
process: feature
branch: feat/aut-126-tracker-item-title-composition
branch_name: feat/aut-126-tracker-item-title-composition
persist_ref: docs/features/aut-126-tracker-item-title-composition
tracker_ref: AUT-126
execution_id: "afa7d5ea-f33e-452e-8e5e-e90edff43c60"
global: NO_APTO
source_process: feature
checks:
  AC1: APTO
  AC2: APTO
  AC3: APTO
  AC4: APTO
  AC5: PARCIAL
  global: NO_APTO
git_changes:
  branch: feat/aut-126-tracker-item-title-composition
  base: main
---
# Validación — AUT-126

Evidencia local: `cargo test -p execute-process --lib tracker_item` (6 passed). **AC5** (saneamiento AUT-125 en Linear) pendiente post-merge. **global: NO_APTO** hasta CI verde en PR.