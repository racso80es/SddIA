---
feature_name: eda-scan-process-index
created: "2026-10-02"
process: bug-fix
branch_name: fix/linear-tracker-adapter-hash
persist_ref: docs/fixes/eda-scan-process-index
pbi_ref: docs/todos/done/[DEUDA] Tracker — escáner EDA de procesos.md
document_id: PBI-DEUDA-EDA-SCAN-PROCESOS
---

# Objetivos — escáner EDA e índice de procesos

Corregir `parse_index_uuids` para indexar filas `| nombre | uuid |` del índice de procesos sin exigir backticks, de modo que el gate Argos refleje huérfanos reales tras el backfill de cobertura (OSC-6).

## Criterios de aceptación

- AC-1: fila sintética sin backticks y sin cobertura incrementa `orphan_count`; con cobertura, no.
- AC-2: con los cinco uuid cubiertos (tracker ×3 + phagocyte + sync-client-assets), `--scan` reporta `orphan_count: 0`.
