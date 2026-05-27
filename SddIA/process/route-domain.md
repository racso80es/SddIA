---
uuid: "d4e5f6a7-b8c9-4012-d3e4-f5a6b7c8d9e0"
name: route-domain
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- event-routing
- ecosystem-evolution
hash_signature: sha256:e8f0f9e9ae8384276bc8d57f0fb0b447781b67cb365211c8e152c974333c9ab6
inputs:
- event_file_path: Ruta relativa al JSON en ./.events/domain/
outputs:
- success: boolean
- delivery_status: mapa subscriber_id → status
phases:
- name: Fan-out dominio fractal
  intent: Cargar event-domain-subscriptions.json y despachar suscriptores sobre instancias en ./.events/domain/.
  delegates_to:
  - agent:cumulo
minteo_maximo: null
porcentaje_de_exito: null
---

# route-domain

Enrutador del bus fractal **domain** (coexiste con `route-domain-event` sobre `pending/` legacy — D0.2).

Implementación: `route_fractal_event_core.route_domain_fractal_event()`.
