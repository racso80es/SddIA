---
context:
- knowledge-management
- filesystem-ops
- tracker-operations
contract: process-contract v1.4.0
hash_signature: sha256:9191c471d5f5af6021d30481dc1f868c1196d71d8650e64492718d951e41f4d4
inputs:
- raw_idea: Idea en bruto
- project_slug: Slug en índice Core
- process: feature | bug-fix | refactorization
- historia_ref: tracker_ref padre, identificador opaco de tracker:lifecycle
- tipo: 'Opcional: kaizen | deuda | spike (además de process bug-fix → fix)'
name: forge-pbi
outputs:
- artifact_path: Referencia opaca del alta devuelta por tracker:lifecycle
- event_path: Ruta del evento PBI_Forged
phases:
- intent: Mayeuta expande la idea (tier reflexivo en contrato de agente).
  name: Recepcion
  id: recepcion
- intent: Registro en Tracker. Un único create_issue vía tracker:lifecycle (labels pbi+tipo, parent_ref = tracker_ref padre). El proveedor resuelto devuelve tracker_ref.
  name: Registro en Tracker
  requires_capability:
  - id: tracker:lifecycle
    contract: tracker.lifecycle
    version: '>=1.0.0'
  id: registro_en_tracker
- intent: Argos sella el ítem con tracker_ref si aplica (tier balístico).
  name: Sellado
  id: sellado
uuid: c4e8a1b2-7d3f-4a96-8c15-2f6e9b0d4a71
version: 1.0.2
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# forge-pbi

Línea de montaje de PBI del códice `codex-software-engineering` (alias `codex-agile-forge`). El handler nativo sella el archivo y emite `PBI_Forged` cuando no hay runtime LLM. Los tiers viven en esta definición, no en nombres de modelo.
