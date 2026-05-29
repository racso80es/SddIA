---
uuid: "8b2c3d4e-5f6a-4b7c-9d0e-1f2a3b4c5d6e"
name: "domain-entity-restored"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Domain_Entity_Restored"
context: "quality-assurance"
capabilities:
  - "domain_entity_restored"
  - "self_healing_redemption"
hash_signature: "sha256:pending-anchor-on-merge"
---

# Event: Domain_Entity_Restored

Redención de entidad previamente degradada. **Solo Radamanto** emite tras consolidar telemetría CLI post-reparación (R4.3).

## Payload ECST

### REQUIRED

- `entity_type`
- `entity_id`
- `success_rate`

### OPTIONAL

- `consecutive_success_count`

### FORBIDDEN

- `structure_valid`
- `target_entity_id`

## Emisores autorizados

- Agente **`radamanto`** (vía `radamanto-batch` — único emisor)

## Suscripciones

Cerbero rehabilita RBAC reactivo a este evento.
