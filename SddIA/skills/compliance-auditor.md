---
uuid: "a9b8c7d6-e5f4-4321-9a0b-1c2d3e4f5a6b"
name: "compliance-auditor"
version: "2.0.0"
contract: "skills-contract v1.1.0"
context: "quality-assurance"
capabilities:
  - "thermodynamic-compliance-audit"
  - "delivery-state-seal"
provides:
  - id: "audit:compliance"
    contract: "audit.compliance"
    version: "1.0.0"
hash_signature: "sha256:afc20daf0b15d81dc4b712fa76fa286a673853062ffee0e6d3a7a4d548257116"
inputs:
  - "operation": "string; enum: AUDIT_TELEMETRY_COMPLIANCE"
  - "operation_payload": "object; event_uri y contexto de cruce"
outputs:
  - "success": "boolean"
  - "exitCode": "integer"
  - "data": "object; resultado auditoría / delivery_state"
  - "error": "string; diagnóstico en fallo"
---

# Skill: compliance-auditor

## Propósito

Contrato DI `audit:compliance` (PBI-043 H9-D); ortogonal a caos `qa:probe`.

## Operación

Operación `AUDIT_TELEMETRY_COMPLIANCE`; handler Rust residual en proceso telemetry-compliance-audit.