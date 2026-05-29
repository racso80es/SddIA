---
uuid: "c4d5e6f7-a8b9-4c0d-9e1f-2a3b4c5d6e7f"
name: "system-immunity-certified"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "System_Immunity_Certified"
context: "quality-assurance"
capabilities:
  - "system_immunity_certified"
  - "chaos_immunity_dlt"
hash_signature: "sha256:pending-anchor-on-merge"
---

# Event: System_Immunity_Certified

Certificación de **inmunidad sistémica** tras campaña Suite exitosa y manifiesto Argos. Sello DLT exclusivo **Radamanto** (D0.4).

## Payload ECST

### REQUIRED

- `suite_id`
- `survival_manifest_path`
- `orchestrator_execution_id`
- `nodes_passed`
- `nodes_total`

### OPTIONAL

- `asset_id`
- `hash_signature_manifest`

### FORBIDDEN

- `branch`
- `pr_url`

## Emisores autorizados

- Proceso **`execute-suite`** (handler lab `run_execute_suite`)

## Suscripciones

Ver `event-domain-subscriptions.json` → `System_Immunity_Certified`.
