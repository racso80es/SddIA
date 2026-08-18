---
context:
- external-ingest
contract: process-contract v1.4.0
hash_signature: "sha256:e391b7b7c1a4f6e7c7b6919c0152bdebf83c65b43f436ddf66cf3a96a969182a"
inputs:
- event_file_path: Ruta de la instancia ECST Email_Received
name: email-triage-gateway
outputs:
- verdict: noise | passive | actionable
- decision_path: deterministic | llm
- emitted: true si se escribió Email_Triaged
phases:
- intent: Reglas deterministas de email-triage-matrix; salida temprana en noise sin gasto de inferencia.
  name: Triaje-C
- intent: Clasificación semántica solo para correo no resuelto por Triaje-C.
  name: Clasificacion
  requires_capability:
  - contract: llm.interact
    id: llm:interact
    version: '>=1.0.0'
- intent: Extracción estructurada y asiento local; solo vía actionable.
  name: Asiento-Agenda
  requires_capability:
  - contract: agenda.persist
    id: agenda:persist
    version: '>=1.0.0'
- intent: Escritura de Email_Triaged en eda_fractal.domain con veredicto y coste.
  name: Emision
uuid: 9cb9a63a-bb86-4b97-8a75-4dac2f2cb5ce
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# email-triage-gateway

Aduana cognitiva del canal aferente de correo. Triaje-C determinista con salida temprana; Clasificacion LLM condicionada; asiento de agenda; emision Email_Triaged.

**Gate G5:** si Triaje-C concluye, la fase `Clasificacion` no se ejecuta (`execution_report.phases[].status: skipped`, `reason: triaje-c-concluded`). Handler nativo en `execute-process`.
