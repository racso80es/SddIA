---
context: knowledge-management, filesystem-ops
contract: process-contract v1.4.0
hash_signature: sha256:fb15792f870602a7c69452aeb36fdf32e29f5376dd243aaec9da8790eb0737ab
inputs:
- pbi_ref: Ruta relativa del artefacto bajo todos_pending
- project_slug: Slug registrado en .SddIA/projects
name: refine-pbi
outputs:
- pbi_ref: Ruta relativa sellada
- event_path: JSON en eda_bus.pending
phases:
- intent: Refina el cuerpo del PBI en todos_pending sin tocar persist_ref ni done/.
  name: Refinamiento Mayeuta
- intent: Frontmatter status refinado y refined; emite PBI_Refined con source_process refine-pbi.
  name: Sellado Argos
uuid: d5ecaaed-ba8d-45a4-85f4-a1d7d1d4308a
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# refine-pbi

Refinamiento pre-forja del markdown PBI en todos_pending; sella status refinado y emite PBI_Refined (handler nativo Tekton).
