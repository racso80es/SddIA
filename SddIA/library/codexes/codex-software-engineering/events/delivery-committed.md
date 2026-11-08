---
uuid: "b1c2d3e4-f5a6-4b7c-8d9e-0a1b2c3d4e5f"
name: "delivery-committed"
version: "1.1.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Delivery_Committed"
context: "source-control"
capabilities:
  - "delivery_committed"
hash_signature: "sha256:cdeb988a6fadf352aa116a96757ba02599d9709b2b8213472a2df8c28bdd7840"
---

# Event: Delivery_Committed

Cierre en trunk_direct. No abre ni fusiona Pull Request.

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
