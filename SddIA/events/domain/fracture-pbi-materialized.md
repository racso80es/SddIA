---
uuid: "6f4c2a81-9d3e-4b17-8a55-c0e1d2f3a4b5"
name: "fracture-pbi-materialized"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Fracture_Pbi_Materialized"
context: "ecosystem-evolution"
capabilities:
  - "fracture_pbi_materialized"
hash_signature: "sha256:dad2be5b53379ec7d20f8a1769ac6df6e27fff945401b1c8c137fc32469e4a59"
---

# Event: Fracture_Pbi_Materialized

Hecho emitido por materialize-fracture-pbi cuando el sumidero del proveedor acepta un alta nueva. Lleva la referencia opaca para que Mayeuta enriquezca ese ítem.

## Payload ECST

### REQUIRED
- `process_name`
- `error_trace`
- `agent_emitter`
- `attempted_action`
- `target_path`
- `trace_hash`
- `reason`

### OPTIONAL
- `tracker_ref`
- `project_slug`
- `persist_ref`
- `branch_name`
- `friction_id`

### FORBIDDEN
- *(ninguno)*

## Emisores autorizados

- `action:materialize-fracture-pbi`

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Fracture_Pbi_Materialized`.
