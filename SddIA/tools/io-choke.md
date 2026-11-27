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
hash_signature: "sha256:2d9398bf7e7f89a0210fb44d7ce81e5b6d166f1a1b02a8bc6acf1d36ef1206ab"
implementation_path_ref: "SddIA/tools/io-choke"
---

# io-choke

Simula asfixia E/S al escribir dentro del `workspace_path` inyectado (archivo read-only). Vector de estrés para fail-soft Peaje Termodinámico.

Proveedor de capacidad `qa:probe` (PBI-043 H9 · Caos).

## Interface

stdin JSON: `workspace_path` (required), `target_file` (optional, default `.io-choke-target`).

Éxito (`success: true`): la escritura fue bloqueada físicamente.

## Propósito

Simula asfixia E/S al escribir dentro del `workspace_path` inyectado (archivo read-only). Vector de estrés para fail-soft Peaje Termodinámico.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
