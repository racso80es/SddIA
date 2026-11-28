---
context: ecosystem-evolution
contract: process-contract v1.4.0
deprecated: true
hash_signature: "sha256:06b1f613e7707cb33e4f9cfebf1ff726767b4944a255990d6c83f413d19f39bd"
inputs:
- 'event_uri: Token lógico EventUri resuelto por EventManagerPort (eda://{family}/{event_id})'
minteo_maximo: null
name: telemetry-batch-stub
outputs:
- consumed: Indica que el evento fue leído y purgado (stub Radamanto Fase 3)
phases:
- intent: Leer instancia Raw_Execution_Finished, registrar consumo y purgar archivo fuente (simula Radamanto Fase 4).
  name: Consumo batch stub
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
  id: consumo_batch_stub
porcentaje_de_exito: null
superseded_by: radamanto-batch
uuid: f1e2d3c4-b5a6-4789-8c0d-1e2f3a4b5c6d
version: 2.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# telemetry-batch-stub

## Propósito

Stub de laboratorio (Fase 3 telemetría reactiva) que simula el consumo batch de `Raw_Execution_Finished` antes del agente Radamanto real (`superseded_by: radamanto-batch`, `deprecated: true`). Handler en `SddIA/engine/execute-process/src/engine/telemetry_batch_stub.rs`.

## Orquestación

1. Entrada obligatoria `event_uri`: token lógico `eda://telemetry/{event_id}` resuelto a ruta relativa bajo el repo.
2. Lee el JSON del evento, devuelve metadatos (`event_id`, `event_type`) y **purga** el archivo fuente (`purged: true`).
3. `residual_runner` etiqueta la fase `consumo_batch_stub` con `handler: telemetry-batch-stub`; enrutado fractal vía `route_fractal_core` / `invoke_process_full`.
