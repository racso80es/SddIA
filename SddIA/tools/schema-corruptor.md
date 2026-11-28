---
uuid: "b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6f"
name: "schema-corruptor"
version: "1.1.0"
contract: "tools-contract v1.3.0"
contract_ref: "SddIA/tools/tools-contract.md"
domain_origin: "SddIA"
context: "chaos-engineering"
telemetry_provided: true
telemetry_schema:
  - "prompt_tokens"
  - "completion_tokens"
capabilities:
  - "schema-corruptor"
  - "chaos-telemetry-stress"
  - "capsule-json-io"
provides:
  - id: "qa:probe"
    contract: "qa.probe"
    version: "1.0.0"
outputs:
  - "success": "boolean"
  - "exitCode": "integer"
  - "error": "string; diagnóstico si aplica"
hash_signature: "sha256:39cffa43a54995a830e8e917c168b6dbd02901bac6fb3a77f074b23fb2dce079"
implementation_path_ref: "SddIA/tools/schema-corruptor"
---

# schema-corruptor

## Propósito

Caos telemetría: envelope inválido pese a `telemetry_provided: true` (PBI-043 H9).

## Operación

Modos `corruption_mode`: empty | invalid_json | partial.

## Interfaz de entrada y salida

stdin JSON: `corruption_mode` — `empty` | `invalid_json` | `partial`.
