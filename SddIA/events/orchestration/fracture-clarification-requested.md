---
uuid: "7d08d6c4-2ee3-4a70-953f-6f49375913d8"
name: "fracture-clarification-requested"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "orchestration"
event_type: "Fracture_Clarification_Requested"
context: "ecosystem-evolution"
capabilities:
  - "fracture_clarification_requested"
hash_signature: "sha256:c4649999385006d99304db8955b756d94d866e7967121d269b9c53699b788d03"
---

# Event: Fracture_Clarification_Requested

Estímulo fractal para triaje semántico asíncrono de fracturas inéditas (unclassified). Emisor: enrich-fracture-pbi-kaizen. No incluye error_trace en payload.

## Payload ECST

### REQUIRED
- `fracture_pbi_path`
- `process_name`
- `error_trace_hash`

### OPTIONAL
- `attempted_action`
- `agent_emitter`
- `correlation_id`

### FORBIDDEN
- *(ninguno)*

## Emisores autorizados

- `action:enrich-fracture-pbi-kaizen`

## Suscripciones

Ver `SddIA/core/event-orchestration-subscriptions.json` → clave `Fracture_Clarification_Requested`. Suscriptor: `mayeuta` / `append-mayeuta-hypothesis` (fail-open).
