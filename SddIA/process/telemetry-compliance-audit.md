---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:3992bbfd29f26e35be28e8fdd4e514ebf59e22f5cc30011866a7902de69154bf
inputs:
- 'event_uri: Token lógico EventUri resuelto por EventManagerPort (eda://{family}/{event_id})'
minteo_maximo: null
name: telemetry-compliance-audit
outputs:
- audit_result: Resultado cruce recibo vs contrato ED
phases:
- delegates_to:
  - agent:argos
  intent: Cruzar telemetry_receipt vs contrato ED; emitir dominio breach si aplica; sellar delivery_state (T5.6 — sin borrado físico).
  name: Auditoría cumplimiento termodinámico
  requires_capability:
  - contract: audit.compliance
    id: audit:compliance
    version: '>=1.0.0'
  id: auditoria_cumplimiento_termodinamico
porcentaje_de_exito: null
uuid: b3c4d5e6-f7a8-4901-b2c3-d4e5f6a7b8c9
version: 2.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# telemetry-compliance-audit

Proceso laboratorio Fase 5: auditoría asíncrona de cumplimiento termodinámico. Consume `Raw_Execution_Finished` en fan-out paralelo a `radamanto-batch`. **No purga** el JSON fuente — solo sella `delivery_state`.
