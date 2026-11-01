---
context: knowledge-management,filesystem-ops,tracker-operations
contract: process-contract v1.4.0
hash_signature: "sha256:b270f0de993c4ad545402a148522fca429ecb8c84c6aacd90980968021d176c9"
name: forge-hu
phases:
- intent: Validar raw_idea y destilar titulo.
  name: Recepcion
- intent: create_issue hu sin parent_ref via tracker:lifecycle.
  name: Registro en tracker
- intent: Emitir HU_Forged en bus.
  name: Sellado
uuid: 3c2ede30-9654-45da-91c4-0b7ffc4df59b
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# forge-hu

Alta de HU raíz en tracker (labels hu, sin parent_ref). Cierra hueco AUT-19 slice 2 / AUT-50.
