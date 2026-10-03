---
uuid: "8fd2d4a2-21f8-484a-95b4-a732faa6ed98"
name: "pbi-refined"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "PBI_Refined"
context: "knowledge-management"
capabilities:
  - "pbi_refined"
hash_signature: "sha256:1a6177d564c21127e9396befe640d23d653835b63f2a2b6d73eae9856739b52d"
---

# Event: PBI_Refined

PBI refinado en todos_pending; trigger hacia todo vía tracker-stamp (PBI 08).

## Payload ECST

### REQUIRED
- `event_id`
- `correlation_id`
- `source_process`
- `pbi_ref`
- `occurred_at`

### OPTIONAL
- `project_slug`
- `tracker_ref`

### FORBIDDEN
- `team_key`
- `hash_signature`

## Emisores autorizados

- `refine-pbi`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `PBI_Refined`.
