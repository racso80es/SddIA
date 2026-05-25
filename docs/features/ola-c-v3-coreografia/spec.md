---
feature_name: ola-c-v3-coreografia
created: "2026-05-22"
updated: "2026-05-25"
process: feature
base: main
scope: eda-bus-subscriber-topology-v3plus
supersedes_topology: docs/events/
priority: P4
manifest: docs/todos/done/[ARQUITECTURA] Especificación Técnica Avanzada_ El Genoma de Eventos y Coreografía Asíncrona (Ola C) V3.md
related_refactor: docs/features/refactor-topologia-eventos-ola-c-v3/
upstream_prs: [24, 25, 27, 29]
---

# Especificación técnica — Ola C V3+: Coreografía Asíncrona (Estado de Suscriptores)

> **Estado (2026-05-25):** Código en `main` vía PRs #24–#29. Esta spec consolida la visión coreográfica V3 con la topología **simétrica V3+** entregada en `refactor-topologia-eventos-ola-c-v3`.

## 1. Contexto

Ola C entregó genoma de Evento (`SddIA/events/`) y bus runtime monolítico bajo `docs/events/` con `delivery_state` embebido en el JSON padre (ledger simplificado Ola A).

**Ola C V3/V3+** refactoriza la topología física del bus a `./.events/` (raíz del workspace), descarta el **Evento Padre Mutante** y externaliza la trazabilidad a:

1. **Padre inmutable** en `pending/[UUID].json`.
2. **Cabeceras réplica** por estado (`processing/`, `processed/`, `dead-letter/`).
3. **Testigos atómicos** por suscriptor bajo `{estado}/subscribers/`.

El JSON padre en `pending/` **no se modifica** durante el procesamiento. La recolección del padre es **diferida** (sweeper + sweep inline al cierre de route).

## 2. Principios (dogma operativo)

| Principio | Regla |
|-----------|-------|
| Inmutabilidad del padre | `pending/[UUID].json` no se muta tras emisión |
| Trazabilidad por suscriptor | Testigo `[UUID].[subscriber_id].json` en `*/subscribers/` |
| Desacoplamiento | Middleware (`promote_witness`) mueve testigos; route no muta padre |
| Recolección diferida | `try_sweep_event` / `event-sweeper.py` purgan padre solo con consenso `processed/` |
| Fallo asimétrico | Testigos en `dead-letter/subscribers/`; padre intacto hasta Kaizen terminal o sweep |
| Ejecución táctica | Agente lee evento, ejecuta lógica, devuelve resultado al middleware; cero estado en padre |

## 3. Topología SSOT (`cumulo.paths.json`)

### 3.1 Esquema canónico (V3+)

```json
{
  "event_bus": "./.events",
  "eda_bus": {
    "pending": "./.events/pending",
    "processing": "./.events/processing",
    "processed": "./.events/processed",
    "dead_letter": "./.events/dead-letter",
    "subscriptions": "SddIA/core/event-subscriptions.json"
  }
}
```

`load_eda_bus()` resuelve claves planas `{estado}_subscribers` → `{estado}/subscribers/`. Alias legacy `subscriber_*` redirigen a `*_subscribers` durante transición.

### 3.2 Árbol físico

```
.events/                              ← gitignored (/.events/)
  pending/
    [UUID].json                       ← ECST crudo (inmutable hasta sweep)
  processing/
    [UUID].json                       ← cabecera réplica (copia del pending)
    subscribers/
      [UUID].[subscriber_id].json     ← bloqueo en vuelo
  processed/
    [UUID].json                       ← cabecera si no existía en este estado
    subscribers/
      [UUID].[subscriber_id].json     ← éxito confirmado
  dead-letter/
    [UUID].json
    subscribers/
      [UUID].[subscriber_id].json     ← fallo operativo / ECST
```

### 3.3 Elementos obsoletos (no reimplementar)

| Elemento PDF / backlog original | Sustituto en runtime |
|---------------------------------|----------------------|
| `docs/events/` como bus | `./.events/` vía Cúmulo |
| Recibos `[UUID].[PURPOSE].notificado` | Testigos `[UUID].[subscriber_id].json` |
| Subcarpetas `receipts/` | `{estado}/subscribers/` |
| Middleware `.procesado` / `.error` | Promoción JSON `state: processed \| dead-letter` |
| **Evento Padre Mutante** (`delivery_state` en disco) | Testigos; `delivery_state` legacy solo en emisión (`{}`) y lectura in-memory IOTA |

## 4. Contrato de cabecera y testigos

### 4.1 Entrada en processing

Al iniciar enrutamiento de `[UUID]`:

1. Leer `pending/[UUID].json`.
2. Crear `processing/[UUID].json` (copia atómica) si ausente.
3. Por cada suscriptor: `write_processing_witness` → `processing/subscribers/[UUID].[sid].json`.
4. **No** modificar `pending/`.

**Esquema testigo (processing):**

```json
{
  "event_uuid": "[UUID]",
  "subscriber": "[subscriber_id]",
  "state": "processing",
  "started_at": "ISO-8601 UTC",
  "event_type": "Domain_Entity_Created",
  "dispatch_mode": "sync|async"
}
```

