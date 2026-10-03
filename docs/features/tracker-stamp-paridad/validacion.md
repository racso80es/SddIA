---
feature_name: tracker-stamp-paridad
process: feature
branch: fix/linear-tracker-adapter-hash
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[DEUDA] Tracker — paridad tracker-stamp con la HU.md
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs
  - SddIA/engine/execute-process/src/engine/tracker_pbi_meta.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/accept_pr.rs
  - SddIA/tools/linear-tracker-adapter/src/main.rs
  - SddIA/events/domain/pull-request-*.md
  - docs/features/tracker-stamp-paridad/
---

# Validación — tracker-stamp-paridad (OSC-8)

- AC-1: suite mock `tracker_stamp` (AC-18, ciclo work→presented→merged, AC-9 vía `LAB-HU-AC9-READY`).
- AC-2: `pr_emits_carry_tracker_ref_when_provided` + cableado delivery/accept-pr.
- AC-3: puntero de una línea en PBI archivado `PBI-ARQUITECTURA-TRACKER-STAMP`; sin reescritura del cuerpo histórico.
