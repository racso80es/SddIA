---
feature_name: tracker-lifecycle-persist-ref
created: "2026-10-22"
process: feature
branch_name: feat/tracker-lifecycle-persist-ref
persist_ref: docs/features/tracker-lifecycle-persist-ref
pbi_ref: docs/todos/done/[ARQUITECTURA] HU-003 PBI-04 — tracker lifecycle.md
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-04
pbi_uuid: "c62ed63e-28b5-4a6b-b742-90661afcfed8"
execution_id: "76a499d1-59eb-4b02-8e18-5586b41190f0"
---

# Objetivos — tracker-lifecycle-persist-ref

## Misión

La forja informa `persist_ref` con la señal `Work_Initiated`. La consume `tracker-stamp`. `feature`, `bug-fix` y `refactorization` no ganan `tracker-operations`.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Nombrar emisor y consumidor en el spec | Añadir el contexto a la forja |
| Probar que el comentario no mueve el issue a `done` | Inventario Draft / HU Refined / PBI Forged |
| Corte `file` sin comentario | Mover artefactos fuera de `docs/features` y `docs/fixes` |

## Criterios

| ID | Criterio |
|----|----------|
| AC-1 | Los tres procesos de forja no declaran `tracker-operations` ni una fase `tool:`. |
| AC-2 | `Work_Initiated` deja `update_issue_state` + `create_comment`. No escribe `done`. |
| AC-3 | El diff de genoma queda en cero. El techo del spec son tres ficheros y no se editan. |
