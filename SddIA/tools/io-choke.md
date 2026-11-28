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
hash_signature: "sha256:b8e0dbd7d9a291729c0a045f6418940438445d67f5d782c6728acc053913b820"
implementation_path_ref: "SddIA/tools/io-choke"
---

# io-choke

## Propósito

Cápsula de caos que fuerza fallo E/S escribiendo un archivo read-only dentro del `workspace_path` (golden `capsule-invoke-smoke` / D-P5.2). Provee `qa:probe`.

## Interfaz de entrada y salida

stdin JSON: `workspace_path` (required), `target_file` (optional, default `.io-choke-target`).

Éxito (`success: true`): la escritura fue bloqueada físicamente.
