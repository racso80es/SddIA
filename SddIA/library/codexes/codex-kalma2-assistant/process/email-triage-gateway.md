---
context:
- external-ingest
contract: process-contract v1.4.0
hash_signature: "sha256:794696a90e36910a64bc35863695b77ad8a54f0381b552dc4a0333b571a35366"
inputs:
- event_file_path: Ruta de la instancia ECST Email_Received
name: email-triage-gateway
outputs:
- verdict: noise | passive | actionable
- decision_path: deterministic | llm
- emitted: true si se escribió Email_Triaged
phases:
- delegates_to:
  - skill:user-preference-store
  intent: Consulta opt-in memory:pref-query (subject_key hash del remitente); fail-open a bloque vacio versionado. Exencion C y mute se evaluan aqui; la query no es Clasificacion.
  name: Triaje-P
  requires_capability:
  - contract: memory.pref_query
    id: memory:pref-query
    version: '>=1.0.0'
- intent: Reglas deterministas de email-triage-matrix; salida temprana en noise sin gasto de inferencia. Omitido si P-EXEMPT-C.
  name: Triaje-C
- intent: Clasificacion semantica solo para correo no resuelto por Triaje-C ni mute P. Prompt + user_preference_context si hay habitos parciales.
  name: Clasificacion
  requires_capability:
  - contract: llm.interact
    id: llm:interact
    version: '>=1.0.0'
- intent: Extraccion estructurada y asiento local; solo via actionable.
  name: Asiento-Agenda
  requires_capability:
  - contract: agenda.persist
    id: agenda:persist
    version: '>=1.0.0'
- intent: Escritura de Email_Triaged en eda_fractal.domain con veredicto y coste.
  name: Emision
uuid: 9cb9a63a-bb86-4b97-8a75-4dac2f2cb5ce
version: 1.1.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# email-triage-gateway

Aduana cognitiva del canal aferente de correo. Triaje-P (memory:pref-query) + muro Triaje-C + mute P + Clasificacion LLM condicionada; asiento de agenda; emision Email_Triaged. `decision_path`: `deterministic` | `llm` | `preference` (quien cerro).

**Gate G5:** si Triaje-C concluye o mute P cierra (`P-MUTE-SENDER`), la fase `Clasificacion` no se ejecuta (`skipped`, `reason: triaje-c-concluded` | `p-mute-sender`). Query P y `P-EXEMPT-C` no son Clasificacion. Handler nativo en `execute-process`.
