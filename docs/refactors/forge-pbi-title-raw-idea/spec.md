---
feature_name: forge-pbi-title-raw-idea
created: "2026-10-27"
process: refactorization
persist_ref: docs/refactors/forge-pbi-title-raw-idea
tracker_ref: AUT-28
document_id: PBI-39b3d861
---

# Especificación

## Cambio

Función `distill_title_from_raw_idea` en `handlers/forge_pbi.rs`: reglas deterministas AUT-28 (encabezado `#`, primera línea útil, espacios colapsados, tope 120). `run` propaga `RAW_IDEA_TITLE_EMPTY` sin registrar issue.

## Fuera de alcance

Renombrar issues Linear ya abiertas; LLM en recepción.
