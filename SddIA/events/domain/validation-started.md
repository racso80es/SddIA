---
uuid: "0b51c028-38f8-4c53-a994-30c2949e55b6"
name: "validation-started"
version: "1.0.1"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Validation_Started"
context: "ecosystem-evolution"
capabilities:
  - "validation_started"
hash_signature: "sha256:5c8f4a30e00775d26cde6715f3e89f163a017824a53d17d285acf4c3def61558"
---

# Event: Validation_Started

Señal histórica al cerrar la fase de Ejecución (`agent:tekton`, `status: executed`). Desde AUT-75 el motor **no** reemite este evento tras Tekton; **In Validation** la aplica `lifecycle_phase_action` (`forge_closed`) al entrar en Verificación.

## Ciclo tracker (AUT-75)

- **Writer:** `forge_closed` en transición a Verificación (presencia de `implementation.md` + `execution.md` en `persist_ref`).
- **Suscripción `tracker-stamp`:** no-op de columna si el evento llegara al bus.

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
