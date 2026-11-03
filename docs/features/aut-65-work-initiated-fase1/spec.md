---
feature_name: aut-65-work-initiated-fase1
created: "2026-11-03"
scope: work-initiated-ignition
process: refactorization
persist_ref: docs/features/aut-65-work-initiated-fase1
pbi_ref: AUT-65
---

# Especificación — AUT-65

## Estado actual

`emit_work_initiated_fail_soft` vive en `workspace_init.rs` y solo corre desde `maybe_emit_work_initiated_on_tekton_entry` (`executor.rs`, `residual_runner.rs`). El test `workspace_init_does_not_emit_work_initiated` afirma que Fase 1 no emite.

## Estado destino

| Momento | Emisión |
|---|---|
| Fase 1 exitosa (`feature`, `bug-fix`, `refactorization`) | una vez |
| Entrada `agent:tekton` | ninguna |
| `SDDIA_LAB_SKIP_WORK_INITIATED` | ninguna |

## Payload

`branch` desde `branch_name`, `persist_ref`, `tracker_ref` si existe, si no `pbi_ref` para el sumidero file. `source_process` = nombre del proceso de entrega.

## Consumidor

`tekton.tracker-stamp` sobre `Work_Initiated`: In Progress, comentario `Work iniciado — rama … · persist …`, promoción de HU padre. Sin cambio de rangos.
