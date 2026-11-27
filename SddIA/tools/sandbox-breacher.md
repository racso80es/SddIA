---
uuid: "c3d4e5f6-a7b8-4c9d-8e1f-2a3b4c5d6e7f"
name: "sandbox-breacher"
version: "1.1.0"
contract: "tools-contract v1.3.0"
contract_ref: "SddIA/tools/tools-contract.md"
domain_origin: "SddIA"
context: "chaos-engineering"
capabilities:
  - "sandbox-breacher"
  - "chaos-sandbox-stress"
  - "capsule-json-io"
provides:
  - id: "qa:probe"
    contract: "qa.probe"
    version: "1.0.0"
outputs:
  - "success": "boolean"
  - "exitCode": "integer"
  - "error": "string; diagnóstico si aplica"
hash_signature: "sha256:1fbe473222f8ed90fa65d2d3e2bcb7a4f9bac742bf22248212aabf4dab44877e"
implementation_path_ref: "SddIA/tools/sandbox-breacher"
---

# sandbox-breacher

Intenta escribir fuera del `workspace_path` usando `assert_workspace_bound`. Éxito defensivo: bloqueo con `exitCode: 1`.

Proveedor de capacidad `qa:probe` (PBI-043 H9 · Caos).

## Interface

stdin JSON: `workspace_path` (required), `escape_target` (optional, default `../breach-marker.txt`).
