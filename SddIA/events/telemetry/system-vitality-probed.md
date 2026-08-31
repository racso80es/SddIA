---
uuid: "380e11c3-49af-47d0-80b0-072575ae8f66"
name: "system-vitality-probed"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "telemetry"
event_type: "System_Vitality_Probed"
context: "system-operations"
capabilities:
  - "system_vitality_probed"
hash_signature: "sha256:6a543ad5b8bf6ee042ed4d1d76005b1db828d9523f70e774d0e1a0dba5b06b66"
---

# Event: System_Vitality_Probed

Censo de sondas de vitalidad no-proceso (y HTTP de Kalma2) con veredicto por sonda. Hecho auditable; la fractura la emite el proceso system-vitality-probe, no esta clase.

## Payload ECST

### REQUIRED
- `probes`
- `verdict`

### OPTIONAL
- `red_probe_ids`

### FORBIDDEN
- *(ninguno)*

## Emisores autorizados

- `system-vitality-probe`

## Suscripciones

Ver `SddIA/core/event-telemetry-subscriptions.json` → clave `System_Vitality_Probed`.
