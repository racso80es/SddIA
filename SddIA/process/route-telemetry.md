---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:c3133c022b917626c396ab0cd4ee45cad001cecda192bc71b02f4d714f4c34cc
inputs:
- 'event_uri: Token lógico EventUri resuelto por EventManagerPort (eda://{family}/{event_id})'
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
  id: fan_out_telemetria
porcentaje_de_exito: null
uuid: b2c3d4e5-f6a7-4890-b1c2-d3e4f5a6b7c8
version: 2.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# route-telemetry

Enrutador del bus fractal **telemetry**. Despacha `Raw_Execution_Finished` hacia `telemetry-batch-stub` (Fase 3) / Radamanto (Fase 4).

Implementación: `route_fractal_event_core.route_telemetry_event()`.
