---
uuid: "9c3d4e5f-6a7b-4c8d-0e1f-2a3b4c5d6e7f"
name: "domain-entity-deprecated"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Domain_Entity_Deprecated"
context: "quality-assurance"
capabilities:
  - "domain_entity_deprecated"
  - "self_healing_death"
hash_signature: "sha256:pending-anchor-on-merge"
---

# Event: Domain_Entity_Deprecated

Muerte definitiva de entidad tras superar `max_recovery_attempts`.

## Payload ECST

### REQUIRED

- `entity_type`
- `entity_id`
- `recovery_attempts`
- `reason`

### FORBIDDEN

- `branch`
- `target_entity_id`

## Emisores autorizados

- Agente **`radamanto`** (vía `radamanto-batch`)

## Suscripciones

Cerbero bloqueo permanente.
