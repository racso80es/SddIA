---
uuid: "0b51c028-38f8-4c53-a994-30c2949e55b6"
name: "validation-started"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Validation_Started"
context: "ecosystem-evolution"
capabilities:
  - "validation_started"
hash_signature: "sha256:6f294fc6a3ae8f501e52f397ebdc8b5aa5a995afc73f59c5a963bc2754ff65e2"
---

# Event: Validation_Started

Señal al cerrar con éxito la fase de Ejecución (`agent:tekton`, `status: executed`) en `feature` | `bug-fix` | `refactorization`. Consumidor principal: `tracker-stamp` (suscripción).

## Payload ECST

### REQUIRED
- `persist_ref`
- `source_process` (`feature` | `bug-fix` | `refactorization`)
- `occurred_at` (ISO-8601 UTC)

### OPTIONAL
- `project_slug`
- `branch`
- `pbi_ref` (ruta relativa del PBI)
- `tracker_ref` (identificador opaco)

### FORBIDDEN
- `team_key`
- ids de `WorkflowState`
- URLs del proveedor

## Emisores autorizados

- `emit-validation-started-event` (invocado desde el motor tras fase Tekton ejecutada)

## Suscripciones

Ver `SddIA/core/event-domain-subscriptions.json` → clave `Validation_Started`.
