---
uuid: "e84218a8-7e40-4436-8ae7-a3fcc3396ad7"
name: "emit-validation-started-event"
version: "1.0.1"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "validation-started-event-emission"
  - "event-bus-pending-write"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs:
  - "persist_ref": "string; ruta lógica del espacio documental"
  - "source_process": "string; feature | bug-fix | refactorization"
  - "project_slug": "string; opcional"
  - "branch": "string; opcional"
  - "pbi_ref": "string; opcional"
  - "tracker_ref": "string; opcional, opaco"
  - "correlation_id": "string; UUID v4 opcional"
outputs:
  - "success": "boolean"
  - "event_id": "string"
  - "target_path": "string"
minteo_maximo: null
porcentaje_de_exito: null
hash_signature: "sha256:751af3e75d62e5c684c01c1d2ca59666bfc5de9afdede748a957b84131029cbd"
---

# Acción: emit-validation-started-event

Emite **Validation_Started** en `eda_bus.pending` conforme a `SddIA/events/domain/validation-started.md`. Fail-soft en el invocante (motor, post-fase Tekton).
