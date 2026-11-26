---



feature_name: aut-126-tracker-item-title-composition
created: "2026-11-26"
process: feature
branch_name: feat/aut-126-tracker-item-title-composition
persist_ref: docs/features/aut-126-tracker-item-title-composition
pbi_ref: AUT-126
tracker_ref: AUT-126
execution_id: "2cddf289-b44b-4855-8339-fee4fb4397d6"
---
# Objetivos — AUT-126

## Objetivo

Composición del contenido del ítem por proveedor de tracker (`file` / `linear`) sin duplicar el título como H1 en el cuerpo.

## Criterios de aceptación

- [x] **AC1:** forja con `raw_idea` con H1 → un solo H1 en file; cero H1 de título en `description` linear.
- [x] **AC2:** `refine-item` canónico local con un H1; sellado remoto linear sin H1 de título.
- [x] **AC3:** round-trip parse → render idempotente por dialecto (tests).
- [x] **AC4:** tests unitarios en `tracker_item_document`.
- [x] **AC5:** regresión testigo AUT-125 (`aut125_witness_duplicate_h1_canonicalized`); saneamiento remoto Linear en siguiente `refine-item`/`seal` con codec en main.