---
uuid: "d4e5f6a7-b8c9-4d0e-1f2a-3b4c5d6e7f8a"
name: "emit-pr-audited-event"
version: "2.0.0"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
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
  - "event_uri": "Token lógico EventUri resuelto por EventManagerPort (eda://{family}/{event_id})"
  - "audit_event_reference": "string; referencia depositada en payload"
hash_signature: "sha256:3a5f84a66f7ce1cc30aff45dc867824ba549db5f5d216cbd514f5c88e54ff1dc"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: emit-pr-audited-event

## 1. Propósito

Emitir la instancia ECST **PullRequest_Audited** en `eda_bus.pending` conforme a `SddIA/events/domain/pull-request-audited.md`. No enruta el bus ni notifica suscriptores; solo mintea `event_id` y registra el ECST (medio según `events-contract.md` §4).

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

### Paso 4 — Persistencia

Según `SDDIA_EVENT_STORE` (`SddIA/events/events-contract.md` §4): en `redb`/`dual` el runtime persiste vía `write_pending_event` en `.SddIA/runtime/events.redb` (sin fichero bajo `.events/pending/` en `redb` puro). En `filesystem`, JSON en `{eda_bus.pending}/<event_id>.json` (resoluble vía `cumulo.paths.json`). El `event_uri` devuelto puede ser la ruta lógica `.events/pending/<event_id>.json` aunque el fichero no exista en `redb`.

### Paso 5 — Cierre (stdout)

Envelope `actions-contract` con `success`, `event_id`, `event_uri`, `audit_event_reference`.

## 3. Límites

* Sin `gh`, sin `git-manager`, sin `route-domain-event` ni IOTA en esta acción.
* Argos no invoca suscriptores; Ceguera Espacial absoluta.
