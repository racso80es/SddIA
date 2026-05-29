---
feature_name: fix-eda-bus-e2e-smoke-entity-payload
created: "2026-05-29"
process: bug-fix
---

# Implementación

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/qa/execute-action.py` | `_run_emit_domain_mutation`: añadir `entity_type`, `entity_id` al payload ECST |

Paridad con `SddIA/scripts/qa/execute_process_capsules.py` → `emit_domain_mutation()`.
