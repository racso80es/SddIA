---
feature_name: forge-pbi-title-raw-idea
created: "2026-10-27"
process: refactorization
branch_name: refactor/forge-pbi-title-raw-idea
persist_ref: docs/refactors/forge-pbi-title-raw-idea
tracker_ref: AUT-28
document_id: PBI-39b3d861
---

# Objetivos — título de forge-pbi desde raw_idea

## Misión (AUT-28)

`create_issue` de `forge-pbi` usa como `title` una frase breve destilada de `raw_idea`. `document_id` permanece en el frontmatter; el H1 del cuerpo coincide con el título visible.

## Criterios de aceptación

| Id | Estado |
| -- | -- |
| AC-1 | Frase plana → `title` = frase; `document_id` distinto |
| AC-2 | `# encabezado` → `title` y H1 = texto del encabezado |
| AC-3 | Recorte a 120 caracteres en límite de palabra |
| AC-4 | Sin línea útil → `RAW_IDEA_TITLE_EMPTY`, sin `create_issue` |
| AC-5 | Sin `refine-pbi` para corregir título en alta correcta |
