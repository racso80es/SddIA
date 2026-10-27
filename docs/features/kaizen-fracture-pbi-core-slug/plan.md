---
feature_name: kaizen-fracture-pbi-core-slug
created: "2026-10-27"
process: refactorization
---

# Plan — kaizen-fracture-pbi-core-slug

## T1 — Motor

- Implementar slug efectivo `core` y bifurcación ledger vs Linear en `materialize_fracture_pbi.rs`.
- Ampliar `create_fracture_issue` (labels + `build_pbi_body`).
- Tests: linear mock (materialized + already_open + huérfano pending), tempfile sin índice.

## T2 — Genoma

- `entity-manager` → bump `materialize-fracture-pbi` action (sumidero dual, `target_path` = `issue_ref` si linear).
- `evolution-register` + `gate-evolution --range`.

## T3 — Cierre

- `validacion.md` APTO en esta ancla; issue Linear en `Done`; borrar o archivar `.md` huérfanos locales (`51dc7bdfd2fe`, `701c77ebeab8-R2`) si ya existen AUT-22/AUT-23.

## Verificación

```bash
cargo test -p execute-process --lib materialize_fracture
```
