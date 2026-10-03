---
uuid: "513d9379-2f33-4418-9ae5-79a8e4b9197e"
name: "pbi-refined"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "PBI_Refined"
context: "ecosystem-evolution"
capabilities:
  - "pbi_refined"
hash_signature: "sha256:f580c20edf0fda2fcaecb8e61e3bf5ff60cf9b3a377d465ec49063a6ea119e22"
---

# Event: PBI_Refined

PBI refinado en cola pending; único trigger hacia estado tracker todo (vía tracker-stamp en PBI A-08).

## Payload ECST

### REQUIRED
- `event_id`
- `correlation_id`
- `source_process`
- `pbi_ref`
- `occurred_at`

### OPTIONAL
- `project_slug`
- `tracker_ref`

### FORBIDDEN
- `team_key`
- `linear_workflow_state_id`
- `linear_issue_url`

## Emisores autorizados

- `refine-pbi`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `PBI_Refined`.
