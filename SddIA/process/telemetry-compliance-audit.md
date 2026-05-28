---
uuid: "b3c4d5e6-f7a8-4901-b2c3-d4e5f6a7b8c9"
name: telemetry-compliance-audit
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- event-routing
- quality-assurance
hash_signature: sha256:d7f2478df454d23763aa512507150ac6cd134973180d2353cea1b99622019019
inputs:
- event_file_path: Ruta relativa al JSON de telemetría en ./.events/telemetry/
outputs:
- audit_result: Resultado cruce recibo vs contrato ED
phases:
- name: Auditoría cumplimiento termodinámico
  intent: Cruzar telemetry_receipt vs contrato ED; emitir dominio breach si aplica; sellar delivery_state (T5.6 — sin borrado físico).
  delegates_to:
  - agent:argos
minteo_maximo: null
porcentaje_de_exito: null
---

# telemetry-compliance-audit

Proceso laboratorio Fase 5: auditoría asíncrona de cumplimiento termodinámico. Consume `Raw_Execution_Finished` en fan-out paralelo a `radamanto-batch`. **No purga** el JSON fuente — solo sella `delivery_state`.
