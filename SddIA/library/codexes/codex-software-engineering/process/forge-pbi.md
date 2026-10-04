---
context:
- knowledge-management
- filesystem-ops
- tracker-operations
contract: process-contract v1.4.0
hash_signature: sha256:0505b3e9f01aff170f7e397aaf7218dd732bcdde3124c1c26cf468396aed86fd
inputs:
- raw_idea: Idea en bruto
- project_slug: Slug en índice Core
- process: feature | bug-fix | refactorization
- historia_ref: Ruta HU en historias/ (tracker_ref padre)
- tipo: 'Opcional: kaizen | deuda | spike (además de process bug-fix → fix)'
name: forge-pbi
outputs:
- artifact_path: PBI sellado bajo docs_layout.todos_pending del proyecto
- event_path: Ruta del evento PBI_Forged
phases:
- intent: Mayeuta expande la idea (tier reflexivo en contrato de agente).
  name: Recepcion
- intent: tool:linear-tracker-adapter create_issue (labels pbi+tipo, parent_ref historia_ref); handler nativo sin LLM.
  name: Registro en tracker
- intent: Argos sella pending con tracker_ref si aplica (tier balístico).
  name: Sellado
uuid: c4e8a1b2-7d3f-4a96-8c15-2f6e9b0d4a71
version: 1.1.3
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# forge-pbi

Línea de montaje de PBI del códice `codex-software-engineering` (alias `codex-agile-forge`). El handler nativo sella el archivo y emite `PBI_Forged` cuando no hay runtime LLM. Los tiers viven en esta definición, no en nombres de modelo.
