---
uuid: "c0d71f2b-c1c1-4c56-8f74-2f4f41b24c4f"
name: "emit-pr-merged-event"
version: "1.1.2"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "pr-merged-event-emission"
  - "event-bus-pending-write"
  - "delegate-git-manager"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs:
  - "source_branch": "string; rama de origen fusionada"
  - "author": "string; autor del merge"
  - "correlation_id": "string; UUID v4 de correlación causal (Sagas)"
  - "audit_event_reference": "string; referencia PullRequest_Audited (opcional; resuelta vía bus si ausente)"
outputs:
  - "success": "boolean"
  - "event_id": "string; UUID v4 del evento minteado"
  - "target_path": "string; ruta relativa al workspace del JSON persistido en pending/"
hash_signature: "sha256:10e18e8344238c62eb9eceb0195af8690592c48411c64251a683aadf86d0c5e2"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: emit-pr-merged-event

## 1. Propósito

Extraer el hash de un merge recién ejecutado y emitir el evento **PullRequest_Merged** en `eda_bus.pending` (`cumulo.paths.json`) para su anclaje inmutable. No ejecuta merge, `gh`, anclaje DLT ni enrutamiento del bus; solo orquesta skills autorizadas vía topología Cúmulo.

**Invariante:** `target_branch` del payload está **fijado a** `"main"`. El invocante debe haber dejado `HEAD` en `main` tras el merge soberano.

## 2. Orquestación

Gate **Cerbero** previo por `context` de cada cápsula. Rutas vía `cumulo.paths.json`. Inyectar `repository_path` (absoluta, validada por Cúmulo) en estado de orquestación antes del Paso 1.

### Paso 1 — Hash del merge (`skill:git-manager`)

```json
{
  "operation_type": "get_last_commit",
  "repository_path": "<repository_path resuelta por Cúmulo>",
  "operation_payload_json": { "ref": "HEAD" }
}
```

- Abortar si `success` es `false` o `exitCode != 0`.
- `merge_commit_hash` ← `data.commitHash` (40 hex).

### Paso 2 — Identidad y tiempo (`action:crypto-broker`)

```json
{
  "operation": "GENERATE_UUID",
  "target_type": "STRING",
  "target_payload": ""
}
```

- `event_id` ← `crypto_response.data.result`.
- `timestamp` ← ISO-8601 UTC en instante de construcción.

### Paso 3 — Cápsula de evento V2

```json
{
  "event_id": "<event_id>",
  "event_type": "PullRequest_Merged",
  "timestamp": "<timestamp>",
  "emitter_agent": "accept-pr",
  "correlation_id": "<correlation_id>",
  "payload": {
    "source_branch": "<source_branch>",
    "target_branch": "main",
    "merge_commit_hash": "<merge_commit_hash>",
    "author": "<author>",
    "security_clearance": {
      "auditor": "Argos",
      "audit_event_reference": "<audit_event_reference>",
      "policy_applied": "pr-acceptance-protocol"
    }
  },
  "delivery_state": {}
}
```

### Paso 4 — Persistencia

Según `SDDIA_EVENT_STORE` (`SddIA/events/events-contract.md` §4): en `redb`/`dual` el runtime persiste vía `write_pending_event` en `.SddIA/runtime/events.redb` (sin fichero bajo `.events/pending/` en `redb` puro). En `filesystem`, invocar `skill:filesystem-manager` con `WRITE_FILE` a `{eda_bus.pending}/<event_id>.json` (crear directorio vía `CREATE_DIR` si falta). El `target_path` devuelto puede ser la ruta lógica `.events/pending/<event_id>.json` aunque el fichero no exista en `redb`.

### Paso 5 — Cierre (stdout)

```json
{
  "success": true,
  "exitCode": 0,
  "data": {
    "success": true,
    "event_id": "<event_id>",
    "target_path": ".events/pending/<event_id>.json"
  }
}
```

## 3. Límites

* Sin terminal cruda; sin `route-domain-event` ni IOTA.
* `context: dlt-auditing` debe existir en `execution-contexts.md` antes de producción.
