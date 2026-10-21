---
uuid: "1f518278-7a3d-4160-b757-a3661d263ec3"
name: "domain-entity-created"
version: "1.1.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Domain_Entity_Created"
context: "ecosystem-evolution"
capabilities:
  - "domain_entity_created"
hash_signature: "sha256:e379020389ff4bddd291813b1c71ce3d3de186ebc29694691c7cc1a4f299c2f5"
---

# Event: Domain_Entity_Created

Mutación genómica create. hash_signature_new REQUIRED; payload_schema_hash OPTIONAL (transición Ola A).

## Payload ECST

### REQUIRED
- `entity_class`
- `entity_type`
- `entity_id`
- `lifecycle_operation`
- `entity_uuid`
- `entity_name`
- `version`
- `hash_signature_new`
- `changes_summary`
- `origin_topology`

### OPTIONAL
- `payload_schema_hash`

### FORBIDDEN
- `hash_signature_old`

## Emisores autorizados

- `emit-domain-mutation`
- `entity-manager`

## Auditoría DLT

`dlt_audit_scope: local_only`.

## Suscripciones

Ver `SddIA/core/event-subscriptions.json` → clave `Domain_Entity_Created`.
