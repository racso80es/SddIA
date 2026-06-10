---
uuid: "d4e5f6a7-b8c9-4d0e-1f2a-3b4c5d6e7f8a"
name: "emit-pr-audited-event"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "quality-assurance"
capabilities:
  - "pr-audited-event-emission"
  - "event-bus-pending-write"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs:
  - "target_entity_id": "string; branch, PR o path auditado"
  - "resolution": "string; PASS | REJECT | FLAG"
  - "audit_event_reference": "string; UUID o hash canónico de la ejecución Argos"
  - "violated_rules": "array; reglas violadas (opcional)"
  - "correlation_id": "string; UUID v4 de correlación causal (opcional)"
  - "emitter_agent": "string; emisor lógico (default argos)"
outputs:
  - "success": "boolean"
  - "event_id": "string; UUID v4 del evento minteado"
  - "target_path": "string; ruta relativa del JSON en pending/"
  - "audit_event_reference": "string; referencia depositada en payload"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: emit-pr-audited-event

## 1. Propósito

Emitir la instancia ECST **PullRequest_Audited** en `eda_bus.pending` conforme a `SddIA/events/domain/pull-request-audited.md`. No enruta el bus ni notifica suscriptores; solo mintea `event_id` y persiste el JSON en pending.

**Invariante:** el invocante típico es la fase **Veredicto y bloqueo** de `pull-request-review` con `emitter_agent: argos`.

## 2. Orquestación

Gate **Cerbero** previo por `context`. Rutas vía `cumulo.paths.json`.

### Paso 1 — Validación

- `target_entity_id` y `resolution` obligatorios.
- `resolution` ∈ {`PASS`, `REJECT`, `FLAG`}.
- `audit_event_reference` obligatorio (string no vacío).

### Paso 2 — Identidad (`action:crypto-broker`)

```json
{ "operation": "GENERATE_UUID", "target_type": "STRING", "target_payload": "" }
```

### Paso 3 — Cápsula de evento

```json
{
  "event_id": "<event_id>",
  "event_type": "PullRequest_Audited",
  "timestamp": "<ISO-8601 UTC>",
  "emitter_agent": "<emitter_agent>",
  "correlation_id": "<correlation_id si aplica>",
  "payload": {
    "audit_event_reference": "<audit_event_reference>",
    "target_entity_id": "<target_entity_id>",
    "resolution": "<resolution>",
    "violated_rules": ["<violated_rules si aplica>"]
  },
  "delivery_state": {}
}
```

### Paso 4 — Persistencia (`skill:filesystem-manager`)

Escribir `{eda_bus.pending}/<event_id>.json`.

### Paso 5 — Cierre (stdout)

Envelope `actions-contract` con `success`, `event_id`, `target_path`, `audit_event_reference`.

## 3. Límites

* Sin `gh`, sin `git-manager`, sin `route-domain-event` ni IOTA en esta acción.
* Argos no invoca suscriptores; Ceguera Espacial absoluta.
