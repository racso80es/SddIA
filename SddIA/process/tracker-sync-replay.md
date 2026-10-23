---
context:
- tracker-operations
contract: process-contract v1.4.0
hash_signature: "sha256:dc2f0deb23f5714b9996ec8d441012f58494d760f64a63c0ffa33124fca2781e"
inputs:
- event_file_path: Ruta relativa al JSON ECST en pending (desde route-domain)
- issue_ref: Identificador opaco de tracker:lifecycle (payload)
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
- intent: Replay de sincronización. Un fetch_issue y reaplicación si el ciclo avanza, vía tracker:lifecycle.
  name: Replay de sincronización
porcentaje_de_exito: null
uuid: e1f2a3b4-c5d6-4789-a012-3456789abc01
version: 1.0.2
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-sync-replay

Suscriptor de `Tracker_Sync_Failed`. Contexto `tracker-operations`. Fail-soft: segundo fallo delegado al bus (witness dead-letter).
