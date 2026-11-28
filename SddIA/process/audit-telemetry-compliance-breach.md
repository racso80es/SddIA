---
context:
- chaos-engineering
- quality-assurance
- event-routing
contract: process-contract v1.4.0
hash_signature: "sha256:3908d05da912dc3386c96f23e15f38fc63319e79123da563ff1f8bd813a08b70"
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
  id: estimulo_alucinacion_recibo
- delegates_to:
  - agent:argos
  intent: Verificar Telemetry_Compliance_Breached por EventUri en EventManagerPort.
  name: Certificación Argos
  id: certificacion_argos
porcentaje_de_exito: null
uuid: fd2f075c-5d01-4b54-8b26-67678417e22b
version: 2.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# audit-telemetry-compliance-breach

## Propósito

Proceso audit atómico **Fase 2 Caos**: ejecuta `schema-corruptor` y certifica emisión de `Telemetry_Compliance_Breached` tras fan-out `telemetry-compliance-audit`.

## Operación

Invocación vía `./sddia-run.sh --process <name>`; fases en frontmatter YAML.
