---
uuid: "5fc8293d-d853-4b20-8387-b039c9eb5438"
name: "vector-memory-indexed"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Vector_Memory_Indexed"
context: "ecosystem-evolution"
capabilities:
  - "vector_memory_indexed"
hash_signature: "sha256:a8bc0af7ef78d20c254c7ed551240c34e584c34fc1ced6c7164ccd9afe0b69f4"
---

# Event: Vector_Memory_Indexed

Registro vectorial indexado en LanceDB tras captura epigenética o chunk semántico.

## Payload ECST

### REQUIRED
- `record_id`
- `store_path`
- `record_class`

### OPTIONAL
- `polarity`
- `embedding_dim`
- `operational_metadata`

### FORBIDDEN
- `secrets`
- `api_keys`

## Emisores autorizados

- `evolution-proxy-service`
- `lancedb-evolution-adapter`

## Auditoría DLT

`dlt_audit_scope: local_only`.

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Vector_Memory_Indexed`.
