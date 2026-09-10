---
feature_name: aiua-motor-anatomy-eda
created: "2026-09-10"
process: feature
items:
  - forge-event-em
  - forge-action-em
  - forge-process-em
  - genome-conscience-subs
  - handler-parser-dispatch
  - route-subscriber
  - tests-lab
branch_name: feat/aiua-motor-anatomy-eda
persist_ref: docs/features/aiua-motor-anatomy-eda
execution_id: "8658220b-47ff-4513-a02e-2a3ed3e1adbe"
document_id: PBI-NUCLEO-AIUA-ANATOMIA-MOTORA-EDA
---

# Implementation — aiua-motor-anatomy-eda

## Touchpoints

| Path | Cambio |
|------|--------|
| `SddIA/events/domain/aiua-process-requested.md` | EM create; UUID `c9a6db76-…`; shape Kalma2 |
| `SddIA/events/domain/index.md` | Fila nueva |
| `SddIA/actions/dispatch-aiua-intent.md` | EM create; UUID `a1086194-…`; nativa |
| `SddIA/actions/index.md` | Fila nueva |
| `SddIA/process/aiua-stimulus-processing.md` | EM update 1.2.0; fase `Despacho-Motor` |
| `SddIA/process/index.md` | Versión 1.2.0 |
| `SddIA/conscience/aiua_core.md` | §6 tendones + fence; versión 1.2.0 |
| `SddIA/core/event-domain-subscriptions.json` | `Aiua_Process_Requested` → TQM (sin IOTA) |
| `SddIA/engine/execute-process/src/engine/aiua_intent.rs` | Parser + despacho fractal domain |
| `SddIA/engine/execute-process/src/engine/actions.rs` | Registro nativo |
| `SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs` | Overlay lab + acuse |
| `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | Sibling Kalma2 |

## Runtime

1. Combustión `agy` intacta (`--sandbox`).
2. `extract_aiua_intent` / overlay `SDDIA_LAB_MOCK_AIUA_INTENT` si outbound lab.
3. SDLC → `write_fractal_event(..., "domain")`. Suite → `emit-suite-execution-requested`.
4. Acuse `intent_dispatched` + `event_id`. Cero join TQM.
