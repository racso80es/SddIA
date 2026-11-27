---
context:
- tracker-operations
contract: process-contract v1.4.0
hash_signature: "sha256:f290918d2ca12ad50a2287c52b05d729541a3576365a99ad5a7271fc1028a3b6"
inputs:
- 'event_uri: Token lógico EventUri (eda://pending/{event_id})'
- 'issue_ref: Identificador opaco de tracker:lifecycle (payload)'
- 'operation: update_issue_state | create_comment'
- 'error_code: Código tipado del fallo original'
- 'target_state: Estado canónico objetivo (opcional)'
- 'comment_kind: Tipo de comentario (opcional)'
minteo_maximo: null
name: tracker-sync-replay
outputs:
- replay_status: applied | discarded | failed
phases:
- intent: Replay de sincronización. Un fetch_issue y reaplicación si el ciclo avanza, vía tracker:lifecycle.
  name: Replay de sincronización
  id: replay_de_sincronizacion
porcentaje_de_exito: null
uuid: e1f2a3b4-c5d6-4789-a012-3456789abc01
version: 2.0.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
---

# tracker-sync-replay

Suscriptor de `Tracker_Sync_Failed`. Contexto `tracker-operations`. Fail-soft: segundo fallo delegado al bus (witness dead-letter).

## Propósito

Suscriptor de `Tracker_Sync_Failed`. Contexto `tracker-operations`. Fail-soft: segundo fallo delegado al bus (witness dead-letter).

## Operación

Ejecución conforme al contrato de familia y topología Cúmulo; delegación en cápsulas autorizadas sin rutas cableadas.
