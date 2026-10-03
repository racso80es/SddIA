---
feature_name: eda-scan-process-index
created: "2026-10-02"
process: bug-fix
items:
  - SddIA/engine/execute-process/src/engine/eda_coverage.rs
---

# Implementación

- Parser de índice de procesos tolerante a filas sin backticks.
- Test `parse_index_row_without_backticks` en módulo `index_parse_tests`.
