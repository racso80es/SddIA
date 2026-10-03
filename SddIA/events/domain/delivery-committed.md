---
uuid: "528c21e8-3804-4a2a-bb26-574b3f39495c"
name: "delivery-committed"
version: "1.1.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Delivery_Committed"
context: "source-control"
capabilities:
  - "delivery_committed"
hash_signature: "sha256:001ca841d2a114627c997b7838bf4e240983863e8eb362df2b86ea4f3cbe8eae"
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
