---
document_id: PBI-TELEMETRIA-ACTIVA-DOMAIN-ENTITY-UPDATED
title: "[OPERATIVO] PBI: Gestión e Ingesta de Telemetría Activa mediante Domain_Entity_Updated"
format: markdown
version: "1.2.0"
created: "2026-07-17"
refined: "2026-07-17"
dedalo: "2026-07-19"
status: done
laudo: plan_b_domain_entity_telemetry_captured
docs_synced: "2026-07-19"
closed: "2026-07-19"
priority: alta
process: feature
feature_name: telemetria-activa-domain-entity-updated
persist_ref: docs/features/telemetria-activa-domain-entity-updated
branch: feat/telemetria-activa-domain-entity-updated
pbi_ref: docs/todos/pending/[OPERATIVO] PBI: Gestión e Ingesta de Telemetría Activa mediante Domain_Entity_Updated.md
depends_on_features:
  - docs/features/telemetria-reactiva-eda-fase4
  - docs/features/adecuar-ed-telemetry
  - docs/features/memoria-vectorial
  - docs/features/boveda-evolucion-epigenetica
---

# PBI-TELEMETRIA-ACTIVA-DOMAIN-ENTITY-UPDATED

Estado: **Blueprint** (Dedalo 2026-07-19) — Plan B vinculante. Spec/plan en `persist_ref`.

## 1. Especificación (Spec)

Sustituir el consumo de telemetría que **evapora el rastro** tras `route-telemetry` → Radamanto por un flujo reactivo cerrado (Grado S+):

1. Tras procesar `Raw_Execution_Finished`, Radamanto materializa métricas de ejecución.
2. Emite chispa secundaria **`Domain_Entity_Telemetry_Captured`** (Plan B; no contaminar `Domain_Entity_Updated` CRUD).
3. Suscriptor `memory-evolution-ingest` indexa en LanceDB vía `EvolutionStore` / `lancedb_evolution_repo`.
4. Sweeper consolida tras sello `delivery_state`.

> **Laudo:** Plan A (`Domain_Entity_Updated` + `telemetry_snapshot`) rechazado — schema REQUIRED plano + gate ECST. Detalle: `docs/features/telemetria-activa-domain-entity-updated/spec.md`.

### Problema actual

| Hecho en `main` | Efecto |
|-----------------|--------|
| `event-telemetry-subscriptions.json` → `radamanto-batch` + `telemetry-compliance-audit` | Hot path = Radamanto, no stub |
| `telemetry_batch_stub` residual | Purga archivo y retorna `ok` sin persistencia semántica |
| Radamanto acumula stats en `.SddIA/radamanto/` y emite Degraded/Restored/Deprecated | No emite snapshot de ejecución indexable en memoria vectorial |
| `Domain_Entity_Updated` = CRUD genómico | Suscriptores `sync-entity-index` + IOTA; schema incompatible con snapshot telemetría sin discriminador |

## 2. Clarificación (Clarify) — decisiones selladas

### D1 — ¿Quién emite?

**Radamanto** (`process: radamanto-batch`), tras consumo válido de `Raw_Execution_Finished`.

- Prohibido: orquestador inerte como emisor.
- Prohibido: tratar `telemetry-batch-stub` como destino de implementación (fósil Fase 3; hot path ya es Radamanto).
- La «aduana de telemetría» = cadena `./.events/telemetry/` → `route-telemetry` → `radamanto-batch`.

### D2 — ¿Qué transporta el evento (ECST)?

Sobre desnormalizado en familia `domain`, tipo `Domain_Entity_Updated`, con **discriminador obligatorio** para no contaminar CRUD genómico:

| Campo | Obligatorio | Notas |
|-------|-------------|-------|
| `entity_id` | sí | UUID/asset del componente auditado (paridad Radamanto `target_entity` / `asset_id`) |
| `entity_type` | sí | Enum alineado a taxonomía agnóstica: `capsule` \| `agent` \| `tool` \| `process` \| `skill` \| … |
| `lifecycle_operation` | sí | Valor canónico propuesto: `telemetry_snapshot` (distinto de `create`\|`update`\|`delete`) |
| `execution_metrics` | sí | `duration_ms`, `exit_code`, `success_status` (bool o derivable) |
| `evolution_footprint` | opcional | hash diff artefactos si aplica |
| `timestamp` | sí | anclaje ISO del estímulo (puede coincidir con envelope) |
| `origin_stimulus` | sí | `{ "event_type": "Raw_Execution_Finished", "event_id": "…" }` |
| `hash_signature_old` / `hash_signature_new` | **no** para este discriminador | reservados a CRUD genómico |
| `state_before` / `state_after` | opcional | resumen métrico; no sustituyen schema CRUD |

Ejemplo envelope (orientativo):

