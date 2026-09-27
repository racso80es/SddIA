---
feature_name: kaizen-mayeuta-precision-diagnostica
created: "2026-09-27"
process: bug-fix
branch_name: fix/kaizen-mayeuta-precision-diagnostica
persist_ref: docs/fixes/kaizen-mayeuta-precision-diagnostica
execution_id: "39778d5c-5085-4eb8-b6ad-5860c2b92a33"
---

# Implementación

- `SddIA/core/fracture-signatures.json` + `core.fractureSignatures` en `cumulo.paths.json`.
- Módulo `fracture_signatures.rs`: normalización K2, match DCC/Mayeuta, refine DLT.
- `enrich_fracture_pbi_kaizen.rs`: clasificación por catálogo, sección K6, sin bypass/catch-all.
- `delivery_close.rs`: supresión DCC vía catálogo.
- `materialize_fracture_pbi.rs`: `friction_id` catalogado en frontmatter; casilla corpus.
- `system-fracture-detected.md`: `friction_id` OPTIONAL.
- Test `fracture_corpus_regression` (8 especímenes).
