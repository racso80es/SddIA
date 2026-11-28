---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: "sha256:69a24c02347ea433fc65d1e3bef2b0b822e1a3a09c9a5572da0ed55347bb582b"
inputs:
- 'event_uri: Token lógico EventUri resuelto por EventManagerPort (eda://{family}/{event_id})'
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
  id: fan_out_orquestacion
porcentaje_de_exito: null
uuid: c3d4e5f6-a7b8-4901-c2d3-e4f5a6b7c8d9
version: 2.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# route-orchestration

## Propósito

Enrutador de la familia **orchestration**: entrega `Process_Execution_Completed` y eventos tácticos de línea de montaje a suscriptores de `event-orchestration-subscriptions.json`.

## Orquestación

- Fase `Fan-out orquestación` con capability `bus:route` y delegación `agent:cumulo`.
- Handler: `route_fractal_event_core.route_orchestration_event()` en `route_fractal_core.rs`.
- UUID legacy `c3d4e5f6-…` pendiente de migración v4 (AC-3); contenido independiente del peaje UUID.
