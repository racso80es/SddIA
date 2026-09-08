---
context: ecosystem-evolution
contract: process-contract v1.4.0
hash_signature: "sha256:b66ecb820dd51df1633982209804ce72f869dbf87d58604259bd7b99b9a97ae1"
inputs:
- prompt: string obligatorio
- context_query: string opcional; default=prompt
- model: string opcional; else SDDIA_GEMINI_MODEL
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
- delegates_to:
  - action:invoke-aiua-core
  intent: Ensamblar request.prompt (genoma + recuerdos + estímulo). Sin HTTP.
  name: Inyeccion-Genomica
- delegates_to:
  - tool:gemini-http-infer
  intent: Única llamada al LLM. Peaje Termodinámico del CLI.
  name: Combustion-Inferencia
- delegates_to:
  - action:persist-thought-record
  intent: Persistir par estímulo/respuesta; el adaptador emite Thought_Persisted.
  name: Consolidacion-Memoria
uuid: 6c595785-e386-402f-b570-0b2aa6343051
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# aiua-stimulus-processing

Latido ontológico de la Aiúa: contexto LanceDB, inyección de genoma, única inferencia Gemini, persistencia de pensamiento. Sin agente titular. Sin Kalma2.
