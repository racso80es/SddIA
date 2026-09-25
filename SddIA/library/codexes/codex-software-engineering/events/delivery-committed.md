---
uuid: "b1c2d3e4-f5a6-4b7c-8d9e-0a1b2c3d4e5f"
name: "delivery-committed"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Delivery_Committed"
context: "source-control"
---

# Event: Delivery_Committed

Cierre en `trunk_direct`. No abre ni fusiona Pull Request.

## Payload ECST

### REQUIRED
- `project_slug`
- `default_branch`
- `delivery_mode`

### OPTIONAL

### FORBIDDEN
- `hash_signature`

## Emisores autorizados

- `delivery-close-cycle`
