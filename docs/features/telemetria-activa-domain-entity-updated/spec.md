---
feature_name: telemetria-activa-domain-entity-updated
created: "2026-07-19"
process: feature
base: main
scope: domain ECST Telemetry_Captured, radamanto-batch emit, route-domain filter, memory-evolution-ingest, lancedb_evolution_repo
pbi_ref: docs/todos/pending/[OPERATIVO] PBI: Gestión e Ingesta de Telemetría Activa mediante Domain_Entity_Updated.md
document_id: PBI-TELEMETRIA-ACTIVA-DOMAIN-ENTITY-UPDATED
agent_planificador: dedalo
laudo: plan_b_domain_entity_telemetry_captured
---

# Especificación técnica — Ingesta telemetría activa (Plan B)

## 1. Contexto

Post Mayeuta v1.1.0. Hot path actual:

```text
Raw_Execution_Finished
  → ./.events/telemetry/
  → route-telemetry
  → radamanto-batch (+ telemetry-compliance-audit)
  → stats .SddIA/radamanto/ + Domain_Entity_{Degraded|Restored|Deprecated}
  ✗ sin snapshot indexable en memoria vectorial
```

`Domain_Entity_Updated` v1.1.0 es **CRUD genómico** (REQUIRED plano: `hash_signature_old/new`, `entity_uuid`, `version`, …). El validador `ecst_validation.rs` no soporta REQUIRED condicional por `lifecycle_operation`. Contaminar esa Clase rompería create/update/delete y `sync-entity-index`.

## 2. Laudo Dedalo — Plan B (vinculante)

| Opción | Veredicto |
|--------|-----------|
| **Plan A** — `Domain_Entity_Updated` + `lifecycle_operation: telemetry_snapshot` | **Rechazado** — incompatible con schema REQUIRED plano + gate ECST |
| **Plan B** — Clase nueva `Domain_Entity_Telemetry_Captured` | **Aprobado** — simetría con Degraded/Restored/Deprecated; CRUD intacto |

El título del PBI («mediante Domain_Entity_Updated») se interpreta como *mutación de estado de entidad de dominio por telemetría*; el `event_type` canónico de entrega es:

```text
Domain_Entity_Telemetry_Captured
```

## 3. Arquitectura objetivo

```text
Raw_Execution_Finished
  → route-telemetry → radamanto-batch
       │
       ├─ (existente) umbrales → Degraded / Restored / Deprecated
       │
       └─ (nuevo) siempre tras consumo OK →
              Domain_Entity_Telemetry_Captured
                → ./.events/domain/<event_id>.json
                → route-domain
                     └─ process: memory-evolution-ingest
                          ├─ EvolutionProxyService::capture_event
                          ├─ LanceDbEvolutionAdapter::store_event
                          │    → .SddIA/vector_store/evolution/
                          └─ (opcional) Vector_Memory_Indexed
```

Ortogonal a Self-Healing y a `telemetry-compliance-audit`.

## 4. Contrato ECST — `Domain_Entity_Telemetry_Captured`

| Campo cabecera | Valor |
|----------------|-------|
| `name` | `domain-entity-telemetry-captured` |
| `event_family` | `domain` |
| `event_type` | `Domain_Entity_Telemetry_Captured` |
| `context` | `ecosystem-evolution` |
| `capabilities` | `domain_entity_telemetry_captured`, `evolution_ingest_trigger` |
| Forja | `entity-manager` → `event-creator` (DA-2) |

### Payload

| Sección | Campos |
|---------|--------|
| **REQUIRED** | `entity_type`, `entity_id`, `execution_metrics`, `origin_stimulus` |
| **OPTIONAL** | `evolution_footprint`, `state_after`, `asset_id` |
| **FORBIDDEN** | `hash_signature_old`, `hash_signature_new`, `target_entity_id`, `secrets`, `api_keys` |

#### Formas

