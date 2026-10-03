---
uuid: "ee0dcb22-a37f-4803-b91a-17eadc205d92"
name: "hu-refined"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "HU_Refined"
context: "knowledge-management"
capabilities:
  - "hu_refined"
hash_signature: "sha256:f6c07eab5c9200b060b17847e188f5653e8d27b29711ebac601cca29be8350b3"
---

# Event: HU_Refined

HU refinada en historias/; trigger hacia todo vía tracker-stamp (PBI 08).

## Payload ECST

### REQUIRED
- `event_id`
- `correlation_id`
- `source_process`
- `hu_ref`
- `occurred_at`

### OPTIONAL
- `project_slug`
- `tracker_ref`

### FORBIDDEN
- `team_key`
- `hash_signature`

## Emisores autorizados

- `refine-hu`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `HU_Refined`.
