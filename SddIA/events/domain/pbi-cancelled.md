---
uuid: "0a45da02-c422-46ba-a2ab-3611eb02fc4d"
name: "pbi-cancelled"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "PBI_Cancelled"
context: "ecosystem-evolution"
capabilities:
  - "pbi_cancelled"
hash_signature: "sha256:7813d5620b61c7b145936a9c78969f54b35c404d0b7957953a31305211045ec5"
---

# Event: PBI_Cancelled

PBI cancelado en cola; emisores task-queue-manager y Kalma2 (tracker-markdown-apply en HU-B).

## Payload ECST

### REQUIRED
- `reason`

### OPTIONAL
- `tracker_ref`
- `pbi_ref`
- `event_id`
- `correlation_id`
- `occurred_at`

### FORBIDDEN
- `team_key`
- `linear_workflow_state_id`
- `linear_issue_url`

## Emisores autorizados

- `task-queue-manager`
- `kalma2-interact`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `PBI_Cancelled`.
