---
feature_name: pbi-005-hito2-action-engine
created: "2026-05-20"
process: feature
items:
  - execute-process-feature-handler
  - bus-operator-skill
  - eda-micro-tools
  - execute-action-bus-delegation
---

# Implementación — touchpoints

| Componente | Cambio |
|------------|--------|
| `SddIA/scripts/qa/execute-process.py` | Handler `feature` fase 1 + reporte `simulated` fases 2–6 |
| `SddIA/skills/bus-operator.md` | Contrato genómico skill agrupadora |
| `scripts/skills/bus-operator.py` | Orquestación tools |
| `SddIA/scripts/tools/read-event-subscriptions/` | Lectura SSOT suscripciones |
| `SddIA/scripts/tools/manage-event-receipt/` | Sufijos `.notificado` / `.procesado` / `.error` |
| `SddIA/scripts/tools/transit-event-payload/` | Tránsito pending→processing→processed |
| `SddIA/scripts/qa/execute-action.py` | Delegación `cumulo` → `bus-operator` |
| `SddIA/skills/index.md` | Catálogo `bus-operator` |

Preexistentes en main: `execute-action.py` base, `markdown-table-editor`, watcher desacoplado.
