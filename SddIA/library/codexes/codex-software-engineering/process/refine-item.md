---
context:
- knowledge-management
- filesystem-ops
- tracker-operations
contract: process-contract v1.4.0
hash_signature: "sha256:39b03aef55a3c7c1c17a9e0f75a56d02c4b6743ce28c5ba60eafe8d0b1758ee4"
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
  id: preparacion
- intent: invoke_agent_phase; verdict.json e item.refined.md.
  name: Refinamiento Mayeuta
  id: refinamiento_mayeuta
- intent: dor.rs por item_type antes de sellado.
  name: Validación DoR
  id: validacion_dor
- intent: 'Sustrato remoto: tracker_ref desde snap.identifier si falta en frontmatter; status conflict si FM diverge. Sin create_issue.'
  name: Registro en Tracker
  requires_capability:
  - contract: tracker.lifecycle
    id: tracker:lifecycle
    version: '>=1.0.0'
  id: registro_en_tracker
- intent: update_issue_description, refinement refined, emite PBI_Refined o HU_Refined con source_process refine-item.
  name: Sellado Argos
  id: sellado_argos
uuid: bb8e8704-80fa-4523-9cec-043a63fe0499
version: 1.1.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# refine-item

Refinamiento unificado de ítems de backlog: preparación YAML, Mayeuta, DoR, sellado tracker_ref desde issue leído (sin alta remota), sellado y evento PBI_Refined/HU_Refined. Salida: `event_uri` / sello vía `EventManagerPort` (`events-contract.md` §4).

## Propósito

Refinamiento unificado de ítems de backlog: preparación YAML, Mayeuta, DoR, sellado tracker_ref desde issue leído (sin alta remota), sellado y evento PBI_Refined/HU_Refined. Salida: `event_uri` / sello vía `EventManagerPort` (`events-contract.md` §4).

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
