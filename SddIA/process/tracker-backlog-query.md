---
context:
- tracker-operations
contract: process-contract v1.4.0
hash_signature: "sha256:b81ec457136633db9e57412d0dfc12922628f8aeba7835b2e7ad7bc024facaaa"
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
- delegates_to:
  - tool:linear-tracker-adapter
  intent: list_issues con filtro de label según kind; síncrono, sin LLM.
  name: Consulta backlog
porcentaje_de_exito: null
uuid: d4e5f6a7-b8c9-4d0e-a12b-3b4c5d6e7f9a
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-backlog-query

Lectura de backlog Linear para Kalma2. `context: tracker-operations`. Ejecutor de registro Tekton; runtime invoca la tool sin LLM.
