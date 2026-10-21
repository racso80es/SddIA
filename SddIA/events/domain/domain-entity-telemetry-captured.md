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
hash_signature: "sha256:1669721f182e081ef965290eea9b96b0632c9973ccef80564be7e89423a6f9db"
---

# Event: Domain_Entity_Telemetry_Captured

Snapshot de ejecución tras Raw_Execution_Finished. Emisor exclusivo Radamanto. Fan-out local: `memory-evolution-ingest` (LanceDB). Sin ancla IOTA.

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

## Auditoría DLT

`dlt_audit_scope: local_only`.

## Suscripciones

SSOT: `SddIA/core/event-domain-subscriptions.json` → clave `Domain_Entity_Telemetry_Captured` (paridad legado `event-subscriptions.json`).

| Suscriptor | Proceso / tool | Intención |
|------------|----------------|-----------|
| `cumulo` | `memory-evolution-ingest` | Indexar snapshot en LanceDB vía EvolutionProxy |
