---
context:
- tracker-operations
- filesystem-ops
- source-control
contract: process-contract v1.4.0
hash_signature: "sha256:10bbcbf48e4b5c3a7bd30883f28f0b84e20bf0be7f747d4863f1ac3cb68ff25c"
inputs:
- event_file_path: Ruta relativa al JSON ECST Tracker_Issue_Changed en pending
minteo_maximo: null
name: tracker-markdown-apply
outputs:
- applied: boolean
- markdown_path: Ruta relativa del markdown actualizado
phases:
- delegates_to:
  - skill:git-manager
  intent: Commit en rama tracker-sync/{project_slug}; prohibido default_branch.
  name: Persistencia tracker-sync
porcentaje_de_exito: null
uuid: b7c8d9e0-f123-4567-89ab-cdef01234567
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-markdown-apply

Suscriptor de `Tracker_Issue_Changed`. Handler nativo (sin LLM). Réplica issue → markdown local; fail-soft con `Tracker_Sync_Failed` si divergencia.
