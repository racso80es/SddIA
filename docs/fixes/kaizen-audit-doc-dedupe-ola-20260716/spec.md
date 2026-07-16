---
feature_name: kaizen-audit-doc-dedupe-ola-20260716
created: "2026-07-16"
process: bug-fix
base: main
branch: fix/kaizen-audit-doc-dedupe-ola-20260716
uuid: fa7b3fc4-2bfd-4c3a-9cce-52bc47b09ee9
scope: materialize-kaizen-alert-doc
---

# Spec — Dedupe PENDING_AUDIT_DOC (event-bus-audit)

## Decisión

Misma táctica que ola centinelas: no reabrir deuda de bus; cortar spam documental.  
`materialize-kaizen-alert-doc` reutiliza TODO abierto con misma huella `alert_kind` + `implicated_files` (sin `review_id`).

## Causa raíz

| Hecho | Evidencia |
|-------|-----------|
| 7 TODOs | hashes distintos por `review_id` |
| 2 clusters | 5× (86 DL / 9 pending) + 2× (1 DL / 11 pending) |
| Idempotencia previa | solo path exacto (`review_id`+files) |

## CA

| ID | Criterio |
|----|----------|
| CA1 | Misma huella, distinto review_id → 1 archivo |
| CA2 | Tests `materialize_kaizen_*` OK |
| CA3 | 7 satélites en `done/` + PBI ola done |
| CA4 | validacion APTO + pbi_archived |
