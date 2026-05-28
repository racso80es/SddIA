---
uuid: "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5e"
name: "io-choke"
version: "1.0.0"
contract: "tools-contract v1.3.0"
contract_ref: "SddIA/tools/tools-contract.md"
domain_origin: "SddIA"
context: "chaos-engineering"
capabilities:
  - "io-choke"
  - "chaos-io-stress"
  - "capsule-json-io"
implementation_path_ref: "scripts/tools/io-choke"
---

# io-choke

Simula asfixia E/S al escribir dentro del `workspace_path` inyectado (archivo read-only). Vector de estrés para fail-soft Peaje Termodinámico.

## Interface

stdin JSON: `workspace_path` (required), `target_file` (optional, default `.io-choke-target`).

Éxito (`success: true`): la escritura fue bloqueada físicamente.
