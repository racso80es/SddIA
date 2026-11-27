---
uuid: "7a4e91c2-3b5d-4f8a-9c1e-6d2f8b0a4c71"
name: "lancedb-memory-adapter"
version: "1.0.0"
contract: "tools-contract v1.5.0"
domain_origin: "SddIA"
context: "ecosystem-evolution"
capabilities:
  - "lancedb_memory_adapter"
  - "capsule-json-io"
provides:
  - id: "memory:evolution-store"
    contract: "memory.evolution_store"
    version: "1.0.0"
  - id: "memory:pref-vector"
    contract: "memory.pref_vector"
    version: "1.0.0"
  - id: "memory:thought-graph"
    contract: "memory.thought_graph"
    version: "1.0.0"
outputs:
  - success: boolean
  - exitCode: integer
  - message: string
  - result: object
hash_signature: "sha256:b0d708c578d7729f27466857d45722c9ab0283124b1ccfdfd68c133bcbbfa171"
implementation_path_ref: "SddIA/tools/lancedb-memory-adapter"
---

# lancedb-memory-adapter

Satélite nativo unificado de memoria vectorial LanceDB (`evolution`, `user_preferences`, `thought_graph_collection`) bajo contrato `capsule-json-io` v2 (`sddia-io`). Absorbe `thought-graph-access` y desacopla `execute-process` de `lancedb`/`tokio`.

## Propósito

Satélite nativo unificado de memoria vectorial LanceDB (`evolution`, `user_preferences`, `thought_graph_collection`) bajo contrato `capsule-json-io` v2 (`sddia-io`). Absorbe `thought-graph-access` y desacopla `execute-process` de `lancedb`/`tokio`.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
