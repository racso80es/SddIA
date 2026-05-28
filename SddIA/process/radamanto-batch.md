---
uuid: "2a3b4c5d-6e7f-4a8b-9c0d-1e2f3a4b5c6d"
name: radamanto-batch
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- event-routing
- quality-assurance
- ecosystem-evolution
hash_signature: sha256:f8976358bbe08cbbe09ecb241cd21104608fac4a18b16b7cb39d57a039d58ab6
inputs:
- event_file_path: Ruta relativa al JSON de telemetría en ./.events/telemetry/
outputs:
- batch_result: Stats actualizados y acciones dominio emitidas
phases:
- name: Consumo batch Radamanto
  intent: Acumular telemetría CLI, evaluar umbrales, emitir dominio + DLT; sellar delivery_state (T5.6).
  delegates_to:
  - agent:radamanto
minteo_maximo: null
porcentaje_de_exito: null
---

# radamanto-batch

Proceso laboratorio del agente **Radamanto**: sustituye `telemetry-batch-stub` (Fase 4). Consume `Raw_Execution_Finished`, actualiza acumulador en `.SddIA/radamanto/` y es el **único emisor** de `Status_Restored`.
