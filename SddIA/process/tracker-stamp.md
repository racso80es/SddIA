---
context:
- tracker-operations
contract: process-contract v1.4.0
hash_signature: "sha256:aa3cb70e265d665c837dd06ea6ae4400d0fcd7196d465be096d5402b88ddfcce"
inputs:
- event_file_path: Ruta relativa al JSON ECST
minteo_maximo: null
name: tracker-stamp
outputs:
- stamped: boolean
phases:
- delegates_to:
  - tool:linear-tracker-adapter
  intent: Transición de estado y/o comentario según tipo de evento; fail-soft con Tracker_Sync_Failed.
  name: Sello Linear
porcentaje_de_exito: null
uuid: f2a3b4c5-d6e7-4890-a123-456789abcdef
version: 1.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-stamp

Suscriptor táctico de señales de dominio → Linear. Ciego al markdown. Fail-soft D5.
