---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: sha256:38fc4d27e9dfb4a1ccafc2f05f7175c736ef1996979fe7b3c9484579e45ae70f
inputs:
- 'prompt: string obligatorio'
- 'context_query: string opcional; default=prompt'
- 'model: string opcional; override de oráculo del registro'
- 'effort: string opcional; default=high'
name: aiua-stimulus-processing
outputs:
- thought_id: string node_id SHA-256
- response: string result.text
- telemetry: object duration_ms model tokens?
phases:
- delegates_to:
  - action:retrieve-active-context
  intent: Recuerdos KNN; memories vacío no es error.
  name: Triaje-Contexto
  id: triaje_contexto
- delegates_to:
  - action:invoke-aiua-core
  intent: Ensamblar request.prompt (genoma + recuerdos + estímulo). Sin HTTP.
  name: Inyeccion-Genomica
  id: inyeccion_genomica
- delegates_to:
  - tool:llm-router
  intent: 'Única llamada al LLM vía router de oráculos de instancia (affinity: aiua); Peaje Termodinámico del adaptador efectivo.'
  name: Combustion-Inferencia
  id: combustion_inferencia
- delegates_to:
  - action:dispatch-aiua-intent
  intent: Si hay tendón motor, traducir a ECST fractal domain. Cero join a TQM.
  name: Despacho-Motor
  id: despacho_motor
- delegates_to:
  - action:persist-thought-record
  intent: Persistir par estímulo/respuesta; el adaptador emite Thought_Persisted.
  name: Consolidacion-Memoria
  id: consolidacion_memoria
uuid: 6c595785-e386-402f-b570-0b2aa6343051
version: 1.3.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# aiua-stimulus-processing

Latido ontológico de la Aiúa: contexto LanceDB, inyección de genoma, combustión vía tool:llm-router (affinity aiua; oráculos de instancia), despacho motor EDA opcional (action:dispatch-aiua-intent → eda_fractal.domain), persistencia de pensamiento. Sin agente titular. Sin Kalma2. Cero join a TQM.