`subscriber_id` = `{agent}.{process|action|tool}` (ej. `cumulo.sync-entity-index`).

### 4.2 Promoción con éxito

`processing/subscribers/…` → `processed/subscribers/…`

- Añade `completed_at`, `result_status`, `delegation`.
- Crea cabecera `processed/[UUID].json` si ausente (copia desde pending).

### 4.3 Promoción con fallo

`processing/subscribers/…` → `dead-letter/subscribers/…`

- Añade `failed_at`, `error_trace`.
- Crea cabecera `dead-letter/[UUID].json` si ausente.
- Padre en `pending/` **permanece**.

### 4.4 Purga cabecera processing

Tras promoción terminal, si todos los suscriptores requeridos están en `processed/subscribers/` o `dead-letter/subscribers/` y no hay in-flight: eliminar `processing/[UUID].json` (`maybe_purge_processing_header`).

## 5. Orquestación

| Actor | Rol |
|-------|-----|
| `event-watcher.py` | Monitoriza `pending/`; delega en proceso `route-domain-event` |
| `route_domain_event_core.py` | ECST gate, fan-out sync/async, testigos, sweep inline |
| `execute-process` + `route-domain-event.md` | Proceso SddIA orquestador (acción legacy retirada PR #27) |
| `event-sweeper.py` | Daemon inerte; escanea `pending/`; alerta Kaizen; purga stale |

### 5.1 Fan-out

- **Default:** async (`ThreadPoolExecutor`, máx. 8 workers).
- **Regresión CI:** `SDDIA_LAB_ROUTE_SYNC=1` fuerza secuencial.

### 5.2 Recolección (doble vía)

```
PARA cada [UUID].json EN pending/:
  required ← suscriptores aplicables (event-subscriptions + origin_topology)
  dead     ← testigos en dead-letter/subscribers/
  done     ← testigos en processed/subscribers/
  in_flight← testigos en processing/subscribers/

  SI dead Y todos required terminales:
    Kaizen terminal → finalize_kaizen_terminal (retira pending, conserva DL)

  SI dead Y suscriptores pendientes:
    Alerta Kaizen (stderr JSON); NO purgar pending

  SI required ⊆ done Y NOT in_flight:
    archive_event_after_sweep → purga pending + cabeceras + testigos processed
```

**Sweep inline:** `route_domain_event` invoca `try_sweep_event` al cierre (PR #29). **Sweeper daemon:** recolector periódico para eventos stale (`POLL_SECONDS=5`).

## 6. Bootstrap

`ensure_event_bus_topology()` crea idempotentemente las 7 rutas (pending + 3 estados × cabecera + subscribers). Invocado al arranque de watcher, sweeper y route.

## 7. `.gitignore`

```
/.events/
```

## 8. `delivery_state` — contrato legacy

| Fase | Comportamiento |
|------|----------------|
| Emisión ECST | Campo opcional `{}` en plantillas; no es fuente de trazabilidad V3+ |
| Route (IOTA) | Lectura **in-memory** de `delivery_state.transaction_digest` post-dispatch; no persiste en padre |
| Verificación éxito bus | Criterio = testigos en `processed/subscribers/`, no `delivery_state` en JSON padre |

Norma: `SddIA/events/events-contract.md` — prohibido mutar padre tras emisión.

## 9. Consumidores

| Componente | Estado |
|------------|:------:|
| `cumulo.paths.json` | ✅ V3+ |
| `eda_bus_utils.py` | ✅ SSOT testigos + sweep |
| `route_domain_event_core.py` | ✅ Middleware + inline sweep |
| `event-watcher.py` | ✅ Proceso route-domain-event |
| `event-sweeper.py` | ✅ Daemon recolector |
| `emit-domain-mutation`, `emit-pr-*` | ✅ Emisión a `pending/` |
| `README.md`, `route-domain-event.md` | ✅ Documentados |

## 10. Verificación (Argos)

- [x] `event_bus` = `"./.events"` en Cúmulo
- [x] `.gitignore`: `/.events/`
- [x] Bootstrap 7 rutas simétricas
- [x] Testigo processing → processed/dead-letter sin mutar padre
- [x] Sweeper + inline sweep purgan con consenso
- [x] Alerta Kaizen ante dead-letter activo
- [x] E2E lab + unit tests `test_eda_bus_v3plus`
- [x] CI job `eda-bus-e2e-smoke` (simulate)

## 11. Trazabilidad

| Artefacto | Ref |
|-----------|-----|
| Backlog P4 | `docs/todos/pending/[OPERATIVO] Backlog pendiente post-PR11…` |
| Manifiesto V3 | `docs/todos/done/[ARQUITECTURA] … Ola C V3.md` |
| Delta topológico V3+ | `docs/features/refactor-topologia-eventos-ola-c-v3/` |
| Clarificación triaje | `docs/features/ola-c-v3-coreografia/clarify.md` |
| Suscripciones | `SddIA/core/event-subscriptions.json` |
