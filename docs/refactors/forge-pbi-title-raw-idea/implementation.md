---
feature_name: forge-pbi-title-raw-idea
created: "2026-10-27"
process: refactorization
persist_ref: docs/refactors/forge-pbi-title-raw-idea
tracker_ref: AUT-28
document_id: PBI-39b3d861
---

# Implementación

- `distill_title_from_raw_idea`, `truncate_title`, heurística de líneas ignoradas (YAML, viñetas, cercas).
- Cuerpo forjado: H1 = título destilado, no `document_id`.
- Tests unitarios AC-1, AC-2, AC-3, AC-4 en `forge_pbi.rs`.
