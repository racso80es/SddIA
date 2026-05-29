---
uuid: "fd2f075c-5d01-4b54-8b26-67678417e22b"
name: audit-telemetry-compliance-breach
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- chaos-engineering
- quality-assurance
- event-routing
hash_signature: sha256:7f26bdd8cbece98b2e4fa3f175d425ed9ddc6017340939be9ae9e2b72caaf985
inputs: []
outputs:
- breach_event_path: Ruta relativa al JSON domain Telemetry_Compliance_Breached
phases:
- name: Estímulo alucinación recibo
  intent: Ejecutar schema-corruptor sin recibo válido.
  delegates_to:
  - agent:tekton
  - tool:schema-corruptor
- name: Certificación Argos
  intent: Verificar Telemetry_Compliance_Breached en ./.events/domain/.
  delegates_to:
  - agent:argos
phase_invocations:
- phase_name: Estímulo alucinación recibo
  invocations:
  - capsule: tool:schema-corruptor
    stdin_json:
      corruption_mode: empty
    on_error: abort
minteo_maximo: null
porcentaje_de_exito: null
---

# audit-telemetry-compliance-breach

Proceso audit atómico **Fase 2 Caos**: ejecuta `schema-corruptor` y certifica emisión de `Telemetry_Compliance_Breached` tras fan-out `telemetry-compliance-audit`.
