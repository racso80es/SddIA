---
context:
- tracker-operations
contract: process-contract v1.4.0
hash_signature: "sha256:2cf44164ccf886d6c532098eb7d96a032f787a14d6bb2d8d7fb5bccdc0a1692c"
inputs:
- event_file_path: Ruta relativa al JSON ECST
minteo_maximo: null
name: tracker-stamp
outputs:
- stamped: boolean
phases:
- intent: Transición de estado y/o comentario según evento (todo, cancelled, trunk_direct). Con provider file no hay cápsula. Fail-soft Tracker_Sync_Failed.
  name: Sello Linear
  requires_capability:
  - id: tracker:lifecycle
    contract: tracker.lifecycle
    version: '>=1.0.0'
porcentaje_de_exito: null
uuid: f2a3b4c5-d6e7-4890-a123-456789abcdef
version: 1.1.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-stamp

Suscriptor táctico de señales de dominio → Linear. Ciego al markdown. Fail-soft D5.
