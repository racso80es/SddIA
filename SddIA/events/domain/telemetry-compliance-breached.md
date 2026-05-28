---
uuid: "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5e"
name: "telemetry-compliance-breached"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Telemetry_Compliance_Breached"
context: "quality-assurance"
capabilities:
  - "telemetry_compliance_breached"
  - "contract_audit_alert"
hash_signature: "sha256:pending-anchor-on-merge"
---

# Event: Telemetry_Compliance_Breached

Alerta de incumplimiento termodinámico: una ED declaró `telemetry_provided: true` pero no entregó recibo válido en stdout. Emisor: **`telemetry-compliance-audit`**.

## Payload ECST

### REQUIRED

- `asset_id`
- `capsule_id`
- `breach_reason` (`missing_receipt` | `schema_mismatch`)
- `process_name`

### OPTIONAL

- `expected_schema`

### FORBIDDEN

- `branch`
- `pr_url`

## Emisores autorizados

- Proceso **`telemetry-compliance-audit`**

## Suscripciones

Ninguna reactiva en Fase 5 (§5.D placeholder). Disponible en `./.events/domain/` para gobernanza futura.
