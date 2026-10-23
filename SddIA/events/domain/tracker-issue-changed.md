---
uuid: "394d59d7-b041-4fda-ba26-6dbacdd86369"
name: "tracker-issue-changed"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Tracker_Issue_Changed"
context: "tracker-operations"
capabilities:
  - "tracker_issue_changed"
hash_signature: "sha256:a748a39e0346deb375482f47c95a938d0deb70d292d82660079e32d096a402f9"
---

# Event: Tracker_Issue_Changed

Cambio detectado en el ítem del tracker; entrada a tracker-markdown-apply.

## Payload ECST

### REQUIRED
- `correlation_id`
- `occurred_at`
- `project_slug`
- `issue_ref`
- `changes`
- `source`

### OPTIONAL
- `actor`
- `linear_updated_at`

### FORBIDDEN
- `LINEAR_API_TOKEN`
- `webhook_body`
- `raw_webhook`
- `hash_signature`

## Emisores autorizados

- `linear-bridge-watcher`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Tracker_Issue_Changed` (`tracker-markdown-apply`).
