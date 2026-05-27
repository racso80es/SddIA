---
uuid: "f1e2d3c4-b5a6-4789-8c0d-1e2f3a4b5c6d"
name: telemetry-batch-stub
version: "1.0.0"
deprecated: true
superseded_by: radamanto-batch
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- event-routing
- quality-assurance
hash_signature: sha256:c612daaefbf199fefe9018f7264178a7f5687795b55e2e5b57d992147ed60d55
inputs:
- event_file_path: Ruta relativa al JSON de telemetría en ./.events/telemetry/
outputs:
- consumed: Indica que el evento fue leído y purgado (stub Radamanto Fase 3)
phases:
- name: Consumo batch stub
  intent: Leer instancia Raw_Execution_Finished, registrar consumo y purgar archivo fuente (simula Radamanto Fase 4).
  delegates_to:
  - skill:filesystem-manager
minteo_maximo: null
porcentaje_de_exito: null
---

# telemetry-batch-stub

Proceso laboratorio **stub** para suscripción telemetría → Radamanto (Fase 3). Sustituido por agente Radamanto real en Fase 4.
