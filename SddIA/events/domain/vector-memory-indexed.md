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
hash_signature: "sha256:b93f620239dd646be8c2ed2631e959ec122a203c9d1d24b703c0a90b613279ee"
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

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Vector_Memory_Indexed`.
