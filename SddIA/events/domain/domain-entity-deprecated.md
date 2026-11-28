---
uuid: "4b5391d9-39ed-450c-8658-2fc3c8d7ec1b"
name: "domain-entity-deprecated"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Domain_Entity_Deprecated"
context: "quality-assurance"
capabilities:
  - "domain_entity_deprecated"
  - "self_healing_death"
hash_signature: "sha256:3d0a2f397cc258b114df44818bed0f16409278a88b9f0ce6562984dc3fdc29ab"
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
