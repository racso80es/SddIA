---
feature_name: kaizen-event-bus-audit-sensor-tuning
created: "2026-09-05"
process: bug-fix
phases: 4
branch_name: fix/kaizen-event-bus-audit-sensor-tuning
persist_ref: docs/fixes/kaizen-event-bus-audit-sensor-tuning
---

# Plan — kaizen-event-bus-audit-sensor-tuning

## Fase 0 — Planificación (esta entrega)

`spec.md` + `plan.md` + `objectives.md`. Commit de diseño. Sin mutación de runtime hasta Fase 1.

## Fase 1 — Sensor (`SddIA/tools/event-bus-audit/src/main.rs`)

1. Extraer `is_non_ecst_sink(&Value) -> bool` y `needs_kaizen_actionable(circuit_alert, actionable_stale) -> bool`.
2. En `validate_ecst_event`: sink no-ECST → `Anomaly.kind = "non_ecst_sink"`.
3. Al marcar stale pending: persistir `event_type` en `detail`; excluir `System_Fracture_Detected` del contador accionable.
4. `needs_kaizen` = S2 del spec. Informe Markdown: conteos `non_ecst_sink` y stale de fractura (informativos).
5. Tests unitarios EBA-CA1–CA3.

## Fase 2 — Plantilla + Clase

1. `build_todo_body` polimórfico; detector de TODO abierto con casilla DIA o bus.
2. Tests `materialize_kaizen_*` existentes + caso `event-bus-audit`.
3. Contratos `.md`: tool, process, action, event (emisores).

## Fase 3 — Cierre documental y entrega

`implementation.md` + `execution.md` + `validacion.md`. Archivar PBI → `docs/todos/done/`. `delivery-close-cycle`. CI verde post-PR (gate del operador).
