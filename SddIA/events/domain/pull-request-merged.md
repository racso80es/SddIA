---
uuid: "cfb8ce66-784e-4826-8a0a-a20c671e3a60"
name: "pull-request-merged"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "PullRequest_Merged"
context: "dlt-auditing"
capabilities:
  - "pull_request_merged"
hash_signature: "sha256:e82cf28dd23db23bafa5a860d46ca61ea431a12bbdd27712e0d49bf4e6dd4c20"
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

| Suscriptor | Agente | Intent |
| :--- | :--- | :--- |
| `iota-immutable-publisher` | cumulo | Anclaje DLT IOTA Rebased |
| `notify-humanized-pr-merged` | argos | Metadatos estáticos + síntesis de valor (fail-soft LLM) |

SSOT: `SddIA/core/event-domain-subscriptions.json` → clave `PullRequest_Merged`.
