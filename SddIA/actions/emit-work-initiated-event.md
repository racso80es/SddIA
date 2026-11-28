---
uuid: "c8d9e0f1-a2b3-4c5d-8e7f-6a5b4c3d2e1f"
name: "emit-work-initiated-event"
version: "2.0.1"
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
  - "event_uri": "string"
hash_signature: "sha256:a0370fbf37aeb3a33b196de01a9e082abdfe40efa37a01af43a41c508b2e821e"
minteo_maximo: null
porcentaje_de_exito: null
---

## Propósito

Emitir ECST `Work_Initiated` al abrir workspace-init (feature/bug-fix/refactorization).

## Orquestación

Handler `actions.rs`: payload con `branch`, `persist_ref`, `source_process`, refs tracker; escribe `pending`. Fail-soft en invocante.

## Límites

No crea rama ni documentación; solo evento.
