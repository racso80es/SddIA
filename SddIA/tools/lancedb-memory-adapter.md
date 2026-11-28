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
hash_signature: "sha256:e450f74052c78ca1d77058c54b044cfcc3b09bbbbb6af843a7f1aabc29f501e1"
implementation_path_ref: "SddIA/tools/lancedb-memory-adapter"
---

# lancedb-memory-adapter

## Propósito

Proveedor unificado LanceDB para `memory:evolution-store`, `memory:pref_vector`, `memory:thought_graph` (crate en `implementation_path_ref`).

## Operación

Invocado vía DI desde `execute-process`; sin embedding `lancedb` en el orquestador.
