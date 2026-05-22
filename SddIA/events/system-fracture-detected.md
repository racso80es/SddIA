---
uuid: "f8e3a1b2-c4d5-4e6f-9a0b-1c2d3e4f5a6b"
name: "system-fracture-detected"
version: "1.0.0"
contract: "events-contract v1.0.0"
event_type: "System_Fracture_Detected"
context: "ecosystem-evolution"
capabilities:
  - "system_fracture_detected"
hash_signature: "sha256:pending-anchor-on-merge"
---

# Event: System_Fracture_Detected

Clase ECST para colapso de un proceso o cápsula oficial SddIA. Dispara materialización automática de deuda (PBI) vía Cúmulo — protocolo Kintsugi Ontológico.

## Payload ECST

### REQUIRED
- `process_name`
- `error_trace`
- `agent_emitter`
- `attempted_action`

### OPTIONAL
- `persist_ref`
- `branch_name`
- `correlation_id`

### FORBIDDEN
- *(ninguno)*

## Emisores autorizados

- Procesos oficiales en fase de fallo (vía escritura en `eda_bus.pending` + watcher)
- Operador humano en retroactivo documentado

## Suscripciones (fan-out ordenado)

| Orden | Agente | Acción | Rol Kintsugi |
|-------|--------|--------|--------------|
| 1 | **Cúmulo** | `materialize-fracture-pbi` | Materializa PBI — el **Qué** ha fallado |
| 2 | **Mayeuta** | `enrich-fracture-pbi-kaizen` | Síntesis analítica — el **Por Qué** y propuesta evolutiva |

Ver `SddIA/core/event-subscriptions.json` → clave `System_Fracture_Detected`.
