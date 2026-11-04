---
uuid: "b7c8d9e0-f1a2-4b3c-8d7e-6f5a4b3c2d1e"
name: "work-initiated"
version: "1.0.1"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Work_Initiated"
context: "ecosystem-evolution"
capabilities:
  - "work_initiated"
hash_signature: "sha256:337d397754ac8772ee14f75269d9d916fb6fe8abce188e2949d8dc4911a981d3"
---

# Event: Work_Initiated

Señal al cerrar la inicialización de espacio de trabajo (`feature` | `bug-fix` | `refactorization`). Anclaje del bus; **no** es el writer de columna In Progress desde AUT-75.

## Ciclo tracker (AUT-75)

- **Writer:** `lifecycle_phase_action` (`forge_started`) en `workspace-init`.
- **Suscripción `tracker-stamp`:** no-op de columna (sellado en origen); puede registrar comentario fail-soft si aplica.

## Payload ECST

### REQUIRED
- `branch`
- `persist_ref`
- `source_process` (`feature` | `bug-fix` | `refactorization`)
- `occurred_at` (ISO-8601 UTC)

### OPTIONAL
- `project_slug`
- `pbi_ref` (ruta relativa del PBI)
- `tracker_ref` (identificador opaco)

### FORBIDDEN
- `team_key`
- ids de `WorkflowState`
- URLs del proveedor

## Emisores autorizados

- `emit-work-initiated-event` (invocado desde `workspace-init` tras init git/documental)
