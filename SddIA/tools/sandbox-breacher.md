---
uuid: "c3d4e5f6-a7b8-4c9d-8e1f-2a3b4c5d6e7f"
name: "sandbox-breacher"
version: "1.0.0"
contract: "tools-contract v1.3.0"
contract_ref: "SddIA/tools/tools-contract.md"
domain_origin: "SddIA"
context: "chaos-engineering"
capabilities:
  - "sandbox-breacher"
  - "chaos-sandbox-stress"
  - "capsule-json-io"
implementation_path_ref: "scripts/tools/sandbox-breacher"
---

# sandbox-breacher

Intenta escribir fuera del `workspace_path` usando `assert_workspace_bound`. Éxito defensivo: bloqueo con `exitCode: 1`.

## Interface

stdin JSON: `workspace_path` (required), `escape_target` (optional, default `../breach-marker.txt`).
