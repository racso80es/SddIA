---
uuid: "1f518278-7a3d-4160-b757-a3661d263ec3"
name: "domain-entity-created"
version: "1.0.0"
contract: "events-contract v1.0.0"
event_type: "Domain_Entity_Created"
context: "ecosystem-evolution"
capabilities:
  - "domain_entity_created"
hash_signature: "sha256:7df0d2d33a8ac664cd3eeef5955e4ca538a03a48c25d830c4a254c74c489c67d"
---

# Event: Domain_Entity_Created

Mutación genómica create. hash_signature_new REQUIRED; payload_schema_hash OPTIONAL (transición Ola A).

## Payload ECST

### REQUIRED
- `entity_class`
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

## Suscripciones

Ver `SddIA/core/event-subscriptions.json` → clave `Domain_Entity_Created`.
