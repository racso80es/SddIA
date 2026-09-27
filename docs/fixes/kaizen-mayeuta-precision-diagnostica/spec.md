---
feature_name: kaizen-mayeuta-precision-diagnostica
created: "2026-09-27"
process: bug-fix
branch_name: fix/kaizen-mayeuta-precision-diagnostica
persist_ref: docs/fixes/kaizen-mayeuta-precision-diagnostica
pbi_ref: docs/todos/pending/[KAIZEN] Mayeuta — precisión diagnóstica, catálogo de firmas, evidencia acotada y corpus de laudos.md
document_id: PBI-KAIZEN-MAYEUTA-PRECISION-DIAGNOSTICA
execution_id: "39778d5c-5085-4eb8-b6ad-5860c2b92a33"
---

# Especificación — Kaizen Mayeuta precisión diagnóstica

## Problema

Clasificación de fracturas por cubos léxicos duplicados entre `delivery_close.rs` y `enrich_fracture_pbi_kaizen.rs`, con `verdict_priority`, catch-all contradictorio y evidencia sobre blob concatenado. Ver PBI v1.1.0 §1–2.

## Cambio requerido

1. **SSOT** `SddIA/core/fracture-signatures.json` + `core.fractureSignatures` en `cumulo.paths.json`.
2. **Loader/matcher** en `execute-process/src/core/fracture_signatures.rs` (AST: all/any/none, any_groups, excludes, refine, scope_only).
3. **DCC** supresión vía catálogo (`consumers: dcc_suppress`).
4. **Mayeuta** diagnóstico vía catálogo; F3: normalización de traza, sin jurisdicción/bypass, sin catch-all, veredicto por orden de catálogo.
5. **Corpus** 8 fixtures + `fracture_corpus_baseline` / `fracture_corpus_regression`.
6. **SFD** `friction_id` OPTIONAL (entity-manager); Cúmulo frontmatter; enrich structured si id catalogado.
7. **Sección** K6 (clasificación, firma, evidencia, señales secundarias).
8. **Evolution** F6.

## Criterios de aceptación

PBI §7 CA1–CA11.

## Fuera de alcance

Supresión DCC non-fast-forward (FIX `4c01d65b972f`). LLM en enrich. Gate CI de precisión mínima.
