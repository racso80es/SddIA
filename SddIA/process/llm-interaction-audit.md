---
context: quality-assurance, filesystem-ops, tracker-operations
contract: process-contract v1.4.0
hash_signature: "sha256:f92eb79325fe2096806819444e900287bf0319e712d8df3f0093c67e24a559bc"
inputs:
- tracker_ref: Identificador opaco AUT-n
- operation: record_interaction | record_friction | probe_active
- timestamp: date-time (record_interaction)
- rol: actor
- tipo: user_query | tool_call | ...
- resumen: una línea
name: llm-interaction-audit
outputs:
- written: boolean o skipped con reason
- path: ruta relativa del audit md
phases:
- intent: 'Handler nativo: record_interaction, record_friction, probe_active. No escribe si llm_audit inactive.'
  name: Registro
uuid: 21d1c95c-e4e3-41cf-b225-c323459b58ca
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# llm-interaction-audit

Registro opt-in de interacciones LLM y fricción de ./sddia-run.sh bajo paths.auditsPath/{tracker_ref}.md cuando llm_audit: active en el ítem Linear.
