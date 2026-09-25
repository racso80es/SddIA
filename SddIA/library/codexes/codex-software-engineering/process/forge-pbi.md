---
uuid: c4e8a1b2-7d3f-4a96-8c15-2f6e9b0d4a71
name: forge-pbi
version: 1.0.0
contract: process-contract v1.4.0
workspace_template: ".SddIA/workspaces/{process_name}/{execution_id}/"
context:
- knowledge-management
- filesystem-ops
inputs:
- raw_idea: Idea en bruto del Vértice Biológico
- project_slug: Proyecto registrado en el índice Core
- process: feature | bug-fix | refactorization
outputs:
- artifact_path: PBI sellado bajo docs_layout.todos_pending del proyecto
- event_path: Ruta del evento PBI_Forged
phases:
- name: Recepcion
  intent: "Mayeuta expande la idea. Tier reflexivo declarado en el contrato de agente. Prohibido nombre de modelo literal."
  delegates_to:
  - agent:mayeuta
- name: Sellado
  intent: "Argos sella el formato en pending del proyecto. Tier balistico. Ceguera espacial: no inventa negocio."
  delegates_to:
  - agent:argos
---

# forge-pbi

Línea de montaje de PBI del códice `codex-software-engineering` (alias `codex-agile-forge`). El handler nativo sella el archivo y emite `PBI_Forged` cuando no hay runtime LLM. Los tiers viven en esta definición, no en nombres de modelo.
