---
context:
- event-routing
- quality-assurance
contract: process-contract v1.4.0
hash_signature: sha256:87881548d653d0fe182faecf5481c0e622fdbc981c9c8af617857a4cfc449c11
inputs:
- event_file_path: Ruta relativa al JSON de telemetría en ./.events/telemetry/
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
porcentaje_de_exito: null
uuid: b3c4d5e6-f7a8-4901-b2c3-d4e5f6a7b8c9
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# telemetry-compliance-audit

Proceso laboratorio Fase 5: auditoría asíncrona de cumplimiento termodinámico. Consume `Raw_Execution_Finished` en fan-out paralelo a `radamanto-batch`. **No purga** el JSON fuente — solo sella `delivery_state`.
