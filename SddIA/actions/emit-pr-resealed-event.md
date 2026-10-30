---
uuid: "e8f9a0b1-c2d3-4e5f-9a0b-1c2d3e4f5a6b"
name: "emit-pr-resealed-event"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "pr-resealed-event-emission"
  - "event-bus-pending-write"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs:
  - "correlation_id": "string; event_id del PullRequest_Presented correlacionado"
  - "validated_head_sha": "string; SHA de cabeza con sello actualizado"
  - "diff_digest": "string; digest sellado"
  - "branch": "string; rama (opcional, payload ECST)"
  - "emitter_agent": "string; emisor lógico (default emit-pr-resealed-event)"
outputs:
  - "success": "boolean"
  - "event_id": "string; UUID v4 del evento minteado"
  - "target_path": "string; ruta relativa del JSON en pending/"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: emit-pr-resealed-event

## 1. Propósito

Emitir **PullRequest_Resealed** en `eda_bus.pending` tras un re-sellado exitoso en pre-push (PR OPEN). No ejecuta re-sello, QA ni push.

**Invariante:** solo se invoca cuando el sello cambió (`outcome: committed`); el primer push (DCC + `PullRequest_Presented`) no emite este evento.

## 2. Orquestación

Gate **Cerbero** previo por `context`. Rutas vía `cumulo.paths.json`.

### Paso 1 — Validación

- `correlation_id`, `validated_head_sha`, `diff_digest` obligatorios (string no vacío).

### Paso 2 — Identidad (`action:crypto-broker`)

`GENERATE_UUID` para `event_id`.

### Paso 3 — Cápsula de evento

Conforme a `SddIA/events/domain/pull-request-resealed.md`.

### Paso 4 — Persistencia

Escribir `{eda_bus.pending}/<event_id>.json`.

### Paso 5 — Cierre (stdout)

Envelope `actions-contract` con `success`, `event_id`, `target_path`.

## 3. Límites

* Sin `delivery-close-cycle`, sin `route-domain-event` en esta acción.
