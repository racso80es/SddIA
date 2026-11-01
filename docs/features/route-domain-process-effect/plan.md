---
feature_name: route-domain-process-effect
created: "2026-11-01"
process: feature
branch_name: Fix_Events_Multiple
persist_ref: docs/features/route-domain-process-effect
tracker_ref: AUT-61
historia_ref: AUT-20
phases:
  - L0-plan
  - L1-child-env
  - L2-table
  - L3-tighten
---

# Plan — AUT-61 en `Fix_Events_Multiple`

Sin `workspace-init`. La rama ya es `Fix_Events_Multiple`. Este sello es L0.

## L0 — Planificación

- [x] AUT-61 forjado y refinado (`refinement: refined`, tipo kaizen), hijo de AUT-20
- [x] `objectives.md`, `spec.md`, `plan.md` bajo `docs/features/route-domain-process-effect/`

## L1 — Entorno del hijo

- [x] El test exporta `SDDIA_EVENT_STORE=redb` además de mock, sync y `SDDIA_EXECUTE_PROCESS_BIN`
- [x] El esqueleto temporal copia un árbol solo cuando un handler falla por fichero ausente

## L2 — Tabla

- [x] Filas nuevas del spec: Aiua, Local QA, Work_Initiated, Domain_Entity_Created, Domain_Entity_Degraded (cerbero y fix-tool), Suite_Execution_Requested, los cuatro de correo/preferencia, Tracker_Sync_Failed
- [x] Cada fila afirma `success` en la clave de proceso

## L3 — Tests ya existentes

- [x] Markdown, memory, telegram con texto y Kalma2 exigen `success` en su clave
- [x] El texto vacío de Telegram sigue en `skipped-empty-text`

`validacion.md` queda para esa corrida, no para este sello.
