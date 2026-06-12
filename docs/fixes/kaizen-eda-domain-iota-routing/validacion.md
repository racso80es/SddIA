---
feature_name: kaizen-eda-domain-iota-routing
created: "2026-06-12"
process: bug-fix
branch_name: fix/kaizen-eda-domain-iota-routing
persist_ref: docs/fixes/kaizen-eda-domain-iota-routing
global: APTO
pbi_archived: true
branch: fix/kaizen-eda-domain-iota-routing
pr_url: https://github.com/racso80es/SddIA/pull/91
---

# Validación — kaizen EDA domain IOTA routing

## Checks

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| KZ-CA1 | `bus-operator` resuelve IOTA para `Manual_Task_Requested` | ✅ | smoke `resolve_subscribers` |
| KZ-CA2 | IOTA Testnet real (`SDDIA_LAB_SIMULATE_IOTA=0`) | ✅ | digest sin `lab-sim-` |
| KZ-CA3 | Watcher enruta `domain/` (15 eventos) | ✅ | `event-watcher --once` |
| KZ-CA4 | `event-domain-subscriptions.json` válido | ✅ | `json.tool` exit 0 |
| KZ-CA5 | PBI archivado + documentación fix | ✅ | `docs/todos/done/` + `spec.md` |

## Cierre documental

PBI movido a `docs/todos/done/` en rama `fix/kaizen-eda-domain-iota-routing`.
