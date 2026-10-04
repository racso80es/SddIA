---
context:
- tracker-operations
contract: process-contract v1.4.0
hash_signature: "sha256:212e94d7feae9eaa4146e288ae729edbe6bbbb561c05b5138c1f51dbd92a681d"
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
- delegates_to:
  - tool:linear-tracker-adapter
  intent: fetch_issue y reaplicar si el ciclo avanza (orden backlog<todo<in_progress<…; cancelled terminal).
  name: Replay Linear
porcentaje_de_exito: null
uuid: e1f2a3b4-c5d6-4789-a012-3456789abc01
version: 1.1.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-sync-replay

Suscriptor de `Tracker_Sync_Failed`. Contexto `tracker-operations`. Fail-soft: segundo fallo delegado al bus (witness dead-letter).
