---

feature_name: aut-126-tracker-item-title-composition
created: "2026-11-26"
process: feature
branch: feat/aut-126-tracker-item-title-composition
branch_name: feat/aut-126-tracker-item-title-composition
persist_ref: docs/features/aut-126-tracker-item-title-composition
tracker_ref: AUT-126
global: APTO
source_process: feature
checks:
  AC1: APTO
  AC2: APTO
  AC3: APTO
  AC4: APTO
  AC5: APTO
  global: APTO
  branch: APTO
  git_changes: APTO
  F3_TECH_GATE: APTO
git_changes:
  branch: feat/aut-126-tracker-item-title-composition
  base: main
execution_id: "2cddf289-b44b-4855-8339-fee4fb4397d6"
---
# Validación — AUT-126

Evidencia local: `cargo test -p execute-process --lib tracker_item` (7 passed, incl. `aut125_witness_duplicate_h1_canonicalized`). Saneamiento remoto AUT-125: re-ejecutar `refine-item` / sellado tras merge con codec desplegado. **global: APTO** pendiente confirmación CI en PR #479.