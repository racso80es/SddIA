---
uuid: "b0c1d2e3-f4a5-4678-9012-3456789abcde"
name: "emit-tracker-sync-failed"
version: "2.0.1"
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
  - "event_uri": "string"
hash_signature: "sha256:3b54358fbd3b7c2340593e6711656bc8304f282ecf497555a036ba163b6898f7"
minteo_maximo: null
porcentaje_de_exito: null
---

## Propósito

Fallo tracker → Tracker_Sync_Failed sin secretos HTTP en ECST.

## Orquestación

emit_tracker_sync_failed en actions.rs: issue_ref, operation, error_code, source_process.

## Límites

Sin tokens Linear.
