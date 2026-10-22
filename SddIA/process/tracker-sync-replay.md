---
context:
- tracker-operations
contract: process-contract v1.4.0
hash_signature: sha256:b4558aa7b6810446ce5e4271afc646c51dfefba43fcf2527e1c30e6b91e75305
inputs:
- event_file_path: Ruta relativa al JSON ECST en pending (desde route-domain)
- issue_ref: Identificador Linear (payload)
- operation: update_issue_state | create_comment
- error_code: Código tipado del fallo original
- target_state: Estado canónico objetivo (opcional)
- comment_kind: Tipo de comentario (opcional)
- project_slug: Slug de proyecto (opcional)
- attempt: Reintento (opcional)
minteo_maximo: null
name: tracker-sync-replay
outputs:
- replay_status: applied | discarded | failed
phases:
- intent: fetch_issue y reaplicar si el ciclo avanza, usando el provider resuelto de tracker:lifecycle. Con file no hay replay.
  name: Replay Linear
porcentaje_de_exito: null
uuid: e1f2a3b4-c5d6-4789-a012-3456789abc01
version: 1.0.1
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-sync-replay

Suscriptor de `Tracker_Sync_Failed`. Contexto `tracker-operations`. Fail-soft: segundo fallo delegado al bus (witness dead-letter).
