---
feature_name: adecuar-ed-telemetry
created: "2026-05-29"
process: feature
items_applied:
  - T1-T4
  - T5
  - T6-T8
  - T9-T10
---

# Ejecución — adecuar-ed-telemetry

## Tareas aplicadas

| ID | Entregable | Estado |
|----|------------|--------|
| T1 | CRUD ECST v1.1.0 con `entity_type` + `entity_id` | ✅ |
| T2 | `domain-entity-degraded/deprecated/restored.md` | ✅ |
| T3 | Purga `tool-degraded`, `tool-deprecated`, `status-restored` | ✅ |
| T4 | `domain/index.md` v1.1.0 | ✅ |
| T5 | `event-domain-subscriptions.json` agnóstico | ✅ |
| T6 | `radamanto_batch_core` + `radamanto.instructions.json` | ✅ |
| T7 | `emit_domain_mutation` + `emit-domain-mutation.md` v1.1.0 | ✅ |
| T8 | Cerbero, fix-tool-process, agente Radamanto | ✅ |
| T9 | Tests Self-Healing + DLT | ✅ 5/5 |
| T10 | `audit-entity-eda-coverage --scan` | ✅ orphan_count: 0 |

## Validación ejecutada

```text
cd SddIA/scripts/qa && python -m unittest test_radamanto_self_healing test_radamanto_dlt_tool_status
→ OK (5 tests)

python SddIA/scripts/qa/audit-entity-eda-coverage.py --scan --json
→ orphan_count: 0
```

## Notas

- `fix-tool-process` acepta `entity_type` `tool` y `skill` (compat telemetría legacy).
- Archivos `pending/` siguen nomenclatura `{event_id}.json` (UUID).
