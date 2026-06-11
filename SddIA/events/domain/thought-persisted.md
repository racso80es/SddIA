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
hash_signature: "sha256:9c5767eca80111ffe2087b52e6244b8b6dd795eff6bee3285bf8d5c95607b82c"
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

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Thought_Persisted`.
