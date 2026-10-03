---
uuid: "cfb8ce66-784e-4826-8a0a-a20c671e3a60"
name: "pull-request-merged"
version: "1.1.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "PullRequest_Merged"
context: "dlt-auditing"
capabilities:
  - "pull_request_merged"
hash_signature: "sha256:f2ee61c93b873d62e6d464da434214d79473ca066ff26257888977c638556c12"
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
- `tracker_ref` (identificador Linear del PBI; sello `tracker-stamp`)

### FORBIDDEN
- `hash_signature`

## Emisores autorizados

- `emit-pr-merged-event`
- `accept-pr`

## Suscripciones

| Suscriptor | Agente | Intent |
| :--- | :--- | :--- |
| `tracker-stamp` | tekton | Sello Linear done + comentario merge (tracker_ref). |
| `iota-immutable-publisher` | cumulo | Anclaje DLT IOTA Rebased. |
| `notify-humanized-pr-merged` | argos | Resumen ejecutivo post-merge: metadatos estáticos + síntesis de valor (fail-soft LLM). |

SSOT: `SddIA/core/event-domain-subscriptions.json` → clave `PullRequest_Merged`.
