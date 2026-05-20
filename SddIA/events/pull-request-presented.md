---
uuid: "5e488ae6-7cb2-4a2c-9725-4a7d4ce239ea"
name: "pull-request-presented"
version: "1.1.0"
contract: "events-contract v1.0.0"
event_type: "PullRequest_Presented"
context: "ecosystem-evolution"
capabilities:
  - "pull_request_presented"
hash_signature: "sha256:d536a30b1d0c39ef7dbc4053775f6efa3ef673a63dcd03dbbd92700e42f6df18"
---

# Event: PullRequest_Presented

Clase ECST para presentación de PR en bus local. Suscripción no-op hasta auditoría Argos.

## Payload ECST

### REQUIRED
- `branch`
- `status`

### OPTIONAL
- `pr_url`

### FORBIDDEN
- *(ninguno)*

## Emisores autorizados

- `emit-pr-presented-event` (invocado por `delivery-close-cycle` con `emitter_agent` del proceso)

## Suscripciones

Ver `SddIA/core/event-subscriptions.json` → clave `PullRequest_Presented`.
