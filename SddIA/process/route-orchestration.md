---
uuid: "c3d4e5f6-a7b8-4901-c2d3-e4f5a6b7c8d9"
name: route-orchestration
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- event-routing
- ecosystem-evolution
hash_signature: sha256:39cc8bc8ff2784d191ac4cc6bf0ccdcf72abaac2cbed2a71b1b55b4888c0b882
inputs:
- event_file_path: Ruta relativa al JSON en ./.events/orchestration/
outputs:
- success: boolean
- delivery_status: mapa subscriber_id → status
phases:
- name: Fan-out orquestación
  intent: Cargar event-orchestration-subscriptions.json y despachar suscriptores de línea de montaje.
  delegates_to:
  - agent:cumulo
minteo_maximo: null
porcentaje_de_exito: null
---

# route-orchestration

Enrutador del bus fractal **orchestration**. Consume `Process_Execution_Completed` y eventos tácticos futuros.

Implementación: `route_fractal_event_core.route_orchestration_event()`.
