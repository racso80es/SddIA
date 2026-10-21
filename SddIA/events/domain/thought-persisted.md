---
uuid: "612a8b69-23fc-48d0-950e-28722ab084b9"
name: "thought-persisted"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Thought_Persisted"
context: "ecosystem-evolution"
capabilities:
  - "thought_persisted"
hash_signature: "sha256:d48d056b9cd993c276996fcb818b1541e30aa0c54b247b3c42f013d9fc506756"
---

# Event: Thought_Persisted

Mutación persistida en grafo de pensamiento espacial tras triaje vectorial exitoso.

## Payload ECST

### REQUIRED
- `node_id`
- `parent_id`
- `status`
- `store_path`

### OPTIONAL
- `embedding_dim`
- `similarity_score`

### FORBIDDEN
- `biological_vertex_output`

## Emisores autorizados

- `thought-triage-service`
- `lancedb-thought-repo`

## Auditoría DLT

`dlt_audit_scope: local_only`.

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Thought_Persisted`.
