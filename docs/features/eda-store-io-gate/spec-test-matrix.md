---
feature_name: eda-store-io-gate
artifact: test-matrix
created: "2026-10-30"
scope: "Matriz escritura/lectura ECST × StoreMode × familia runtime"
---

# Spec — Matriz de tests por `event_type`

## Inventario (SSOT)

| Registro | Claves `event_type` | Familia store | Ruta FS lógica | Consumo |
|----------|---------------------|---------------|----------------|---------|
| `event-subscriptions.json` | 35 (bus dominio pending) | `domain` vía `publish_ecst(..., "pending", …)` | `.events/pending/{id}.json` | `route-domain-event` |
| `event-domain-subscriptions.json` | 40 (fractal domain) | `domain` | `.events/domain/{id}.json` | `route-domain` |
| `event-telemetry-subscriptions.json` | 3 suscritos + `System_Vitality_Probed`* | `telemetry` | `.events/telemetry/{id}.json` | `route-telemetry` |
| `event-orchestration-subscriptions.json` | 2 suscritos + clases codice† | `orchestration` | `.events/orchestration/{id}.json` | `route-orchestration` |

\* `System_Vitality_Probed` está en códice telemetría; no figura en el JSON de suscripciones ( `_disposition: no_subscriber` ). Incluir fila **telemetry / sin fan-out**.

† `TQM_Dispatch_Discarded`, `Local_QA_Requested` (orquestación en códice; `Local_QA` también en domain-subscriptions): una fila cada una según emisor canónico documentado en la Clase.

**Total filas de matriz:** ~48 Clases + 2 tipos DI/piloto (`CapabilityDi_Requested`, `CapabilityDi_Resolved` ack) — agrupadas en olas (ver plan).

## Dimensiones de cada celda

| Dimensión | Valores |
|-----------|---------|
| `StoreMode` | `redb`, `filesystem`, `dual` |
| Operación | `write`, `read`, `write+read` |
| Assert escritura `redb` | Sin `{id}.json` en carpeta FS de la familia |
| Assert escritura `filesystem` | Fichero presente; store ausente o no consultado |
| Assert escritura `dual` | Fila en store **y** JSON espejo |
| Assert lectura | `load_dispatched_event(repo, id, logical_path)` → `Some(ecst)` con mismo `event_id` |

## Harness común (crate / integration)

Fixture mínimo (patrón `eda_subscriber_ecst_redb.rs`):

- Copia: `cumulo.paths.json`, `event-subscriptions.json`, `event-{telemetry,orchestration,domain}-subscriptions.json`, `capability-bindings.md` (si stamp).
- `override_store_mode` + reset en `Drop`.
- Helpers:
  - `minimal_ecst(event_type, payload) -> Value` con `delivery_state` acorde (suscriptores fake `tekton.lab` o `_disposition: no_subscriber`).
  - `assert_store_roundtrip(repo, family, event_type, fs_dir, mode)`.
  - `assert_no_fs_artifact(repo, path)`.

Generación de filas: script de lab (opcional) que liste claves JSON ≠ tests manuales; **no** commitear generador en genoma sin `entity-manager`.

## Criterios de aceptación de la matriz

| ID | Criterio |
|----|----------|
| TM-1 | Toda clave de `event-subscriptions.json`: `publish_ecst` + lectura en `redb` sin fichero pending. |
| TM-2 | Toda clave de `event-telemetry-subscriptions.json` + `System_Vitality_Probed`: roundtrip fractal telemetry. |
| TM-3 | Toda clave de `event-orchestration-subscriptions.json` + clases orquestación solo-codice: roundtrip fractal orchestration. |
| TM-4 | Toda clave de `event-domain-subscriptions.json` no duplicada en pending-only: roundtrip fractal domain + `route_fractal_event` sin fichero en `redb`. |
| TM-5 | Modo `filesystem`: cada ola mantiene JSON legible en disco (regresión AC-N1). |
| TM-6 | Modo `dual`: lectura prioriza store; JSON espejo existe. |
| TM-7 | Tipos `_disposition: no_subscriber` no exigen filas `subscriber_delivery` Pending. |
| TM-8 | Tipos con suscriptores reales: tras stamp, al menos una fila `list_pending_deliveries` (o Success si test acorta en load). |

## Tipos prioritarios (ola 0 — ya parcialmente cubiertos)

| event_type | Escritor prod. | Test existente |
|------------|----------------|----------------|
| `System_Fracture_Detected` | heartbeat, kalma2, route_domain | `orphan_lock_redb_*`, gate publish |
| `Kaizen_Alert_Required` | event-bus-audit | `kaizen_emit_redb_*` |
| `PullRequest_Presented` / `PullRequest_Audited` | delivery-close | `eda_subscriber_ecst_redb` (parcial) |
| `Work_Initiated` / `Tracker_Issue_Changed` | forge/refine/linear | `eda_subscriber_ecst_redb` |
| `Domain_Entity_Telemetry_Captured` | radamanto | `memory_evolution_ingest_*` |
| `CapabilityDi_Requested` | capability_di_reactor | `ac_r6_*` (filesystem) |
| `Thought_Persisted` | lancedb adapter | `store_thought_emits_*` (filesystem) |
| `Raw_Execution_Finished` | thermodynamic | **gap** |
| `Daemon_Heartbeat` | thermodynamic / side-channel | watcher claim + side-channel tests |
| `Process_Execution_Completed` | thermodynamic PEC | **gap** |

## Ubicación de tests (propuesta)

| Ola | Crate / path | Comando |
|-----|--------------|---------|
| 0 | `sddia-event-store/tests/gate.rs` | `cargo test -p sddia-event-store --test gate` |
| 1 | `engine/execute-process/tests/eda_store_io_matrix_pending.rs` (nuevo) | `cargo test -p execute-process --test eda_store_io_matrix_pending` |
| 2 | `engine/execute-process/tests/eda_store_io_matrix_fractal.rs` (nuevo) | `cargo test -p execute-process --test eda_store_io_matrix_fractal` |
| 3 | `daemons/event-watcher` (tests existentes + 1 fila por familia claim) | `cargo test -p event-watcher` |
| 4 | `dual` subset en `eda_store_io_matrix_pending` con `StoreMode::Dual` | mismo binario de test |

## Definition of Done (matriz)

```text
Done = olas 1–2 mergeadas en main
     + TM-1..TM-8 verdes en validacion.md de eda-store-io-gate (o extensión test-matrix)
     + sin duplicar 48×3 tests manuales: tabla en spec-test-matrix § Cobertura actualizada
```
