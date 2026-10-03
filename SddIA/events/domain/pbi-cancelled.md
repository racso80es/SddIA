---
uuid: "d203f467-082f-4cd6-895f-63736e5ab7b1"
name: "pbi-cancelled"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "PBI_Cancelled"
context: "knowledge-management"
capabilities:
  - "pbi_cancelled"
hash_signature: "sha256:911f7819fface7b4a3f07f528dcc1a5b2469302274ee9531a8feac16f952d6fe"
---

# Event: PBI_Cancelled

Cancelación de PBI; requiere reason y al menos tracker_ref o pbi_ref en emisión.

## Payload ECST

### REQUIRED
- `reason`

### OPTIONAL
- `tracker_ref`
- `pbi_ref`
- `project_slug`

### FORBIDDEN
- `team_key`
- `hash_signature`

## Emisores autorizados

- `task-queue-manager`
- `kalma2-interact`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `PBI_Cancelled`.
