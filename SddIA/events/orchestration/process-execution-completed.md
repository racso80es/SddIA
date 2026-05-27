---
uuid: "a8f3c2e1-9b4d-4a7c-8e6f-1d2b3c4d5e6f"
name: "process-execution-completed"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "orchestration"
event_type: "Process_Execution_Completed"
context: "system-operations"
capabilities:
  - "process_execution_completed"
  - "orchestration_handoff"
hash_signature: "sha256:3c8f1a2b4d5e6f708192a3b4c5d6e7f8091a2b3c4d5e6f708192a3b4c5d6e7f8"
---

# Event: Process_Execution_Completed

Notificación táctica de orquestación emitida por el CLI tras ejecución exitosa de un proceso (`status: success`). Transporta blueprint mínimo y coordenada de workspace.

## Payload ECST

### REQUIRED

- `process_name`
- `asset_id`
- `workspace_path`
- `status`

### OPTIONAL

- `execution_id`
- `phase_count`
- `persist_ref`

### FORBIDDEN

- *(ninguno en v1.0.0)*

## Emisores autorizados

- `execute-process` (Peaje Termodinámico — post-éxito)

## Suscripciones

Ver `SddIA/core/event-orchestration-subscriptions.json` → clave `Process_Execution_Completed`.
