---
uuid: "131ca963-db42-43cd-ade3-a41c3b704147"
name: "rbac-governor"
version: "2.0.0"
contract: "skills-contract v1.1.0"
context: "knowledge-management"
capabilities:
  - "rbac-governance"
  - "self-healing-react"
provides:
  - id: "gov:rbac"
    contract: "gov.rbac"
    version: "1.0.0"
hash_signature: "sha256:eddd7a57a19b0093864f75805533e59cb86d712285fd7a9f5348fad03c1432fb"
inputs:
  - "operation": "string; enum: REACT_RBAC_GOVERNANCE"
  - "operation_payload": "object; event_uri / event_type"
outputs:
  - "success": "boolean"
  - "exitCode": "integer"
  - "data": "object; governance_result"
  - "error": "string; diagnóstico en fallo"
---

# Skill: rbac-governor

## Propósito

Contrato DI `gov:rbac` (PBI-045 H11-C); gobernanza RBAC self-healing.

## Operación

`REACT_RBAC_GOVERNANCE`; lógica residual en `cerbero_governance_react_core`.