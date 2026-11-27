---
context:
- tracker-operations
contract: process-contract v1.4.0
hash_signature: sha256:aff56386bdda8c19b1bb569c30d9dbdea6d4edefc8693ff47812e09b1a07352e
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
- intent: Consulta de backlog. Un list_issues vía tracker:lifecycle con el filtro de label según kind.
  name: Consulta de backlog
  id: consulta_de_backlog
porcentaje_de_exito: null
uuid: d4e5f6a7-b8c9-4d0e-a12b-3b4c5d6e7f9a
version: 1.0.2
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-backlog-query

Consulta de backlog para Kalma2. `context: tracker-operations`. Un `list_issues` de `tracker:lifecycle`. Ejecutor de registro Tekton; runtime invoca el puerto sin LLM.
