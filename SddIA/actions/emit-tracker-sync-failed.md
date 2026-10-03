---
uuid: "b0c1d2e3-f4a5-4678-9012-3456789abcde"
name: "emit-tracker-sync-failed"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "tracker-operations"
capabilities:
  - "tracker-sync-failed-emission"
  - "event-bus-pending-write"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs:
  - "issue_ref": "string"
  - "operation": "update_issue_state | create_comment"
  - "error_code": "string"
  - "source_process": "string"
  - "target_state": "string; opcional"
  - "comment_kind": "string; opcional"
  - "project_slug": "string; opcional"
  - "pr_url": "string; opcional"
  - "commit_sha": "string; opcional"
  - "attempt": "number; opcional"
  - "correlation_id": "string; opcional"
outputs:
  - "success": "boolean"
  - "event_id": "string"
  - "target_path": "string"
minteo_maximo: null
porcentaje_de_exito: null
hash_signature: "sha256:71aa7970cc08531dbddaeeb607cf66addcd42c4c44af5e78e3d9c48ba27e22d1"
---

# Acción: emit-tracker-sync-failed

Emite **Tracker_Sync_Failed** en `eda_bus.pending`. Sin secretos ni cuerpos HTTP de Linear.
