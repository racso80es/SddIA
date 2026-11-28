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
hash_signature: "sha256:b04f0ddbb1f4fe99e0765185f5fdedf2a365d8d74b1d7a6b817cc1dff04e74f8"
implementation_path_ref: "SddIA/tools/sandbox-breacher"
---

# sandbox-breacher

## Propósito

Caos RBAC: intento de escape del workspace; éxito = rechazo con `exitCode: 1` (`qa:probe`).

## Operación

stdin `workspace_path`, `escape_target`; crate `sandbox-breacher`.

## Interfaz de entrada y salida

stdin JSON: `workspace_path` (required), `escape_target` (optional, default `../breach-marker.txt`).
