---
uuid: "2011196d-9294-47d6-86e7-62586a4d711a"
name: "delivery-committed"
version: "1.1.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Delivery_Committed"
context: "source-control"
capabilities:
  - "delivery_committed"
hash_signature: "sha256:b36c6625bd9c2b588245c55a7eb67e37274e177c3bfbb75dd86cbd3cd340bf2b"
---

# Event: Delivery_Committed

Cierre en trunk_direct o sello con metadatos de entrega (D7). No abre ni fusiona Pull Request.

## Payload ECST

### REQUIRED
- `project_slug`
- `default_branch`
- `delivery_mode`

### OPTIONAL
- `tracker_ref`
- `pbi_ref`
- `persist_ref`
- `commit_sha`

### FORBIDDEN
- `hash_signature`

## Emisores autorizados

- `delivery-close-cycle`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Delivery_Committed`.
