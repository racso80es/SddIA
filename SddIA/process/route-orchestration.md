---
context:
- event-routing
- ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:e0801b944925b2c3a2adfa4f148981d11f07e705f5996412b184f26ab4bddf3b
inputs:
- event_file_path: Ruta relativa al JSON en ./.events/orchestration/
minteo_maximo: null
name: route-orchestration
outputs:
- success: boolean
- delivery_status: mapa subscriber_id → status
phases:
- delegates_to:
  - agent:cumulo
  intent: Cargar event-orchestration-subscriptions.json y despachar suscriptores de línea de montaje.
  name: Fan-out orquestación
  requires_capability:
  - contract: bus.route
    id: bus:route
    version: '>=1.0.0'
porcentaje_de_exito: null
uuid: c3d4e5f6-a7b8-4901-c2d3-e4f5a6b7c8d9
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# route-orchestration

Enrutador del bus fractal **orchestration**. Consume `Process_Execution_Completed` y eventos tácticos futuros.

Implementación: `route_fractal_event_core.route_orchestration_event()`.
