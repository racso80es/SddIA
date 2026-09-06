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
hash_signature: "sha256:01884d5cb77da51e5b931314a7b3a3505369d53bca4b1c0aafa491c1ffcc25cd"
---

# Event: Domain_Entity_Telemetry_Captured

Snapshot de ejecución tras Raw_Execution_Finished. Emisor exclusivo Radamanto. Fan-out: `memory-evolution-ingest` (LanceDB) e `iota-immutable-publisher` (anclaje DLT).

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

SSOT: `SddIA/core/event-domain-subscriptions.json` → clave `Domain_Entity_Telemetry_Captured` (paridad legado `event-subscriptions.json`).

| Suscriptor | Proceso / tool | Intención |
|------------|----------------|-----------|
| `cumulo` | `memory-evolution-ingest` | Indexar snapshot en LanceDB vía EvolutionProxy |
| `cumulo` | `iota-immutable-publisher` | Anclaje DLT inmutable del snapshot en IOTA Rebased |
