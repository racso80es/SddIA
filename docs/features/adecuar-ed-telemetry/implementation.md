---
feature_name: adecuar-ed-telemetry
created: "2026-05-29"
process: feature
items:
  - id: T1-T4
    touchpoint: SddIA/events/domain/
    proposal: CRUD v1.1.0 + Clases agnósticas degraded/deprecated/restored; purga fósiles tool-*
  - id: T5
    touchpoint: SddIA/core/event-domain-subscriptions.json
    proposal: Reemplazar Tool_* / Status_Restored por Domain_Entity_Degraded/Restored/Deprecated
  - id: T6-T8
    touchpoint: radamanto_batch_core, cerbero_governance_react_core, fix_tool_process_core, emit_domain_mutation
    proposal: Payload entity_type/entity_id; consumidores agnósticos
  - id: T9
    touchpoint: test_radamanto_*.py
    proposal: Assert tipos Domain_Entity_*
---

# Implementación — adecuar-ed-telemetry

Propuestas materializadas según `spec.md` y `plan.md` T1–T11.
