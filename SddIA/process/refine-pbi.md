---
context:
- knowledge-management
- filesystem-ops
- tracker-operations
contract: process-contract v1.4.0
hash_signature: "sha256:a4b8be4ddb9a6c1de5d53011c71cf1d9f0a403bc1743b2c400c3e4157a6b36b4"
inputs:
- pbi_ref: Referencia opaca del PBI en el gestor activo
- project_slug: Slug registrado en .SddIA/projects
name: refine-pbi
outputs:
- pbi_ref: Referencia opaca sellada
- event_path: JSON en eda_bus.pending
phases:
- intent: Refina el cuerpo del PBI en la copia de workspace. La persistencia definitiva la hace tracker:lifecycle. No toca persist_ref.
  name: Refinamiento Mayeuta
- intent: Frontmatter status refinado y refined; emite PBI_Refined con source_process refine-pbi.
  name: Sellado Argos
  requires_capability:
  - id: tracker:lifecycle
    contract: tracker.lifecycle
    version: '>=1.0.0'
uuid: d5ecaaed-ba8d-45a4-85f4-a1d7d1d4308a
version: 1.0.2
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# refine-pbi

Refinamiento pre-forja. `pbi_ref` es referencia opaca. El sello opera sobre la copia de workspace y la persistencia va por `tracker:lifecycle`. Emite PBI_Refined.
