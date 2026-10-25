---
feature_name: frente-documental-ciclo-vida
created: "2026-10-25"
process: refactorization
tracker_ref: AUT-5
execution_id: "c3a91f2e-7d4b-4e1a-9f06-8b2d4e5f1a0c"
---

# Implementación

- `item_lifecycle_frontmatter.rs`: sellado, parches YAML, tests unitarios.
- `handlers/forge_pbi.rs`: frontmatter de alta.
- `handlers/refine.rs`: sellado de refinamiento PBI/HU.
- `handlers/tracker_stamp.rs`: sync remoto, sumidero file, cancelación.
