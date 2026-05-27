---
uuid: "9c3d4e5f-6a7b-4c8d-0e1f-2a3b4c5d6e7f"
name: "tool-deprecated"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Tool_Deprecated"
context: "quality-assurance"
capabilities:
  - "tool_deprecated"
  - "self_healing_death"
hash_signature: "sha256:pending-anchor-on-merge"
---

# Event: Tool_Deprecated

Muerte definitiva de entidad tras superar `max_recovery_attempts` (AC4.6).

## Payload ECST

### REQUIRED

- `target_entity_id`
- `recovery_attempts`
- `reason`

### FORBIDDEN

- `branch`

## Emisores autorizados

- Agente **`radamanto`** (vía `radamanto-batch`)

## Suscripciones

Cerbero bloqueo permanente (D4.14).
