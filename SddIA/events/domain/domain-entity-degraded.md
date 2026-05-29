---
uuid: "7a1b2c3d-4e5f-4a6b-8c9d-0e1f2a3b4c5d"
name: "domain-entity-degraded"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Domain_Entity_Degraded"
context: "quality-assurance"
capabilities:
  - "domain_entity_degraded"
  - "self_healing_trigger"
hash_signature: "sha256:pending-anchor-on-merge"
---

# Event: Domain_Entity_Degraded

Degradación de estatus S+ Grade de una entidad de dominio. Emisor exclusivo: **Radamanto**. Dispara revocación RBAC (Cerbero) e instanciación de reparación (`fix-tool-process` cuando `entity_type=tool`).

## Payload ECST

### REQUIRED

- `entity_type`
- `entity_id`
- `reason`
- `success_rate`
- `recovery_attempt`

### OPTIONAL

- `avg_duration_ms`

### FORBIDDEN

- `branch`
- `pr_url`
- `target_entity_id`

## Emisores autorizados

- Agente **`radamanto`** (vía `radamanto-batch`)

## Suscripciones

Ver `event-domain-subscriptions.json` → `Domain_Entity_Degraded`.
