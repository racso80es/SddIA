---
context:
- event-routing
- quality-assurance
contract: process-contract v1.4.0
deprecated: true
hash_signature: sha256:7f7cc081acf3327b99382eca446625f04a7137fc784ce38e31ee085283e4a0bc
inputs:
- event_file_path: Ruta relativa al JSON de telemetría en ./.events/telemetry/
minteo_maximo: null
name: telemetry-batch-stub
outputs:
- consumed: Indica que el evento fue leído y purgado (stub Radamanto Fase 3)
phases:
- intent: Leer instancia Raw_Execution_Finished, registrar consumo y purgar archivo fuente (simula Radamanto Fase 4).
  name: Consumo batch stub
  requires_capability:
  - contract: fs.persist
    id: fs:persist
    version: '>=1.0.0'
porcentaje_de_exito: null
superseded_by: radamanto-batch
uuid: f1e2d3c4-b5a6-4789-8c0d-1e2f3a4b5c6d
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# telemetry-batch-stub

Proceso laboratorio **stub** para suscripción telemetría → Radamanto (Fase 3). Sustituido por agente Radamanto real en Fase 4.
