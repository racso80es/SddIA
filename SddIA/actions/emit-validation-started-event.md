---
uuid: "e84218a8-7e40-4436-8ae7-a3fcc3396ad7"
name: "emit-validation-started-event"
version: "2.0.1"
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
  - "event_uri": "string"
hash_signature: "sha256:063d68282b75cef68261c673840c96775528a724a50c9baa2dccfa68880ae953"
minteo_maximo: null
porcentaje_de_exito: null
---

## Propósito

Sellar `Validation_Started` tras fases Tekton previas a validación Argos.

## Orquestación

`emit_validation_started` + hook `maybe_emit_validation_started_after_tekton_agent`; payload según `validation-started.md`.

## Límites

Fail-soft; no escribe `validacion.md`.
