---
feature_name: route-domain-event-pr-merged-resilience
created: "2026-05-25"
process: bug-fix
branch: fix/route-domain-event-pr-merged-resilience
global: APTO
pbi_archived: true
pr_url: https://github.com/racso80es/SddIA/pull/50
checks:
  RM-CA1: pass
  RM-CA2: pass
  RM-CA3: pass
  RM-CA4: pass
  RM-CA5: pass
  RM-CA6: pass
git_changes:
  - SddIA/scripts/qa/eda_bus_utils.py
  - SddIA/scripts/qa/route_domain_event_core.py
  - SddIA/scripts/qa/test_eda_bus_v3plus.py
  - SddIA/events/events-contract.md
  - .dev/.env.example
  - docs/fixes/route-domain-event-pr-merged-resilience/
  - docs/todos/done/[Kaizen] route-domain-event — resiliencia PR mergeado sin depender solo de gh en watcher.md
---

# Validación — route-domain-event PR merged resilience

**Veredicto global: APTO**

## Criterios de aceptación

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| RM-CA1 | Laudo en spec | ✅ | `spec.md` §3 cadena multicapa |
| RM-CA2 | Sin DL pathspec post-merge | ✅ | `merge_already_done` vía gh/pull-ref |
| RM-CA3 | gh ausente + pull-ref | ✅ | `test_lifecycle_git_pull_ref_fallback` |
| RM-CA4 | PR abierto + rama ausente → error explícito | ✅ | `_pull_request_review_precheck` + test |
| RM-CA5 | Regresión bus V3+ | ✅ | 14/14 tests OK |
| RM-CA6 | E2E lab | ✅ | `run-eda-e2e-lab.py` success true |

## Integridad

| Check | Resultado |
|-------|-----------|
| `test_eda_bus_v3plus.py` | OK (14 tests) |
| `run-eda-e2e-lab.py --entity-class tool --json` | OK |

## Incidente referencia

Evento `ce5f287e-4e27-4d18-98f6-b9201596ae00` (PR #48): con fix aplicado, `dispatch_subscriber` resuelve merge vía gh y evita checkout fallido.
