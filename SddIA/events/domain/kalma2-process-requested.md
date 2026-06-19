---
uuid: "458c34a8-9ad5-4a40-88c4-0be1e5d9598e"
name: "kalma2-process-requested"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Kalma2_Process_Requested"
context: "ecosystem-evolution"
capabilities:
  - "kalma2_process_requested"
hash_signature: "sha256:pending-anchor-on-merge"
---

# Event: Kalma2_Process_Requested

Solicitud de proceso de ciclo de vida emitida desde la interfaz Kalma2 (`kalma2-interact`).

## Payload ECST

### REQUIRED
- `process`
- `raw_text`

### OPTIONAL
- `pbi_ref`
- `process_inputs`

### FORBIDDEN
- *(ninguno)*

## Emisores autorizados

- Proceso **`kalma2-interact`**

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json`: `task-queue-manager` (despacho) + `iota-immutable-publisher` (DLT).
