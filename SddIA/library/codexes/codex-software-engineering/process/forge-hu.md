---
context: knowledge-management,filesystem-ops,tracker-operations
contract: process-contract v1.4.0
hash_signature: "sha256:c4c3ad4b344a52847b0f6c06096951962fda1a0c49fd3d2e40add65aad511f07"
name: forge-hu
phases:
- intent: Validar raw_idea y destilar titulo.
  name: Recepcion
  id: recepcion
- intent: create_issue hu sin parent_ref via tracker:lifecycle.
  name: Registro en tracker
  id: registro_en_tracker
- intent: Emitir HU_Forged en bus.
  name: Sellado
  id: sellado
uuid: 3c2ede30-9654-45da-91c4-0b7ffc4df59b
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# forge-hu

## Propósito

Alta de HU raíz en tracker (labels hu, sin parent_ref). Cierra hueco AUT-19 slice 2 / AUT-50.

## Operación

Invocación vía `./sddia-run.sh --process forge-hu`; fases en frontmatter YAML.
