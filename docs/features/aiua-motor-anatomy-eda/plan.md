---
feature_name: aiua-motor-anatomy-eda
created: "2026-09-10"
process: feature
phases:
  - forge-event-em
  - forge-action-em
  - forge-process-em
  - genome-conscience-subs
  - handler-parser-dispatch
  - route-subscriber
  - tests-lab
  - evolution-register
branch_name: feat/aiua-motor-anatomy-eda
persist_ref: docs/features/aiua-motor-anatomy-eda
execution_id: "8658220b-47ff-4513-a02e-2a3ed3e1adbe"
document_id: PBI-NUCLEO-AIUA-ANATOMIA-MOTORA-EDA
---

# Plan — aiua-motor-anatomy-eda

## Orden

1. **Forge evento** — `entity-manager` create `aiua-process-requested`.
2. **Forge acción** — `entity-manager` create `dispatch-aiua-intent`.
3. **Forge proceso** — `entity-manager` update `aiua-stimulus-processing` 1.2.0 (phases + outputs; replacements de cuerpo aparte si el factory corta).
4. **Conscience + SSOT Cúmulo** — tendones en `aiua_core.md`; clave en `event-domain-subscriptions.json`.
5. **Handler** — `aiua_intent.rs` + parseo en `aiua_stimulus.rs` + `try_run_native`.
6. **Route** — sibling `event_type` en `dispatch_subscriber`.
7. **Tests** — crate `execute-process --lib` acotados (`aiua_intent`, `aiua_stimulus`, route Kalma2-sibling).
8. **Evolution** — `sddia-qa evolution-register` + fila en `Evolution_log.md`.

## Forja DA-2

Prefijo RAW KERNEL antes de cada EM. Semillas en `.tmp/` (gitignore). `SDDIA_AGENT_RELAY_IDE=1` si el creator tiene fases agente.

No editar a mano `SddIA/events/`, `SddIA/actions/`, `SddIA/process/` (salvo que EM deje el índice a sincronizar por el propio creator).

## Commit

1. Planificación (este árbol documental + PBI v1.2.0).
2. Implementación (código + genoma EM + tests).
3. Cierre documental en rama tras CI verde (`validacion.md` APTO + PBI `done/`) antes de `accept-pr`.

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| EM update destruye I/O del proceso | Enviar `process_phases`+`process_outputs`+`process_version`; no `process_contract_version` 1.3.0. |
| `validate_ecst_event` exige clase en índice | Forjar evento **antes** del handler que escribe. |
| Lab-mock-agy 80 chars | Overlay env; no mutar crate skill. |
| Tests de route invocan TQM real | Lab-sync de mapeo de inputs; no join al hijo SDLC (CA-8). |
