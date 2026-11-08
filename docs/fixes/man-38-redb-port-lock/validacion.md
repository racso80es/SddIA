---
feature_name: man-38-redb-port-lock
process: bug-fix
phase: Verificación
agent: tekton
branch: fix/man-38-redb-port-lock
branch_name: fix/man-38-redb-port-lock
persist_ref: docs/fixes/man-38-redb-port-lock
tracker_ref: MAN-38
document_id: PBI-90397f23
execution_id: "0c5d6ea9-b949-4b2e-859f-6f225504cdf3"
source_process: bug-fix
global: APTO
pbi_archived: false
approval_status: pendiente_argos
verdict: pendiente_argos
delivery_state: in_progress
---

# Validación — man-38-redb-port-lock (Tekton)

## Criterios CA (spec)

| ID | Evidencia |
|----|-----------|
| CA-1 | Sin bootstrap Redb al arrancar CLI/`run_process` |
| CA-2 | Wrappers EDA usan `with_productive_port` / `with_event_port` |
| CA-3 | Watcher: claim y `record_delivery` scoped; sin Arc de proceso |
| CA-6 | Test `productive_not_left_in_thread_local_after_op` |
| CA-8 | Línea `open_contention` en host |

CA-4 (lab PPR concurrente) y CA-7 (no regresión MAN-35): pendiente CI / Argos en PR.

## Comandos

Ver `implementation.md` § Verificación mínima — ejecutados en rama `fix/man-38-redb-port-lock`.

## Nota

`global: APTO` preliminar Tekton; sellado Argos y `delivery-close-cycle` cierran MAN-38 en Linear (`done_gate: linear`).
