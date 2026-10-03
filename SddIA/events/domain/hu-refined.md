---
uuid: "b48cd041-efd3-4980-8013-a56b82b6889d"
name: "hu-refined"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "HU_Refined"
context: "ecosystem-evolution"
capabilities:
  - "hu_refined"
hash_signature: "sha256:f7c6b8d7d10fea288a3284ebb52342bddc57e20d0f6c6fe754eb2354bdbb005b"
---

# Event: HU_Refined

Historia de usuario refinada; trigger hacia estado tracker todo (vía tracker-stamp en PBI A-08).

## Payload ECST

### REQUIRED
- `event_id`
- `correlation_id`
- `source_process`
- `hu_ref`
- `occurred_at`

### OPTIONAL
- `project_slug`
- `tracker_ref`

### FORBIDDEN
- `team_key`
- `linear_workflow_state_id`
- `linear_issue_url`

## Emisores autorizados

- `refine-hu`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `HU_Refined`.
