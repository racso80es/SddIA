---
uuid: "a1b2c3d4-e5f6-4789-a012-3456789abcde"
name: "emit-pr-presented-event"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "pr-presented-event-emission"
  - "event-bus-pending-write"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs:
  - "branch": "string; rama de la feature presentada"
  - "status": "string; estado ECST (default presented)"
  - "emitter_agent": "string; emisor lógico (default emit-pr-presented-event)"
outputs:
  - "success": "boolean"
  - "event_id": "string; UUID v4 del evento minteado"
  - "target_path": "string; ruta relativa del JSON en pending/"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: emit-pr-presented-event

## 1. Propósito

Emitir la instancia ECST **PullRequest_Presented** en `eda_bus.pending` conforme a `SddIA/events/pull-request-presented.md`. No abre PR en GitHub ni enruta el bus; solo mintea `event_id` y persiste el JSON en pending.

## 2. Orquestación (laboratorio)

1. Validar `branch` (obligatorio).
2. `action:crypto-broker` → `event_id`.
3. `skill:filesystem-manager` → escribir `{pending}/{event_id}.json`.
