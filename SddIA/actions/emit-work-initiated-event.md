---
uuid: "c8d9e0f1-a2b3-4c5d-8e7f-6a5b4c3d2e1f"
name: "emit-work-initiated-event"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "work-initiated-event-emission"
  - "event-bus-pending-write"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs:
  - "branch": "string; rama de trabajo"
  - "persist_ref": "string; ruta lógica del espacio documental"
  - "source_process": "string; feature | bug-fix | refactorization"
  - "project_slug": "string; opcional"
  - "pbi_ref": "string; opcional"
  - "tracker_ref": "string; opcional, opaco"
  - "correlation_id": "string; UUID v4 opcional"
outputs:
  - "success": "boolean"
  - "event_id": "string"
  - "target_path": "string"
minteo_maximo: null
porcentaje_de_exito: null
hash_signature: "sha256:be754cf0517029238eddad56296fdf8addf974f71ecdd0ed27e7fef4d01f5d5a"
---

# Acción: emit-work-initiated-event

Emite **Work_Initiated** en `eda_bus.pending` conforme a `SddIA/events/domain/work-initiated.md`. Fail-soft en el invocante (`workspace-init`).
