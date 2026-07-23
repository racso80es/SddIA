---
context:
- chaos-engineering
- quality-assurance
- event-routing
contract: process-contract v1.4.0
hash_signature: sha256:d76a759b6bba2eec181d4527c4cbfa1ace866518ab8593effbdfe594d52dbdbe
inputs: []
minteo_maximo: null
name: audit-telemetry-compliance-breach
outputs:
- breach_event_path: Ruta relativa al JSON domain Telemetry_Compliance_Breached
phase_invocations:
- invocations:
  - capsule: tool:schema-corruptor
    on_error: abort
    stdin_json:
      corruption_mode: empty
  phase_name: Estímulo alucinación recibo
phases:
- delegates_to:
  - agent:tekton
  - tool:schema-corruptor
  intent: Ejecutar schema-corruptor sin recibo válido.
  name: Estímulo alucinación recibo
  requires_capability:
  - contract: qa.probe
    id: qa:probe
    version: '>=1.0.0'
- delegates_to:
  - agent:argos
  intent: Verificar Telemetry_Compliance_Breached en ./.events/domain/.
  name: Certificación Argos
porcentaje_de_exito: null
uuid: fd2f075c-5d01-4b54-8b26-67678417e22b
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# audit-telemetry-compliance-breach

Proceso audit atómico **Fase 2 Caos**: ejecuta `schema-corruptor` y certifica emisión de `Telemetry_Compliance_Breached` tras fan-out `telemetry-compliance-audit`.