```json
{
  "event_id": "88b78eb5-98e6-49ab-97aa-e807b57d2aec",
  "event_family": "domain",
  "event_type": "Domain_Entity_Updated",
  "timestamp": "2026-07-17T13:51:00Z",
  "emitter_agent": "radamanto",
  "payload": {
    "entity_id": "<uuid-o-asset>",
    "entity_type": "agent",
    "lifecycle_operation": "telemetry_snapshot",
    "origin_stimulus": {
      "event_type": "Raw_Execution_Finished",
      "event_id": "<uuid-telemetria>"
    },
    "execution_metrics": {
      "duration_ms": 142,
      "exit_code": 0,
      "success_status": true
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

### D3 — ¿Quién gestiona / persiste?

| Capa | Ruta SSOT (Cúmulo / genoma) |
|------|------------------------------|
| Puerto | `SddIA/core/memory` → `EvolutionStore` + `EvolutionProxyService` |
| Adaptador | `SddIA/infrastructure/adapters/lancedb_evolution_repo` |
| Store físico | `.SddIA/vector_store/evolution/` (connection_string del adaptador) |
| Modelo | `EvolutionEvent` (`id` SHA-256, `polarity`, `payload`, `operational_metadata`, `embedding`) — **no** el pseudotipo `EvolutionNode` del borrador v1.0 |

Tras indexación exitosa: emitir `Vector_Memory_Indexed` (clase existente) si el adaptador/proxy ya lo contemplan; no inventar sello paralelo.

### D4 — Bus y suscripciones

| Incorrecto (borrador v1.0) | Correcto |
|----------------------------|----------|
| `.SddIA/events/pending/domain_entity_updated_[UUID].json` | `./.events/domain/<event_id>.json` (fractal Cúmulo `eda_fractal.domain`) |
| Solo ampliar mapa sin filtros | Añadir suscriptor memory **y** filtrar `sync-entity-index` / IOTA para que **ignoren** `lifecycle_operation: telemetry_snapshot` |

Suscripción objetivo (conceptual):

```json
"Domain_Entity_Updated": [
  { "agent": "cumulo", "action": "sync-entity-index", "applies_to_lifecycle": ["create", "update", "delete"] },
  { "agent": "cumulo", "tool": "iota-immutable-publisher", "applies_to_lifecycle": ["create", "update", "delete"], "applies_to_origin_topology": ["core"] },
  { "agent": "cumulo", "process_or_handler": "memory-evolution-ingest", "applies_to_lifecycle": ["telemetry_snapshot"], "intent": "Indexar snapshot en LanceDB vía EvolutionProxy" }
]
```

*(Nombre exacto del handler: Dedalo/Tekton; debe residir en runtime nativo, no en agente IDE.)*

### D5 — Relación con Self-Healing

- `Domain_Entity_Degraded` / `Restored` / `Deprecated` **siguen** gobernados por umbrales Radamanto (sin cambio de contrato).
- Este PBI añade el **snapshot de cada ejecución** (trazabilidad vectorial), ortogonal al umbral de degradación.
- No sustituye `telemetry-compliance-audit` ni recibos termodinámicos.

### D6 — Laudo Dedalo (cerrado)

**Plan B aprobado.** Clase `Domain_Entity_Telemetry_Captured`. Plan A descartado.

## 3. Plan de Ejecución (Plan)

SSOT operativo: `docs/features/telemetria-activa-domain-entity-updated/plan.md` (pasos T0–T5).

| Fase | Qué | Gate |
|------|-----|------|
| **T0** | Forjar ECST + proceso `memory-evolution-ingest` | Genoma catalogado |
| **T1** | Suscripción domain → ingest | Clave en subscriptions |
| **T2** | Emisión en `radamanto_batch_core` | AC1 |
| **T3** | Handler nativo + store mínimo | AC2–AC3 |
| **T4–T5** | Tests, EDA, cierre documental | AC4–AC6 |

Fuera de alcance salvo deuda explícita: eliminar `telemetry_batch_stub` del residual_runner (Kaizen aparte).

## 4. Implementación (Implementation) — notas para Tekton

- Pseudocódigo v1.0 con `EvolutionNode { nature, entorno, entropia_asimilada }` → **descartado**; usar `EvolutionEvent` + `EvolutionProxyService::capture_event`.
- Prohibido forjar genoma a mano: `entity-manager` / creators según DA-2–DA-3.
- Persistencia: **no** archivos plano como SSOT de métricas; LanceDB vía adaptador.
- Embeddings: reutilizar `inference_binding` del core memory si existe ruta; si no, metadata sin embedding con deuda documentada.

## 5. Validación (Validacion)

| ID | Criterio |
|----|----------|
| AC1 | Ejecución que genere `Raw_Execution_Finished` produce chispa `Domain_Entity_Updated` con `lifecycle_operation: telemetry_snapshot` en `./.events/domain/` |
| AC2 | Watcher/route-domain enruta snapshot al handler memory (log/sello), **sin** invocar `sync-entity-index` |
| AC3 | Registro consultable en store evolution (adapter/LanceDB) correlacionable por `entity_id` + métricas |
| AC4 | CRUD genómico `Domain_Entity_Updated` (`create`/`update`/`delete`) sin regresión |
| AC5 | Cierre documental: PBI → `docs/todos/done/` + `validacion.md` APTO + `pbi_archived: true` en la rama del PR |

### Smoke esperado (orientativo)

```text
[WATCHER] Detectado: domain/<event_id>.json (Domain_Entity_Updated / telemetry_snapshot)
         → memory-evolution-ingest → EvolutionProxy / LanceDB
```

## 6. Referencias

- Feature: `docs/features/telemetria-activa-domain-entity-updated/`
- Init: `_init-feature.json`
- Clarify transcript: `clarify.md`
- Contrato actual: `SddIA/events/domain/domain-entity-updated.md` v1.1.0
- Suscripciones: `SddIA/core/event-domain-subscriptions.json`, `SddIA/core/event-telemetry-subscriptions.json`
- Emisor hot path: `SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs`
- Memoria: `SddIA/core/memory/src/services/evolution_proxy.rs`
- Adapter: `SddIA/infrastructure/adapters/lancedb_evolution_repo/`
