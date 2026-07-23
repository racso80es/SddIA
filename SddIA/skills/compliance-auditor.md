---
uuid: "a9b8c7d6-e5f4-4321-9a0b-1c2d3e4f5a6b"
name: "compliance-auditor"
version: "1.0.0"
contract: "skills-contract v1.1.0"
context: "quality-assurance"
capabilities:
  - "thermodynamic-compliance-audit"
  - "delivery-state-seal"
provides:
  - id: "audit:compliance"
    contract: "audit.compliance"
    version: "1.0.0"
hash_signature: "sha256:072507c2f8dbe932e522f9321162bc571fce5594637553b9f280ceb47ac7196c"
inputs:
  - "operation": "string; enum: AUDIT_TELEMETRY_COMPLIANCE"
  - "operation_payload": "object; event_file_path y contexto de cruce"
outputs:
  - "success": "boolean"
  - "exitCode": "integer"
  - "data": "object; resultado auditoría / delivery_state"
  - "error": "string; diagnóstico en fallo"
---

# Skill: compliance-auditor

Proveedor canónico de `audit:compliance` (PBI-043 H9-D). Capacidad de **Gobernanza/cumplimiento** — ortogonal a `qa:probe` (Caos).

Usado por DI en `telemetry-compliance-audit` (path ciego / inyección). La ejecución residual nativa del proceso permanece en el handler Rust; esta skill ancla el contrato semántico DI.
