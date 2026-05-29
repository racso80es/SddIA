---
uuid: "b3c4d5e6-f7a8-4b9c-8d0e-1f2a3b4c5d6f"
name: "suite-execution-requested"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Suite_Execution_Requested"
context: "chaos-engineering"
capabilities:
  - "suite_execution_requested"
  - "chaos_campaign_stimulus"
hash_signature: "sha256:pending-anchor-on-merge"
---

# Event: Suite_Execution_Requested

Estímulo reactivo para orquestar una **campaña Suite** (Ingeniería del Caos). Dispara `process:execute-suite` vía bus domain.

## Payload ECST

### REQUIRED

- `suite_id`

### OPTIONAL

- `asset_id`
- `execution_strategy`

### FORBIDDEN

- `branch`
- `pr_url`

## Emisores autorizados

- Acción **`emit-suite-execution-requested`** (indexada)

## Suscripciones

Ver `event-domain-subscriptions.json` → `Suite_Execution_Requested`.
