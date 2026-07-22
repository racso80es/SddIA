---
context:
- event-routing
- ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:6f8ec16b43edf7504384110ef18f55fa373dcf97c9dac693f433799129ff8272
inputs:
- event_file_path: Ruta relativa al JSON en ./.events/telemetry/
minteo_maximo: null
name: route-telemetry
outputs:
- success: boolean
- delivery_status: mapa subscriber_id → status
phases:
- delegates_to:
  - agent:cumulo
  intent: Cargar event-telemetry-subscriptions.json y despachar suscriptores (stub Radamanto).
  name: Fan-out telemetría
  requires_capability:
  - contract: bus.route
    id: bus:route
    version: '>=1.0.0'
porcentaje_de_exito: null
uuid: b2c3d4e5-f6a7-4890-b1c2-d3e4f5a6b7c8
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# route-telemetry

Enrutador del bus fractal **telemetry**. Despacha `Raw_Execution_Finished` hacia `telemetry-batch-stub` (Fase 3) / Radamanto (Fase 4).

Implementación: `route_fractal_event_core.route_telemetry_event()`.
