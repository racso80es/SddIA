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
hash_signature: "sha256:879c5ff4604412f4fa266004bb79a50a946a664772059715250e5f9cfac20c9d"
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
