---
feature_name: kaizen-tqm-single-flight-pbi
created: "2026-08-28"
process: bug-fix
branch_name: fix/kaizen-tqm-single-flight-pbi
persist_ref: docs/fixes/kaizen-tqm-single-flight-pbi
pbi_ref: docs/todos/done/[KAIZEN] TQM sin single-flight por PBI — cadenas bug-fix duplicadas y agentes en carrera.md
document_id: PBI-KAIZEN-TQM-SINGLE-FLIGHT-PBI
items_applied:
  - L1-lock-key-pbi
  - L2-liveness-hardening
  - L3-discard-proof-event
  - L4-detach-invariant
  - tests-unit
---

# Ejecución — TQM single-flight por PBI

## Tests unitarios

```text
cd SddIA && cargo test -p execute-process task_queue_manager
→ 13 passed
```

Cobertura: CA2/CA3/CA4/CA9/CA10/CA11 (tests dedicados); CA1/CA7/CA8 vía lógica de `dispatch_child` + emisión/proof.

## Forja genoma

```text
./sddia-run.sh --process entity-manager
→ event_id c037983e-…; artefacto SddIA/events/orchestration/tqm-dispatch-discarded.md
```

## Residual documentado

- **TQM-CA12** (smoke dos domain events → un `cursor-agent`): pendiente post-merge en entorno con `event-watcher` activo.
- **D2b fuera de Linux**: sin `starttime` en `/proc`; backend `kill0` en Unix no-linux (log `[TQM-SF-LIVENESS]`).
- **Suscripción orchestration**: sin entrada en `event-orchestration-subscriptions.json`; CA8 cubierto por proof durable, no por bus.
