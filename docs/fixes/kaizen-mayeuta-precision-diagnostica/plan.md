---
feature_name: kaizen-mayeuta-precision-diagnostica
created: "2026-09-27"
process: bug-fix
branch_name: fix/kaizen-mayeuta-precision-diagnostica
persist_ref: docs/fixes/kaizen-mayeuta-precision-diagnostica
execution_id: "39778d5c-5085-4eb8-b6ad-5860c2b92a33"
---

# Plan de ejecución

| Paso | Entrega |
|------|---------|
| 1 | `fracture-signatures.json` migración 1:1 + loader Rust |
| 2 | Corpus fixtures F1 + test baseline |
| 3 | `delivery_close.rs` consume loader (F2 parity) |
| 4 | `analyze_fracture_kaizen` refactor F3 (catálogo, K2–K6) |
| 5 | `materialize_fracture_pbi` casilla corpus + `friction_id` |
| 6 | entity-manager: SFD class, enrich + materialize actions |
| 7 | Tests CA + `cargo test -p execute-process` |
| 8 | evolution + validacion.md + PBI → done + DCC |
