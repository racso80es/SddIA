---

feature_name: man-39-refactorization-reentry-cli-collapse
created: "2026-11-10"
process: feature
items_applied:
  - T0-forge-reentry-skip: applied
  - T1-timeout-floor: applied
  - T3-dirty-peaje: applied
  - T5-lab-kaizen-label: applied
  - T6-tests: applied
branch_name: feat/man-39-refactorization-reentry-cli-collapse
persist_ref: docs/features/man-39-refactorization-reentry-cli-collapse
pbi_ref: MAN-39
execution_id: "454bcd99-8bbb-4cb3-8038-31cbe046ac55"
---
# Ejecución — man-39-refactorization-reentry-cli-collapse

## Comandos

```bash
cd SddIA && CARGO_TARGET_DIR=target cargo test -p execute-process forge_already_present execution_phase_timeout_floor path_in_scope_accepts_tracker --lib
cd SddIA && CARGO_TARGET_DIR=target cargo build -p execute-process -p linear-tracker-adapter
```

## Commits previstos

Un commit de forja `fix(execute-process): reentrada lifecycle y peaje dirty MAN-39` con código + cascada documental MAN-39.
