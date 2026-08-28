---
uuid: "be28d7c5-18fd-4e08-9e53-e5e2bec63f16"
name: "tqm-dispatch-discarded"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "orchestration"
event_type: "TQM_Dispatch_Discarded"
context: "system-operations"
capabilities:
  - "tqm_dispatch_discarded"
hash_signature: "sha256:876a2aa15b6286ab195282bdd955b78a215d97cc85ff39b0259a6ca0d20d1b95"
---

# Event: TQM_Dispatch_Discarded

Despacho Kalma2 descartado por single-flight de PBI en task-queue-manager.

## Payload ECST

### REQUIRED
- `pbi_ref`
- `lock_key`
- `discarded_correlation_id`
- `reason`

### OPTIONAL
- `holder_correlation_id`

### FORBIDDEN
- *(ninguno)*

## Emisores autorizados

- `task-queue-manager`

## Suscripciones

Ver `SddIA/core/event-orchestration-subscriptions.json` → clave `TQM_Dispatch_Discarded`.
