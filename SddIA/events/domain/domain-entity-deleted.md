---
uuid: "a7c81b2f-b466-4b18-82c5-84ef0a5941b8"
name: "domain-entity-deleted"
version: "1.1.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Domain_Entity_Deleted"
context: "ecosystem-evolution"
capabilities:
  - "domain_entity_deleted"
hash_signature: "sha256:832127188eebae9e26b9b08da6186f387b1e797d4222b4970860710c66c7e1b1"
---

# Event: Domain_Entity_Deleted

Mutación genómica delete. hash_signature_old REQUIRED; hash_signature_new debe ser null.

## Payload ECST

### REQUIRED
- `entity_class`
- `entity_type`
- `entity_id`
- `lifecycle_operation`
- `entity_uuid`
- `entity_name`
- `hash_signature_old`
- `changes_summary`
- `origin_topology`

### OPTIONAL
- `version`

### FORBIDDEN
- `hash_signature_new`

## Emisores autorizados

- `emit-domain-mutation`
- `entity-manager`

## Auditoría DLT

`dlt_audit_scope: local_only`.

## Suscripciones

Ver `SddIA/core/event-subscriptions.json` → clave `Domain_Entity_Deleted`.
