---
context:
- event-routing
- ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:28aaa21d0cddc7ab0fab779ea43e2a3af8f1e7b90090440846283f8dff2cfc53
inputs:
- event_file_path: Ruta relativa al JSON en ./.events/domain/
minteo_maximo: null
name: route-domain
outputs:
- success: boolean
- delivery_status: mapa subscriber_id → status
phases:
- delegates_to:
  - agent:cumulo
  intent: Cargar event-domain-subscriptions.json y despachar suscriptores sobre instancias en ./.events/domain/.
  name: Fan-out dominio fractal
  requires_capability:
  - contract: bus.route
    id: bus:route
    version: '>=1.0.0'
porcentaje_de_exito: null
uuid: d4e5f6a7-b8c9-4012-d3e4-f5a6b7c8d9e0
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# route-domain

Enrutador del bus fractal **domain** (coexiste con `route-domain-event` sobre `pending/` legacy — D0.2).

Implementación: `route_fractal_event_core.route_domain_fractal_event()`.
