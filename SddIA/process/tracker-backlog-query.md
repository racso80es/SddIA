---
context:
- tracker-operations
contract: process-contract v1.4.0
hash_signature: sha256:608584b6059e15d0cce17c083d374e5b322aec6a5bb486d7a5d1a35efcc843c5
inputs:
- project_slug: Slug registrado (opcional; ausente = Core-self)
- kind: hu | pbi | all (default all)
- state: Estado canónico backlog|in_progress|… (opcional)
minteo_maximo: null
name: tracker-backlog-query
outputs:
- tracker_configured: boolean
- items: Lista de issues HU/PBI
phases:
- intent: list_issues con filtro de label según kind vía tracker:lifecycle si el provider es linear. Con file lista pending y done sin cápsula.
  name: Consulta backlog
porcentaje_de_exito: null
uuid: d4e5f6a7-b8c9-4d0e-a12b-3b4c5d6e7f9a
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-backlog-query

Lectura de backlog Linear para Kalma2. `context: tracker-operations`. Ejecutor de registro Tekton; runtime invoca la tool sin LLM.
