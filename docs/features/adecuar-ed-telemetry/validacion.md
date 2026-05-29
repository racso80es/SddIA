---
feature_name: adecuar-ed-telemetry
created: "2026-05-29"
process: feature
branch: feat/adecuar-ed-telemetry
global: APTO
pbi_archived: true
checks:
  AC1: pass
  AC2: pass
  AC3: pass
  AC4: pass
  AC5: pass
  AC6: pass
  AC7: pass
  AC8: pass
  test_radamanto_self_healing: pass
  test_radamanto_dlt_tool_status: pass
  eda_scan_orphan_count: pass
git_changes:
  - SddIA/core/event-domain-subscriptions.json
  - SddIA/events/domain/domain-entity-created.md
  - SddIA/events/domain/domain-entity-updated.md
  - SddIA/events/domain/domain-entity-deleted.md
  - SddIA/events/domain/domain-entity-degraded.md
  - SddIA/events/domain/domain-entity-restored.md
  - SddIA/events/domain/domain-entity-deprecated.md
  - SddIA/events/domain/index.md
  - SddIA/actions/emit-domain-mutation.md
  - SddIA/agents/radamanto.md
  - SddIA/agents/radamanto.instructions.json
  - SddIA/process/cerbero-governance-react.md
  - SddIA/process/fix-tool-process.md
  - SddIA/scripts/qa/radamanto_batch_core.py
  - SddIA/scripts/qa/cerbero_governance_react_core.py
  - SddIA/scripts/qa/fix_tool_process_core.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/scripts/qa/test_radamanto_self_healing.py
  - SddIA/scripts/qa/test_radamanto_dlt_tool_status.py
  - docs/features/adecuar-ed-telemetry/
  - docs/todos/done/event_domain_subscriptions_Adecuar_ED_Telemetry.md
---

# Validación — Enrutamiento semántico agnóstico

**Veredicto global: APTO**

## Criterios PBI (AC1–AC8)

| AC | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| AC1 | Cero claves Tool_/Status_ en suscripciones | ✅ | `event-domain-subscriptions.json` |
| AC2 | Clases agnósticas; fósiles eliminados | ✅ | `domain-entity-degraded.md` etc.; sin `tool-degraded.md` |
| AC3 | CRUD payload con routing fields | ✅ | ECST v1.1.0 + `emit_domain_mutation` |
| AC4 | Radamanto emite Domain_Entity_* | ✅ | `test_full_self_healing_cycle` |
| AC5 | Cerbero RBAC operativo | ✅ | Revocación/restauración en ciclo E2E |
| AC6 | DLT Radamanto exclusivo | ✅ | `test_dlt_via_route_fanout_simulated` |
| AC7 | orphan_count: 0 | ✅ | `--scan` 2026-05-29 |
| AC8 | PBI archivado pre-merge | ✅ | `docs/todos/done/` |

## Regresión

- `test_radamanto_self_healing`: 4/4 OK
- `test_radamanto_dlt_tool_status`: 1/1 OK

## Cierre documental

- PBI movido a `docs/todos/done/` en rama del PR.
- `pbi_archived: true`.
