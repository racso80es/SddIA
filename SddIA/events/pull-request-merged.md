---
uuid: "cfb8ce66-784e-4826-8a0a-a20c671e3a60"
name: "pull-request-merged"
version: "1.0.0"
contract: "events-contract v1.0.0"
event_type: "PullRequest_Merged"
context: "dlt-auditing"
capabilities:
  - "pull_request_merged"
hash_signature: "sha256:fea0f07b140a90516e3a14b262043980bde33cb65d4c828ad7c7b04cfff52eae"
---

# Event: PullRequest_Merged

Clase ECST para sello post-merge en main. Ancla DLT via merge_commit_hash (40 hex); prohibido hash_signature en payload.

## Payload ECST

### REQUIRED
- `source_branch`
- `target_branch`
- `merge_commit_hash`
- `author`
- `security_clearance`

### OPTIONAL
- `pr_url`
- `repository_name`

### FORBIDDEN
- `hash_signature`

## Emisores autorizados

- `emit-pr-merged-event`
- `accept-pr`

## Suscripciones

Ver `SddIA/core/event-subscriptions.json` → clave `PullRequest_Merged`.
