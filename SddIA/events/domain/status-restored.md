---
uuid: "8b2c3d4e-5f6a-4b7c-9d0e-1f2a3b4c5d6e"
name: "status-restored"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Status_Restored"
context: "quality-assurance"
capabilities:
  - "status_restored"
  - "self_healing_redemption"
hash_signature: "sha256:pending-anchor-on-merge"
---

# Event: Status_Restored

Redención de entidad previamente degradada. **Solo Radamanto** emite tras consolidar telemetría CLI post-reparación (R4.3). Argos no puede emitir este evento.

## Payload ECST

### REQUIRED

- `target_entity_id`
- `success_rate`

### OPTIONAL

- `consecutive_success_count`

### FORBIDDEN

- `structure_valid`

## Emisores autorizados

- Agente **`radamanto`** (vía `radamanto-batch` — único emisor)

## Suscripciones

Cerbero rehabilita RBAC reactivo a este evento (D4.14).
