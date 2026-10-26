---

feature_name: tracker-item-document-codec
created: "2026-10-26"
process: refactorization
branch_name: refactor/tracker-item-document-codec
persist_ref: docs/refactors/tracker-item-document-codec
tracker_ref: AUT-15
document_id: PBI-e1d15344
execution_id: "5fdff8ff-4df3-43d7-a1a9-aed1f9d7a85c"
---
# Objetivos — codec de documento de ítem en tracker

## Misión (AUT-15)

Desacoplar el dialecto de la descripción del issue (Linear `yaml` fence vs fichero `---`) del sello de ciclo (`tracker_column`, `refinement`, `refined`). El handler valida propiedades obligatorias sobre un documento canónico; el proveedor traduce formato.

## Criterios de aceptación

1. `refine-pbi` / `refine-hu` sellan ítems Linear sin `REFINE_NOT_FOUND` / `REFINE_INVALID` por fence.
2. `tracker-stamp::sync_item_tracker_column_remote` actualiza `tracker_column` en la descripción remota (sin `let _` silencioso ante parseo inválido).
3. Módulo `tracker_item_document` con `parse_item_description` y `render_item_description` (puerto lógico previo a operaciones en cápsula).
4. `normalize_item_markdown` cubre lectura Linear; `seal_markdown` delega en el codec.
5. Tests unitarios de fence Linear y roundtrip fichero.
6. Fuera: operaciones GraphQL nuevas en contrato `tracker.lifecycle` (iteración posterior); AUT-6 binding git; AUT-11 WASM token.

## Relación

Kaizen bajo `MAN-14`. Sustituye el parche táctico único en `refine.rs` por codec compartido con `tracker-stamp`.
