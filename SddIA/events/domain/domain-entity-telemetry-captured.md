---
uuid: "54a49fa7-8d45-4376-9aa1-deeebeb301ea"
name: "domain-entity-telemetry-captured"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Domain_Entity_Telemetry_Captured"
context: "ecosystem-evolution"
capabilities:
  - "domain_entity_telemetry_captured"
hash_signature: "sha256:cb5658086b143cf791798f9dd8798dbedef3dff7c80ff789fc29f4fd8e104cfc"
---

# Event: Domain_Entity_Telemetry_Captured

Snapshot de ejecución tras Raw_Execution_Finished. Emisor exclusivo Radamanto. Dispara ingesta vectorial vía memory-evolution-ingest.

## Payload ECST

### REQUIRED
- `entity_type`
- `entity_id`
- `execution_metrics`
- `origin_stimulus`

### OPTIONAL
- `evolution_footprint`
- `state_after`
- `asset_id`

### FORBIDDEN
- `hash_signature_old`
- `hash_signature_new`
- `target_entity_id`
- `secrets`
- `api_keys`

## Emisores autorizados

- `radamanto`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Domain_Entity_Telemetry_Captured`.
