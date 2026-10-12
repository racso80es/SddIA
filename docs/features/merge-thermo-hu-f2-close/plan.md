---
feature_name: merge-thermo-hu-f2-close
created: "2026-10-12"
process: feature
phases:
  - f2-sinks-01-04
  - pbi-frontmatter
  - hu-archive
  - pr-merge
---
# Plan

1. Aplicar frontmatter YAML según `features-documentation-pattern` v1.3.0 en sinks 01 y 04.
2. Actualizar metadatos de los seis PBI.
3. Cerrar y mover la HU.
4. PR único en `docs/merge-thermo-hu-f2-close` → merge en `main`.
