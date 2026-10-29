---
context:
- knowledge-management
- filesystem-ops
- tracker-operations
contract: process-contract v1.4.0
hash_signature: sha256:3c0fd28bb21e6e4fdd6908f452cf3ad694a86e9abca2121235c1bfb20a5c0e34
inputs:
- item_ref: Referencia opaca del ítem en el gestor activo
- project_slug: Slug registrado
- execution_id: Opcional; workspace .SddIA/workspaces/refine-item/{execution_id}/
name: refine-item
outputs:
- item_ref: Referencia opaca sellada
- item_type: hu|pbi|fix|kaizen|deuda|spike
- tracker_ref: Opcional
- event_path: JSON en eda_bus.pending
- status: refined|needs_clarification|awaiting_agents|conflict
phases:
- intent: fetch_issue, ensure_canonical_markdown, brief.json en workspace.
  name: Preparación
- intent: invoke_agent_phase; verdict.json e item.refined.md.
  name: Refinamiento Mayeuta
- intent: dor.rs por item_type antes de sellado.
  name: Validación DoR
- intent: 'Sustrato remoto: tracker_ref desde snap.identifier si falta en frontmatter; status conflict si FM diverge. Sin create_issue.'
  name: Registro en Tracker
  requires_capability:
  - contract: tracker.lifecycle
    id: tracker:lifecycle
    version: '>=1.0.0'
- intent: update_issue_description, refinement refined, emite PBI_Refined o HU_Refined con source_process refine-item.
  name: Sellado Argos
uuid: bb8e8704-80fa-4523-9cec-043a63fe0499
version: 1.1.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# refine-item

Refinamiento unificado de ítems de backlog: preparación YAML, Mayeuta, DoR, sellado tracker_ref desde issue leído (sin alta remota), sellado y evento PBI_Refined/HU_Refined.
