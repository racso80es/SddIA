---
context:
- tracker-operations
contract: process-contract v1.4.0
hash_signature: "sha256:5803f1e48cecae528539f3f17dddc42fcf21a948ce1f4397076cbd8bf4ac7fbb"
inputs:
- event_file_path: Ruta relativa al JSON ECST
minteo_maximo: null
name: tracker-stamp
outputs:
- stamped: boolean
phases:
- intent: Sello de ciclo. Transición de estado y/o comentario según evento (todo, cancelled, trunk_direct) vía tracker:lifecycle. Fail-soft Tracker_Sync_Failed.
  name: Sello de ciclo
  requires_capability:
  - id: tracker:lifecycle
    contract: tracker.lifecycle
    version: '>=1.0.0'
porcentaje_de_exito: null
uuid: f2a3b4c5-d6e7-4890-a123-456789abcdef
version: 1.1.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-stamp

Suscriptor táctico de señales de dominio. Invoca `tracker:lifecycle`. Ciego al markdown. Fail-soft D5.
