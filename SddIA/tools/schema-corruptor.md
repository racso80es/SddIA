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
hash_signature: "sha256:2f3b6790328d6f93a656027fd124a7f03aacce978ca0616f7f676fdebd855821"
implementation_path_ref: "SddIA/tools/schema-corruptor"
---

# schema-corruptor

Declara `telemetry_provided: true` pero emite envelope stdout sin recibo válido. Vector para `telemetry-compliance-audit` → `Telemetry_Compliance_Breached`.

Proveedor de capacidad `qa:probe` (PBI-043 H9 · Caos).

## Interface

stdin JSON: `corruption_mode` — `empty` | `invalid_json` | `partial`.

## Propósito

Declara `telemetry_provided: true` pero emite envelope stdout sin recibo válido. Vector para `telemetry-compliance-audit` → `Telemetry_Compliance_Breached`.

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
