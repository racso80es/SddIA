---
feature_name: aut-64-sellado-tracker-origen
created: "2026-11-03"
scope: tracker-lifecycle-origin-seal
process: refactorization
persist_ref: docs/features/aut-64-sellado-tracker-origen
pbi_ref: AUT-64
---

# Especificación — AUT-64

## Comportamiento

| Origen | Tras | Sellado |
|--------|------|---------|
| `forge-pbi` | `create_issue` + descripción | backlog + comentario `PBI forjado: {document_id}` |
| `forge-hu` | `create_issue` | backlog + comentario `HU forjada: {document_id}` |
| `refine-item` / `refine` | `update_issue_description` refinado | todo + comentario `PBI/HU refinado/a: {id} v{version}` |

## EDA

Entradas vacías para `PBI_Forged`, `HU_Forged`, `PBI_Refined`, `HU_Refined` en `SddIA/core/event-domain-subscriptions.json`.

## Outbound

Todas las invocaciones lifecycle desde `tracker_stamp` incluyen `source_process` para anti-eco en `tracker-outbound`.
