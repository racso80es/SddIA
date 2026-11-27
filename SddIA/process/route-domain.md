---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:6f360c40d9973b96de1f565e535b7fa41dd75b69451def18ea1707ecebef41bb
inputs:
- 'event_uri: Token lógico EventUri resuelto por EventManagerPort (eda://{family}/{event_id})'
minteo_maximo: null
name: route-domain
outputs:
- success: boolean
- delivery_status: mapa subscriber_id → status
phases:
- delegates_to:
  - agent:cumulo
  intent: Cargar event-domain-subscriptions.json y despachar suscriptores sobre instancias vía EventManagerPort (familia domain).
  name: Fan-out dominio fractal
  requires_capability:
  - contract: bus.route
    id: bus:route
    version: '>=1.0.0'
  id: fan_out_dominio_fractal
porcentaje_de_exito: null
uuid: d4e5f6a7-b8c9-4012-d3e4-f5a6b7c8d9e0
version: 2.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# route-domain

Enrutador del bus fractal **domain** (coexiste con `route-domain-event` sobre `pending/` legacy — D0.2).

Implementación: `route_fractal_event_core.route_domain_fractal_event()`.