```text
execution_metrics: {
  duration_ms: number,
  exit_code: number,
  success_status: boolean
}

origin_stimulus: {
  event_type: "Raw_Execution_Finished",
  event_id: "<uuid>"
}
```

#### Emisor autorizado

- Solo `radamanto` vía `radamanto-batch` (paridad Restored).

#### Envelope ejemplo

```json
{
  "event_id": "88b78eb5-98e6-49ab-97aa-e807b57d2aec",
  "event_family": "domain",
  "event_type": "Domain_Entity_Telemetry_Captured",
  "timestamp": "2026-07-19T17:00:00Z",
  "emitter_agent": "radamanto",
  "payload": {
    "entity_type": "process",
    "entity_id": "<target-entity>",
    "asset_id": "<asset-id>",
    "execution_metrics": {
      "duration_ms": 142,
      "exit_code": 0,
      "success_status": true
    },
    "origin_stimulus": {
      "event_type": "Raw_Execution_Finished",
      "event_id": "<uuid-telemetria>"
    },
    "evolution_footprint": null,
    "state_after": {
      "last_execution_ms": 142,
      "last_exit_code": 0
    }
  },
  "delivery_state": {}
}
```

**Prohibido** alterar REQUIRED de `domain-entity-updated.md` para este PBI.

## 5. Emisión — `radamanto_batch_core`

Tras consumo válido de telemetría (antes o junto al sello `delivery_state` success del batch), **siempre** emitir snapshot (también si no hay Degraded/Restored):

1. Construir payload desde `Raw_Execution_Finished.payload` (`duration_ms`, `exit_code`, `asset_id`) + `entity_id` ya resuelto por Radamanto.
2. `entity_type`: inferir si el payload lo trae; fallback `"process"` o valor estable documentado en implementation (no inventar taxonomía ad hoc).
3. `build_domain_event("Domain_Entity_Telemetry_Captured", payload)` + `emit_domain_and_route`.
4. Incluir resultado en `actions[]` del batch (`type: Domain_Entity_Telemetry_Captured`).
5. Fallo de emisión snapshot: **fail-soft** (log + acción `error` en `actions`); no tumbar umbrales Self-Healing ni el sello del estímulo telemetry.

Punto de inserción preferente: final de `process_telemetry_file_inner` (todos los caminos `ok: true` no-duplicate), una sola emisión por asset consumido.

## 6. Suscripciones y enrutado

### 6.1 `event-domain-subscriptions.json`

Añadir clave (sin tocar entradas CRUD de `Domain_Entity_Updated`):

```json
"Domain_Entity_Telemetry_Captured": [
  {
    "agent": "cumulo",
    "process": "memory-evolution-ingest",
    "intent": "Indexar snapshot de ejecución en LanceDB vía EvolutionProxy."
  }
]
```

Opcional fase posterior: suscriptor IOTA (fuera de alcance v1 si no hay umbral DLT definido).

### 6.2 `Domain_Entity_Updated`

**Sin cambios** de suscriptores. No se requiere `applies_to_lifecycle` en v1 (Plan B elimina la necesidad).

### 6.3 Filtro runtime

No obligatorio para v1. Si en el futuro se reutiliza `Domain_Entity_Updated`, entonces sí: `subscriber_applies_to_lifecycle` análogo a `subscriber_applies_to_topology`.

## 7. Proceso `memory-evolution-ingest`

| Campo | Valor |
|-------|-------|
| `name` | `memory-evolution-ingest` |
| Forja | `entity-manager` → `process-creator` |
| Input | `event_file_path` (relativo `./.events/domain/…`) |
| Runtime | Handler nativo en `execute-process` (residual / módulo dedicado) |

### Algoritmo

