---
uuid: "65dcff67-d392-4ab1-9977-2e320d3c8c34"
name: "domain-entity-updated"
version: "1.0.0"
contract: "events-contract v1.0.0"
event_type: "Domain_Entity_Updated"
context: "ecosystem-evolution"
capabilities:
  - "domain_entity_updated"
hash_signature: "sha256:ab9e37a0a1be908e9edd96e7e394622fe9aece8ea8452893320bf8851224883a"
---

# Event: Domain_Entity_Updated

Mutación genómica update. hash_signature_old y hash_signature_new REQUIRED; payload_schema_hash OPTIONAL si cambia esquema.

## Payload ECST

### REQUIRED
- `entity_class`
- `lifecycle_operation`
- `entity_uuid`
- `entity_name`
- `version`
- `hash_signature_old`
- `hash_signature_new`
- `changes_summary`
- `origin_topology`

### OPTIONAL
- `payload_schema_hash`

### FORBIDDEN
- *(ninguno)*

## Emisores autorizados

- `emit-domain-mutation`
- `entity-manager`

## Suscripciones

Ver `SddIA/core/event-subscriptions.json` → clave `Domain_Entity_Updated`.
