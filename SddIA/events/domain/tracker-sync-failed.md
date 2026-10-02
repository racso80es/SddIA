---
uuid: "a9b8c7d6-e5f4-4321-b987-6543210fedcc"
name: "tracker-sync-failed"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Tracker_Sync_Failed"
context: "tracker-operations"
capabilities:
  - "tracker_sync_failed"
hash_signature: "sha256:267be4b7373a9e1df08aaf50882792ea6cb075aedc193d887b48c9f9693d69c2"
---

# Event: Tracker_Sync_Failed

Deuda de sincronización con Linear tras fail-soft en `tracker-stamp`. Suscriptor: `tracker-sync-replay`.

## Payload ECST

### REQUIRED
- `issue_ref`
- `operation` (`update_issue_state` | `create_comment`)
- `error_code` (incl. `TRACKER_STATE_DIVERGED`, códigos `LINEAR_*` de la cápsula)
- `occurred_at` (ISO-8601 UTC)
- `source_process`

### Condicional
- `target_state` (canónico) si `operation` = `update_issue_state`
- `comment_kind` si `operation` = `create_comment`

### OPTIONAL
- `project_slug`
- `pr_url`
- `commit_sha`
- `attempt` (entero; default lógico 0)

### FORBIDDEN
- Token API Linear
- Cuerpo HTTP de respuesta de Linear

## Emisores autorizados

- `tracker-stamp` (vía `emit-tracker-sync-failed`)
- Procesos de sellado autorizados en la HU §4.D
