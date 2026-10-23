---
uuid: "b7c8d9e0-f1a2-4b3c-8d7e-6f5a4b3c2d1e"
name: "work-initiated"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Work_Initiated"
context: "ecosystem-evolution"
capabilities:
  - "work_initiated"
hash_signature: "sha256:d741267eb2df2509fd87501c6d90acbdfb9272c990336ab104afe4386aba9c64"
---

# Event: Work_Initiated

Señal al cerrar la inicialización de espacio de trabajo (`feature` | `bug-fix` | `refactorization`). Consumidor principal: `tracker-stamp` (suscripción).

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
