---
context:
- tracker-operations
contract: process-contract v1.4.0
hash_signature: "sha256:c8529fa0afec304e9b79fc9f9ae6bf9f3ff3b8e2278412cc45049adb791470ec"
inputs:
- event_file_path: Ruta relativa al JSON ECST
minteo_maximo: null
name: tracker-stamp
outputs:
- stamped: boolean
phases:
- delegates_to:
  - tool:linear-tracker-adapter
  intent: Transición de estado y/o comentario según evento (todo, cancelled, trunk_direct); fail-soft Tracker_Sync_Failed.
  name: Sello Linear
porcentaje_de_exito: null
uuid: f2a3b4c5-d6e7-4890-a123-456789abcdef
version: 1.1.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-stamp

Suscriptor táctico de señales de dominio → Linear. Ciego al markdown. Fail-soft D5.