1. Leer JSON; validar `event_type == Domain_Entity_Telemetry_Captured` (o confiar en gate ECST previo de route-domain).
2. Serializar payload canónico → string para `EvolutionEvent`.
3. `metadata` = `{ success: execution_metrics.success_status, entity_id, entity_type, origin_stimulus, duration_ms, exit_code }`.
4. `EvolutionProxyService::capture_event(payload_str, metadata)` → polarity por `success`.
5. Persistencia vía `LanceDbEvolutionAdapter` (ver §8).
6. Sellar `delivery_state` del evento domain con clave suscriptor (`cumulo.memory-evolution-ingest` o convención fractal vigente).
7. **Opcional v1:** emitir `Vector_Memory_Indexed` si el cableado es trivial; si no, deuda explícita en `implementation.md` (AC3 se cumple con registro en store).

## 8. Persistencia LanceDB

| Capa | Estado actual | Objetivo v1 |
|------|---------------|-------------|
| `EvolutionEvent` / proxy | Existe | Sin cambio de modelo |
| `LanceDbEvolutionAdapter::store_event` | Mock `Ok(())` | Persistencia real mínima |
| Store path | `.SddIA/vector_store/evolution/` | Mantener |

**Mínimo aceptable (AC3):** escribir registro durable e idempotente bajo `.SddIA/vector_store/evolution/` (p.ej. un JSON por `EvolutionEvent.id` o append-log indexable) correlacionable por `entity_id` + métricas en `operational_metadata`. Bindings LanceDB nativos si el crate ya está en el workspace; si no, capa archivo-estructurada con la misma interface `EvolutionStore` y TODO de bindings en evolution log.

**Embeddings:** metadata-first; `embedding: None` aceptable en v1 (deuda documentada). Reutilizar `inference_binding` solo si coste ≤ trivial.

## 9. Criterios de aceptación (spec)

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC1 | Tras `Raw_Execution_Finished` consumido por Radamanto, existe JSON domain `Domain_Entity_Telemetry_Captured` | Inspección `./.events/domain/` o lab sync |
| AC2 | `route-domain` invoca `memory-evolution-ingest`; **no** dispara `sync-entity-index` por este evento | Logs / `delivery_state` / ausencia de side-effect index |
| AC3 | Registro en `.SddIA/vector_store/evolution/` con `entity_id` + métricas | Lectura filesystem o API adapter |
| AC4 | CRUD `Domain_Entity_Updated` / Created / Deleted sin regresión | Smoke emit-domain-mutation o suite existente |
| AC5 | Clase + proceso en EDA coverage (`orphan_count: 0`) | `sddia-qa audit-eda-coverage --scan` |
| AC6 | Cierre documental single-PR | PBI `done/` + `validacion.md` APTO + `pbi_archived: true` |

## 10. Fuera de alcance

- Mutar schema de `Domain_Entity_Updated`.
- Eliminar `telemetry_batch_stub` (Kaizen aparte).
- Cambiar umbrales / Self-Healing / compliance audit.
- Embeddings obligatorios / KNN search UI.
- Anclaje DLT del snapshot (salvo suscriptor IOTA opcional futuro).

## 11. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Fallo ingest tumba Radamanto | Fail-soft en emisión e ingest |
| Duplicados por re-consumo | Idempotencia por `origin_stimulus.event_id` o `asset_id` en store |
| Forja manual genoma | Solo `entity-manager` |
| Nombre feature vs event_type | Documentado en este laudo; no renombrar rama |

## 12. Touchpoints Tekton

| # | Artefacto |
|---|-----------|
| G1 | `SddIA/events/domain/domain-entity-telemetry-captured.md` (+ index) |
| G2 | `SddIA/process/memory-evolution-ingest.md` (+ index) |
| G3 | `SddIA/core/event-domain-subscriptions.json` |
| R1 | `radamanto_batch_core.rs` — emisión snapshot |
| R2 | Handler nativo `memory-evolution-ingest` |
| R3 | `lancedb_evolution_repo` — store real mínimo |
| R4 | Tests / smoke lab |
| D1 | `eda-coverage.json` vía emit-domain-mutation |
| D2 | `implementation.md`, `execution.md`, `validacion.md`, PBI → `done/` |
