---
uuid: "13c61cbc-45d6-46bb-9f6b-3d1a9a1298d0"
name: "domain-entity-deprecated"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Domain_Entity_Deprecated"
context: "quality-assurance"
capabilities:
  - "domain_entity_deprecated"
  - "self_healing_death"
hash_signature: "sha256:048c28ea34d55de71c8d3404003e198e499f6f1e9ce0ca67de12180c3d656bc1"
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

## Auditoría DLT

`dlt_audit_scope: local_only`.

## Suscripciones

Cerbero bloqueo permanente.

## Propósito (complemento)

Muerte definitiva de entidad tras superar `max_recovery_attempts`.
