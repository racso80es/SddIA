---
uuid: "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5e"
name: "io-choke"
version: "1.1.0"
contract: "tools-contract v1.3.0"
contract_ref: "SddIA/tools/tools-contract.md"
domain_origin: "SddIA"
context: "chaos-engineering"
capabilities:
  - "io-choke"
  - "chaos-io-stress"
  - "capsule-json-io"
provides:
  - id: "qa:probe"
    contract: "qa.probe"
    version: "1.0.0"
outputs:
  - "success": "boolean"
  - "exitCode": "integer"
  - "error": "string; diagnóstico si aplica"
hash_signature: "sha256:ee711c90805f62f4f11863c731dae4b5cb66f4299fc6d821c5ec943e148b8d63"
implementation_path_ref: "SddIA/tools/io-choke"
---

# io-choke

Simula asfixia E/S al escribir dentro del `workspace_path` inyectado (archivo read-only). Vector de estrés para fail-soft Peaje Termodinámico.

Proveedor de capacidad `qa:probe` (PBI-043 H9 · Caos).

## Interface

stdin JSON: `workspace_path` (required), `target_file` (optional, default `.io-choke-target`).

Éxito (`success: true`): la escritura fue bloqueada físicamente.
