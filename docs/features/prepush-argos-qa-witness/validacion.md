---
feature_name: prepush-argos-qa-witness
process: bug-fix
branch: fix/linear-tracker-adapter-hash
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[DEUDA] Tracker — witness Argos en pre-push.md
checks:
  AC-1: APTO
  AC-2: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/route_domain_core.rs
  - SddIA/engine/execute-process/src/engine/phase_terminal.rs
  - SddIA/engine/execute-process/src/engine/residual_runner.rs
  - SddIA/engine/execute-process/src/engine/executor.rs
  - docs/features/prepush-argos-qa-witness/
---

# Validación — prepush-argos-qa-witness (OSC-12)

- AC-1: `spec.md` cita exit/status del witness (`delegation.exit_code: 1`, `dispatch_mode: sync`, `error_trace`).
- AC-2: tests `witness_local_qa_a55f1d12_subscriber_exit_one` + `ppr_blocked_phase_matches_witness_semantics`; alineado con `local-qa-requested.md` (sync bloqueante, exit a Git).
