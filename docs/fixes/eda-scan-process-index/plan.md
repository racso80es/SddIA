---
feature_name: eda-scan-process-index
created: "2026-10-02"
process: bug-fix
phases:
  - name: Parser
    intent: Relajar filtro de backticks en índice de procesos
  - name: Gate
    intent: Confirmar orphan_count con matriz sellada
---

# Plan

1. Ajustar `parse_index_uuids` y añadir test.
2. Recompilar `execute-process`.
3. Ejecutar `--audit-eda-coverage --scan`.
4. Cierre documental (PBI → done/, `validacion.md` APTO).
