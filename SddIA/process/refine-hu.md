---
context:
- knowledge-management
- filesystem-ops
- tracker-operations
contract: process-contract v1.4.0
hash_signature: "sha256:636356b4f7c45681de8707ffb0ca354389aca82024180c6a87661b21b27c36cd"
inputs:
- hu_ref: Referencia opaca de la historia en el gestor activo
- project_slug: Slug registrado
- tracker_ref: Identificador opaco devuelto por tracker:lifecycle
name: refine-hu
outputs:
- hu_ref: Referencia opaca sellada
- tracker_ref: Identificador opaco de tracker:lifecycle
- event_path: JSON en eda_bus.pending
phases:
- intent: Refina la copia de trabajo de la historia.
  name: Refinamiento Mayeuta
- intent: Registro en Tracker. Un único create_issue vía tracker:lifecycle si falta tracker_ref. Etiqueta de historia del manifiesto.
  name: Registro en Tracker
  requires_capability:
  - id: tracker:lifecycle
    contract: tracker.lifecycle
    version: '>=1.0.0'
- intent: status refinada, tracker_ref opcional; emite HU_Refined.
  name: Sellado Argos
uuid: 417025aa-6c8f-49fb-a4e7-d32efe9c896b
version: 1.0.3
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# refine-hu

Refinamiento pre-forja. `hu_ref` es referencia opaca. `tracker_ref` es el identificador opaco de `tracker:lifecycle`. La fase de alta es `Registro en Tracker` y llama un solo `create_issue`. Sella refinada y emite HU_Refined.
