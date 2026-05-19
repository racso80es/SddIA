---
uuid: "7e4a9c2b-1d3f-4a8e-9b6c-0f1e2d3a4b5c"
name: "emit-domain-mutation"
version: "1.0.0"
contract: "actions-contract v1.2.0"
context: "ecosystem-evolution"
capabilities:
  - "domain-mutation-emission"
  - "event-bus-pending-write"
  - "delegate-filesystem-manager"
  - "delegate-crypto-broker"
  - "domain-event-type-translation"
inputs:
  - "entity_class": "string; enum: process | agent | skill | tool | action | norm | codex"
  - "lifecycle_operation": "string; enum: create | update | delete (no usar operation_type: colisión con git-manager)"
  - "entity_uuid": "string; UUID v4 inmutable de la entidad afectada"
  - "entity_name": "string; nombre canónico (en delete: nombre al momento del borrado)"
  - "version": "string|null; versión resultante; null permitido en delete"
  - "hash_signature_new": "string|null; sello sha256:… ; obligatorio salvo delete → null"
  - "hash_signature_old": "string|null; sello sha256:… ; obligatorio salvo create → null"
  - "changes_summary": "string; descripción breve del cambio; UTF-8; máx. 2048 caracteres"
  - "emitter_agent": "string; nombre o UUID del invocante indexado (ej. process-creator, cumulo)"
  - "correlation_id": "string|null; opcional; UUID v4 para sagas; omitir en raíz del evento si ausente"
outputs:
  - "success": "boolean"
  - "event_id": "string; UUID v4 del evento minteado"
  - "target_path": "string; ruta relativa al workspace del JSON en pending/"
minteo_maximo: null
porcentaje_de_exito: null
---

# Acción: emit-domain-mutation

## 1. Propósito

Inyectar un evento de dominio **ECST** estandarizado en el bus local (`eda_bus.pending` en `cumulo.paths.json`, por defecto `docs/events/pending/`) cuando una entidad estructural del genoma (Proceso, Agente, Skill, Tool, Norma, Acción o Códice) sufre una mutación en su ciclo de vida. Es el **Sello Universal** que garantiza la consciencia EDA del sistema sobre su propio genoma.

No calcula SHA-256 de entidades, no interactúa con Git, no enruta el bus ni ancla en DLT; solo valida forma de inputs, mintea `event_id`, traduce `lifecycle_operation` → `event_type` y persiste vía cápsulas autorizadas (topología Cúmulo).

## 2. Orquestación

Gate **Cerbero** previo por `context` de cada cápsula. Rutas vía `cumulo.paths.json`. El **Gestor de Entidad** invocante (p. ej. `*-creator`, runtime de forja) debe inyectar hashes y metadatos ya resueltos.

### Paso 1 — Validación de inputs

| Regla | Condición |
| :--- | :--- |
| Enums | `entity_class` y `lifecycle_operation` en los valores declarados en cabecera YAML |
| `changes_summary` | Longitud ≤ 2048; UTF-8 válido |
| `create` | `hash_signature_old` es `null`; `hash_signature_new` no nulo |
| `update` | `hash_signature_new` y `hash_signature_old` no nulos |
| `delete` | `hash_signature_new` es `null`; `hash_signature_old` no nulo |
| `correlation_id` | Si presente, formato UUID v4 |

- Cualquier violación: `success: false`, `exitCode: 1`, `data: null`, `error` causal. No persistir archivo.

### Paso 2 — Identidad del evento (`action:crypto-broker`)

```json
{
  "operation": "GENERATE_UUID",
  "target_type": "STRING",
  "target_payload": ""
}
```

- Abortar si `success` es `false` o `exitCode != 0`.
- `event_id` ← `data.result`.
- `timestamp` ← ISO-8601 UTC en instante de ensamblaje.

### Paso 3 — Traducción `lifecycle_operation` → `event_type`

| `lifecycle_operation` | `event_type` |
| :--- | :--- |
| `create` | `Domain_Entity_Created` |
| `update` | `Domain_Entity_Updated` |
| `delete` | `Domain_Entity_Deleted` |

### Paso 4 — Cápsula de evento V2 (ECST)

Construir JSON UTF-8. En raíz incluir `correlation_id` **solo** si el input no es `null` ni cadena vacía.

```json
{
  "event_id": "<event_id>",
  "event_type": "<Domain_Entity_Created|Updated|Deleted>",
  "timestamp": "<timestamp>",
  "emitter_agent": "<emitter_agent>",
  "payload": {
    "entity_class": "<entity_class>",
    "lifecycle_operation": "<lifecycle_operation>",
    "entity_uuid": "<entity_uuid>",
    "entity_name": "<entity_name>",
    "version": "<version|null>",
    "hash_signature_new": "<hash_signature_new|null>",
    "hash_signature_old": "<hash_signature_old|null>",
    "changes_summary": "<changes_summary>"
  },
  "delivery_state": {}
}
```

### Paso 5 — Persistencia (`skill:filesystem-manager`)

1. Resolver `pending_dir` desde `cumulo.paths.json` → `eda_bus.pending` (fallback `docs/events/pending`).
2. Si falta `pending_dir`, invocar `CREATE_DIR` en esa ruta.
3. Escribir el JSON del Paso 4:

| Campo | Valor |
| :--- | :--- |
| `operation` | `WRITE_FILE` |
| `target_path` | `{eda_bus.pending}/<event_id>.json` |
| `content` | JSON UTF-8 del Paso 4 |

- Abortar si `exitCode != 0`.

### Paso 6 — Cierre (stdout)

```json
{
  "success": true,
  "exitCode": 0,
  "data": {
    "success": true,
    "event_id": "<event_id>",
    "target_path": "{eda_bus.pending}/<event_id>.json"
  }
}
```

En fallo de validación, broker o escritura: `success: false`, `exitCode: 1`, `data: null`, `error` causal. Sin logs verbosos ni salida humana redundante.

## 3. Límites

* Sin terminal cruda; sin `route-domain-event`, `git-manager` ni `GENERATE_SHA256` sobre la entidad.
* `context: ecosystem-evolution` está registrado en `execution-contexts.md` §2.5.
* Los tipos `Domain_Entity_*` deben existir en `event-subscriptions.json` antes de que el fan-out tenga efecto; hasta entonces el watcher puede mover eventos a `processed/` como no-op documentado en `route-domain-event`.
* Invocación esperada al cierre de forja o mutación física del artefacto en procesos `*-creator` (deuda de cableado en cada proceso).
