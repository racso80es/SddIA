---
feature_name: eda-scan-process-index
created: "2026-10-02"
process: bug-fix
persist_ref: docs/fixes/eda-scan-process-index
---

# Especificación

## Cambio

En `SddIA/engine/execute-process/src/engine/eda_coverage.rs`, `parse_index_uuids`:

- Omite líneas `---` y cabeceras de tabla (`| Name`).
- Extrae `entity_name` y `uuid` de filas pipe-separated sin requerir `` ` `` en la línea.
- Mantiene el regex UUID existente.

## Verificación

- Test unitario `parse_index_row_without_backticks`.
- Binario `execute-process --audit-eda-coverage --scan --json` → `orphan_count: 0` con matriz actual.
