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
hash_signature: "sha256:af1fe3e8168f966b6d79d2f42c52eff6f20b83bb3bb39c75dc1c095a7de33bcd"
implementation_path_ref: "SddIA/tools/lancedb-memory-adapter"
---

# lancedb-memory-adapter

Satélite nativo unificado de memoria vectorial LanceDB (`evolution`, `user_preferences`, `thought_graph_collection`) bajo contrato `capsule-json-io` v2 (`sddia-io`). Absorbe `thought-graph-access` y desacopla `execute-process` de `lancedb`/`tokio`.
