---
uuid: "c8e1f4a2-6b3d-4f9e-a1c0-2d7e8f9a0b1c"
name: "bus-operator"
version: "1.1.0"
contract: "skills-contract v1.1.0"
context: "ecosystem-evolution"
capabilities:
  - "eda-subscription-lookup"
  - "event-bus-transit"
  - "receipt-suffix-mutation"
  - "delegate-markdown-table-editor"
provides:
  - id: "bus:route"
    contract: "bus.route"
    version: "1.0.0"
hash_signature: "sha256:555f77d07871af518a628825ee0de9b9c64a90c9c044e6b9661bb9f01775e469"
inputs:
  - "operation": "string; enum: resolve_subscribers | transit_payload | apply_receipt | sync_entity_index"
  - "operation_payload": "object; forma estricta según operation"
outputs:
  - "success": "boolean"
  - "exitCode": "integer"
  - "result": "object; payload delegado de la tool invocada"
  - "error": "string; diagnóstico en fallo"
---

# Skill: bus-operator

Capacidad cognitiva del dominio para gobernar el ciclo de vida del bus de eventos en archivos planos. **No** ejecuta comandos de sistema directos: orquesta las micro-tools ciegas (`read-event-subscriptions`, `manage-event-receipt`, `transit-event-payload`) y delega mutaciones de catálogo en `tool:markdown-table-editor`.

Proveedor canónico de la capacidad `bus:route` (PBI-043 H8 · laudo Racso A).

## Jerarquía

```
[Acción de Dominio] → [Agente] → [bus-operator] → [Tools atómicas]
```

## Operaciones

| `operation` | Delegación |
|-------------|------------|
| `resolve_subscribers` | `read-event-subscriptions` |
| `transit_payload` | `transit-event-payload` |
| `apply_receipt` | `manage-event-receipt` |
| `sync_entity_index` | `markdown-table-editor` (vía payload normalizado) |

## Cápsula física

`SddIA/skills/bus-operator/bus-operator.wasm` — un JSON por stdin (`operation`, `operation_payload`).
