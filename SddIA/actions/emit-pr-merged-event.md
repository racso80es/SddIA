---
uuid: "c0d71f2b-c1c1-4c56-8f74-2f4f41b24c4f"
name: "emit-pr-merged-event"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "dlt-auditing"
capabilities:
  - "pr-merged-event-emission"
  - "event-bus-pending-write"
  - "delegate-git-manager"
  - "delegate-crypto-broker"
  - "delegate-filesystem-manager"
inputs:
  - "source_branch": "string; rama de origen fusionada"
  - "target_branch": "string; rama destino (p. ej. main)"
  - "author": "string; autor del merge"
  - "correlation_id": "string; UUID v4 de correlación causal (Sagas)"
outputs:
  - "success": "boolean"
  - "event_id": "string; UUID v4 del evento minteado"
  - "target_path": "string; ruta relativa al workspace del JSON persistido en pending/"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: emit-pr-merged-event

## 1. Propósito

Materializar el **Sello Criptográfico de Evento**: extraer el hash del commit de merge y depositar el contrato **PullRequest_Merged** (V2) en el bus de eventos local (`.SddIA/events/pending/`). No ejecuta terminal cruda ni anclaje DLT; solo orquesta skills autorizadas vía topología Cúmulo.

## 2. Orquestación

Toda invocación física exige gate **Cerbero** previo por `context` de cada cápsula destino. Rutas y cápsulas se resuelven exclusivamente desde `cumulo.paths.json`. El invocante debe inyectar `repository_path` (absoluta, validada por Cúmulo) en el estado de orquestación antes del Paso 1.

### Paso 1 — Hash del merge (`skill:git-manager`)

Invocar `skill:git-manager` con envelope JSON por stdin:

```json
{
  "operation_type": "get_last_commit",
  "repository_path": "<repository_path resuelta por Cúmulo>",
  "operation_payload_json": { "ref": "HEAD" }
}
```

- Si `success` es `false` o `exitCode != 0`: abortar; devolver `success: false`, `exitCode: 1`, `error` con `data.gitStderr` / `error` de la skill.
- Extraer `merge_commit_hash` desde `data.commitHash` (40 caracteres hexadecimales).

### Paso 2 — Identidad y tiempo (`action:crypto-broker`)

1. Invocar `action:crypto-broker` con `crypto_request`:

```json
{
  "operation": "GENERATE_UUID",
  "target_type": "STRING",
  "target_payload": ""
}
```

- Si falla: abortar con `success: false`, `exitCode: 1`.
- Asignar `event_id` = `crypto_response.data.result` (UUID v4).

2. Capturar `timestamp` en **ISO-8601** UTC (p. ej. `2026-05-16T15:05:00Z`) en el instante de construcción del evento.

### Paso 3 — Cápsula de evento V2

Construir el objeto JSON **autosuficiente** (Event-Carried State Transfer):

```json
{
  "event_id": "<event_id>",
  "event_type": "PullRequest_Merged",
  "timestamp": "<timestamp ISO-8601>",
  "emitter_agent": "delivery-close-cycle",
  "correlation_id": "<correlation_id del input>",
  "payload": {
    "source_branch": "<source_branch>",
    "target_branch": "<target_branch>",
    "merge_commit_hash": "<merge_commit_hash>",
    "author": "<author>",
    "security_clearance": {
      "auditor": "Argos",
      "audit_event_reference": "TODO: pending_argos_eda_emission",
      "policy_applied": "pr-acceptance-protocol"
    }
  },
  "delivery_state": {}
}
```

Serializar a cadena UTF-8 con `json.dumps` (orden de claves estable recomendado: orden del esquema anterior).

### Paso 4 — Persistencia en bus (`skill:filesystem-manager`)

Invocar `skill:filesystem-manager` (modalidad LLM-Native según contrato de skill):

| Campo | Valor |
| :--- | :--- |
| `operation` | `WRITE_FILE` |
| `target_path` | `.SddIA/events/pending/<event_id>.json` |
| `content` | Cadena JSON del paso 3 |

- Si el directorio `.SddIA/events/pending/` no existe, invocar previamente `CREATE_DIR` sobre `.SddIA/events/pending/` (misma skill, gate Cerbero).
- Si `exitCode != 0`: abortar; propagar `error_log` en `error` del envelope de acción.

### Paso 5 — Cierre

Emitir envelope canónico de acción en **stdout**:

```json
{
  "success": true,
  "exitCode": 0,
  "data": {
    "success": true,
    "event_id": "<event_id>",
    "target_path": ".SddIA/events/pending/<event_id>.json"
  }
}
```

En fallo: `success: false`, `exitCode: 1`, `data: null`, `error` con causa textual.

## 3. Límites

* No invoca `gh`, `git` ni binarios del host fuera de `skill:git-manager`.
* No implementa `route-domain-event`, anclaje IOTA ni mutación de `delivery_state`.
* No sobrescribe archivos en `pending/` sin política de idempotencia acordada por el proceso invocante.
* `context: dlt-auditing` debe figurar en la matriz S+ de `execution-contexts.md` y en `allowed_policies` del invocante antes de ejecución en producción.
