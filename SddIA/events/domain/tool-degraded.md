---
uuid: "7a1b2c3d-4e5f-4a6b-8c9d-0e1f2a3b4c5d"
name: "tool-degraded"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Tool_Degraded"
context: "quality-assurance"
capabilities:
  - "tool_degraded"
  - "self_healing_trigger"
hash_signature: "sha256:pending-anchor-on-merge"
---

# Event: Tool_Degraded

Degradación de estatus S+ Grade de una herramienta/skill. Emisor exclusivo: **Radamanto**. Dispara revocación RBAC (Cerbero) e instanciación de reparación (`fix-tool-process`).

## Payload ECST

### REQUIRED

- `target_entity_id`
- `reason`
- `success_rate`
- `recovery_attempt`

### OPTIONAL

- `avg_duration_ms`

### FORBIDDEN

- `branch`
- `pr_url`

## Emisores autorizados

- Agente **`radamanto`** (vía `radamanto-batch`)

## Suscripciones

Ver `event-domain-subscriptions.json` → `Tool_Degraded`.
