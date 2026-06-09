---
feature_name: argos-pr-audited
process: feature
created: 2026-05-31T08:00:00Z
---

# Specification

- **Evento:** `SddIA/events/domain/pull-request-audited.md` — ECST `PullRequest_Audited`, payload `audit_event_reference`, `target_entity_id`, `resolution`, `violated_rules`.
- **Acción:** `SddIA/actions/emit-pr-audited-event.md` — persistencia en `eda_bus.pending`.
- **Aduana:** `execute_process_capsules.py` — fase Veredicto invoca `emit-pr-audited-event`.
- **Merge DLT:** `emit-pr-merged-event` — `security_clearance.audit_event_reference` desde input o bus correlacionado.
- **Argos:** `SddIA/agents/argos.md` — `outputs` alineados con bucle operativo §2.
