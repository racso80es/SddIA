---
context:
- tracker-operations
contract: process-contract v1.4.0
hash_signature: sha256:bbec62c3a2c5bb4e220b1b88388283390dc4e7466b548ac3b1be7eff7a31f03d
inputs:
- 'event_uri: Token lógico EventUri resuelto por EventManagerPort (eda://domain/{event_id})'
minteo_maximo: null
name: tracker-stamp
outputs:
- stamped: boolean
phases:
- intent: Sello de ciclo. Transición de estado y/o comentario según evento (todo, cancelled, trunk_direct) vía tracker:lifecycle. Fail-soft Tracker_Sync_Failed.
  name: Sello de ciclo
  requires_capability:
  - contract: tracker.lifecycle
    id: tracker:lifecycle
    version: '>=1.0.0'
  id: sello_de_ciclo
porcentaje_de_exito: null
uuid: f2a3b4c5-d6e7-4890-a123-456789abcdef
version: 2.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-stamp

Suscriptor táctico de señales de dominio. Invoca `tracker:lifecycle`. Ciego al markdown. Fail-soft D5.
