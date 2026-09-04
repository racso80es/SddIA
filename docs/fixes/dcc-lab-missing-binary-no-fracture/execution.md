---
feature_name: dcc-lab-missing-binary-no-fracture
created: "2026-09-04"
process: bug-fix
branch_name: fix/ignition-pre-push-guard
items_applied:
  - predicate-lab-binary-missing
  - emit-dcc-skip
  - unit-tests-ca1-ca3
---

# Ejecución — Ola 3

```bash
cd SddIA && cargo test -p execute-process --lib dcc_lab_binary_missing_trace_positives
cd SddIA && cargo test -p execute-process --lib dcc_fracture_suppressed_on_sddia_qa_missing
cd SddIA && cargo test -p execute-process --lib dcc_fracture_suppressed_on_git_manager_capsule_missing
cd SddIA && cargo test -p execute-process --lib dcc_fracture_still_emits_on_rbac
```

Cuatro tests `ok`. Ciclo DCC no requerido: CAs son unitarios sobre `emit_dcc_phase_fractures`.
