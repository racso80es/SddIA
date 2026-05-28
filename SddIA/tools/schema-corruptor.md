---
uuid: "b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6f"
name: "schema-corruptor"
version: "1.0.0"
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
implementation_path_ref: "scripts/tools/schema-corruptor"
---

# schema-corruptor

Declara `telemetry_provided: true` pero emite envelope stdout sin recibo válido. Vector para `telemetry-compliance-audit` → `Telemetry_Compliance_Breached`.

## Interface

stdin JSON: `corruption_mode` — `empty` | `invalid_json` | `partial`.
