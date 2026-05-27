---
uuid: "5a02d313-685d-4464-84c1-ffe16ef6ba6d"
name: "raw-execution-finished"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "telemetry"
event_type: "Raw_Execution_Finished"
context: "system-operations"
capabilities:
  - "raw_execution_finished"
  - "thermodynamic_toll"
hash_signature: "sha256:1966b61a60f674771546de1f248bed6a4957a31f74ca16ad65ff1580e9405cf7"
---

# Event: Raw_Execution_Finished

Telemetría física emitida por el CLI al finalizar el Peaje Termodinámico (cronómetro, `exit_code`, `asset_id`). Pre-requisito genómico de Fase 3; instancias en `./.events/telemetry/` (futuro).

## Payload ECST

### REQUIRED

- `asset_id`
- `exit_code`
- `duration_ms`
- `process_name`

### OPTIONAL

- `telemetry_receipt`

### FORBIDDEN

- *(ninguno en v1.0.0)*

## Emisores autorizados

- `execute-process`
- `execute-action`
- Procesos/cápsulas CLI indexados que implementen Peaje Termodinámico (`execute_process_capsules`)

## Suscripciones

Reservado Fase 3.C — `event-telemetry-subscriptions.json` → `route-telemetry` → Radamanto (stub `telemetry-batch-stub` en Fase 3).
