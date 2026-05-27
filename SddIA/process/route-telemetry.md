---
uuid: "b2c3d4e5-f6a7-4890-b1c2-d3e4f5a6b7c8"
name: route-telemetry
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- event-routing
- ecosystem-evolution
hash_signature: sha256:77a8ce73b9bac1e703b7f705699bdfaa8cf4889362ed132a1f11cd9a7c79c9e6
inputs:
- event_file_path: Ruta relativa al JSON en ./.events/telemetry/
outputs:
- success: boolean
- delivery_status: mapa subscriber_id → status
phases:
- name: Fan-out telemetría
  intent: Cargar event-telemetry-subscriptions.json y despachar suscriptores (stub Radamanto).
  delegates_to:
  - agent:cumulo
minteo_maximo: null
porcentaje_de_exito: null
---

# route-telemetry

Enrutador del bus fractal **telemetry**. Despacha `Raw_Execution_Finished` hacia `telemetry-batch-stub` (Fase 3) / Radamanto (Fase 4).

Implementación: `route_fractal_event_core.route_telemetry_event()`.
