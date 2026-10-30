---
uuid: "a1b2c3d4-e5f6-4789-a012-3456789abcde"
name: "emit-pr-presented-event"
version: "1.1.3"
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
  - "emitter_agent": "string; emisor lógico (default delivery-close-cycle cuando invocado desde cierre)"
  - "pr_url": "string; URL del PR en forja (opcional, payload ECST v1.1)"
  - "correlation_id": "string; UUID v4 de correlación causal (opcional)"
  - "signer_identity_rbac": "string; identidad relay local (opcional; default Vertice_Biologico_Relay — aduana RBAC_SIGNER_PRESENT)"
outputs:
  - "success": "boolean"
  - "event_id": "string; UUID v4 del evento minteado"
  - "target_path": "string; ruta relativa del JSON en pending/"
hash_signature: "sha256:12c6d44924d8f1477c0d1ce783b90ca35bf57f0b1277815883decc41aaf7c508"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: emit-pr-presented-event

## 1. Propósito

Emitir la instancia ECST **PullRequest_Presented** en `eda_bus.pending` conforme a `SddIA/events/domain/pull-request-presented.md`. No abre PR en GitHub, no ejecuta `push` ni enruta el bus; solo mintea `event_id` y registra el ECST (medio según `events-contract.md` §4).

**Invariante:** el invocante típico es el proceso **`delivery-close-cycle`** tras la fase «Apertura en forja»; debe pasar `emitter_agent: delivery-close-cycle` y `pr_url` cuando esté disponible.

**Firmante:** `payload.signer_identity_rbac` se rellena siempre (input opcional o default `Vertice_Biologico_Relay`) para satisfacer aduana `RBAC_SIGNER_PRESENT` (paridad `github-bridge-watcher`).

## 2. Orquestación

Gate **Cerbero** previo por `context`. Rutas vía `cumulo.paths.json`.

### Paso 1 — Validación

- `branch` obligatorio (string no vacío).

### Paso 2 — Identidad (`action:crypto-broker`)

```json
{ "operation": "GENERATE_UUID", "target_type": "STRING", "target_payload": "" }
```

### Paso 3 — Cápsula de evento

```json
{
  "event_id": "<event_id>",
  "event_type": "PullRequest_Presented",
  "timestamp": "<ISO-8601 UTC>",
  "emitter_agent": "<emitter_agent>",
  "correlation_id": "<correlation_id si aplica>",
  "payload": {
    "branch": "<branch>",
    "status": "<status>",
    "pr_url": "<pr_url si aplica>",
    "signer_identity_rbac": "<signer_identity_rbac | Vertice_Biologico_Relay>"
  },
  "delivery_state": {}
}
```

### Paso 4 — Persistencia

Según `SDDIA_EVENT_STORE` (`SddIA/events/events-contract.md` §4): en `redb`/`dual` el runtime persiste vía `write_pending_event` en `.SddIA/runtime/events.redb` (sin fichero bajo `.events/pending/` en `redb` puro). En `filesystem`, JSON en `{eda_bus.pending}/<event_id>.json` (resoluble vía `cumulo.paths.json`). El `target_path` devuelto puede ser la ruta lógica `.events/pending/<event_id>.json` aunque el fichero no exista en `redb`.

### Paso 5 — Cierre (stdout)

Envelope `actions-contract` con `success`, `event_id`, `target_path`.

## 3. Límites

* Sin `gh`, sin `git-manager`, sin `route-domain-event` ni IOTA en esta acción.
