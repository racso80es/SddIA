---
uuid: "131ca963-db42-43cd-ade3-a41c3b704147"
name: "rbac-governor"
version: "1.0.0"
contract: "skills-contract v1.1.0"
context: "knowledge-management"
capabilities:
  - "rbac-governance"
  - "self-healing-react"
provides:
  - id: "gov:rbac"
    contract: "gov.rbac"
    version: "1.0.0"
hash_signature: "sha256:f9efcfaf84c66e41b05c5ccd4faab35fe6fe1fcbc30ddc665d00acbedc656d6d"
inputs:
  - "operation": "string; enum: REACT_RBAC_GOVERNANCE"
  - "operation_payload": "object; event_file_path / event_type"
outputs:
  - "success": "boolean"
  - "exitCode": "integer"
  - "data": "object; governance_result"
  - "error": "string; diagnóstico en fallo"
---

# Skill: rbac-governor

Proveedor canónico de `gov:rbac` (PBI-045 H11-C · laudo Racso). Ancla semántica DI para gobernanza RBAC Self-Healing.

La ejecución residual nativa permanece en `cerbero_governance_react_core`; esta skill declara el contrato DI. Ortogonal a `audit:compliance`.
