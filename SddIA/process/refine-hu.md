---
context: knowledge-management, filesystem-ops, tracker-operations
contract: process-contract v1.4.0
hash_signature: sha256:b8b61105318a865e8bc3e66e17a84e0078dc6e37bc6c9a6e52bb58bd99814867
inputs:
- hu_ref: Ruta relativa bajo historias/
- project_slug: Slug registrado
name: refine-hu
outputs:
- hu_ref: Ruta sellada
- tracker_ref: Issue Linear si aplica
- event_path: JSON en eda_bus.pending
phases:
- intent: Refina markdown en docs/todos/historias/.
  name: Refinamiento Mayeuta
- intent: tool:linear-tracker-adapter create_issue con label hu si no hay tracker_ref y hay tracker en manifiesto.
  name: Registro Linear
- intent: status refinada, tracker_ref opcional; emite HU_Refined.
  name: Sellado Argos
uuid: 417025aa-6c8f-49fb-a4e7-d32efe9c896b
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# refine-hu

Refinamiento pre-forja de HU en historias/; create_issue si falta tracker_ref; sella refinada y emite HU_Refined.
