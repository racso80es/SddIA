---
feature_name: frente-documental-ciclo-vida
created: "2026-10-25"
process: refactorization
branch_name: refactor/frente-documental-ciclo-vida
persist_ref: docs/refactors/frente-documental-ciclo-vida
tracker_ref: AUT-5
document_id: PBI-5cb3fcb9
execution_id: "c3a91f2e-7d4b-4e1a-9f06-8b2d4e5f1a0c"
---

# Objetivos — frente documental y ciclo de vida

## Misión (AUT-5)

Separar en el frontmatter del ítem la **columna de tracker** (`tracker_column`) del **estado de refinamiento** (`refinement`), alineado con `tracker.state_map` y los eventos `PBI_Forged` … `Delivery_Committed`.

## Criterios de aceptación

1. `forge-pbi` escribe `tracker_column: backlog` y `refinement: pending` (sin `status: pending`).
2. `refine-pbi` / `refine-hu` sellan `tracker_column: todo`, `refinement: refined` y fecha `refined`.
3. `tracker-stamp` sincroniza `tracker_column` en la descripción remota y en fichero (sumidero `file`).
4. Transiciones: `in_progress`, `in_review`, `done`, `cancelled` según eventos de dominio.
5. No se usa `status: pending|refinado|refinada` como sustituto de columna.
