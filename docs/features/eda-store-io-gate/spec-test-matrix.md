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

## Ubicación de tests

| Ola | Crate / path | Comando | Rama |
|-----|--------------|---------|------|
| 0 | `sddia-event-store/tests/gate.rs` | `cargo test -p sddia-event-store --test gate` | main / rama |
| 1 | `engine/execute-process/tests/eda_store_io_matrix_pending.rs` + `tests/common/mod.rs` | `cargo test -p execute-process --test eda_store_io_matrix_pending -- --test-threads=1` | `ed9137a8` |
| 2 | `engine/execute-process/tests/eda_store_io_matrix_fractal.rs` | `cargo test -p execute-process --test eda_store_io_matrix_fractal -- --test-threads=1` | `ed9137a8` |
| 3 | `daemons/event-watcher` (claim fractal; smoke parcial vía route en ola 2) | `cargo test -p event-watcher` | parcial |
| 4 | `dual` subset en `eda_store_io_matrix_pending` | mismo binario | `ed9137a8` (6 tipos) |

<!-- MATRIX-COVERAGE:START — generado 2026-10-30; no editar a mano -->
## Cobertura actual (generada)

Comando: `./docs/features/eda-store-io-gate/gen-test-matrix-table.sh`

Resumen: **48** `event_type` distintos · pending_bus **41** · telemetría **4** · orquestación **4**

Gaps PTC explícitos (fuera matriz ECST): side-channel latidos; `progress_traces` — ver `objectives-test-matrix.md`.

| `event_type` | Registro(s) | TM-1 | TM-2 | TM-3 | TM-4 | TM-5 | TM-6 | TM-7 | TM-8 |
|--------------|-------------|------|------|------|------|------|------|------|------|
| `Aiua_Process_Requested` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `CI_Chronic_Failure_Detected` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `CI_Job_Failed` | telemetry-subscriptions | — | APTO | — | — | — | — | fan-out | parcial |
| `CapabilityDi_Requested` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `Daemon_Heartbeat` | telemetry-subscriptions | — | APTO | — | — | APTO | — | fan-out | parcial |
| `Delivery_Committed` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `Domain_Entity_Created` | subscriptions, domain-subscriptions | APTO | — | — | muestra | APTO | APTO | fan-out | parcial |
| `Domain_Entity_Degraded` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `Domain_Entity_Deleted` | subscriptions, domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `Domain_Entity_Deprecated` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `Domain_Entity_Restored` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `Domain_Entity_Telemetry_Captured` | subscriptions, domain-subscriptions | APTO | — | — | APTO | APTO | gap | fan-out | parcial |
| `Domain_Entity_Updated` | subscriptions, domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `Email_Quick_Action_Requested` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `Email_Received` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `Email_Triaged` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `Fracture_Clarification_Requested` | orchestration-subscriptions | — | — | APTO | — | — | — | fan-out | parcial |
| `Fracture_Pbi_Materialized` | subscriptions, domain-subscriptions | APTO | — | — | APTO | APTO | gap | fan-out | parcial |
| `HU_Refined` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `Instance_Deployed` | subscriptions, domain-subscriptions | APTO | — | — | muestra | APTO | gap | APTO | N/A |
| `Instance_Torn_Down` | subscriptions, domain-subscriptions | APTO | — | — | muestra | APTO | gap | APTO | N/A |
| `Kaizen_Alert_Required` | subscriptions, domain-subscriptions | APTO | — | — | muestra | APTO | APTO | fan-out | parcial |
| `Kaizen_Idea_Captured` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | APTO | N/A |
| `Kalma2_Process_Requested` | domain-subscriptions | APTO | — | — | APTO | APTO | gap | fan-out | parcial |
| `Local_QA_Requested` | domain-subscriptions | APTO | — | APTO | APTO | APTO | gap | fan-out | parcial |
| `Manual_Task_Requested` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | APTO | N/A |
| `PBI_Cancelled` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `PBI_Forged` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `PBI_Refined` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `Process_Execution_Completed` | orchestration-subscriptions | — | — | APTO | — | — | — | fan-out | parcial |
| `PullRequest_Audited` | subscriptions, domain-subscriptions | APTO | — | — | muestra | APTO | gap | APTO | N/A |
| `PullRequest_Merged` | subscriptions, domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `PullRequest_Presented` | subscriptions, domain-subscriptions | APTO | — | — | muestra | APTO | APTO | fan-out | APTO |
| `PullRequest_Resealed` | subscriptions, domain-subscriptions | APTO | — | — | muestra | APTO | gap | APTO | N/A |
| `Raw_Execution_Finished` | telemetry-subscriptions | — | APTO | — | — | APTO | — | fan-out | parcial |
| `Suite_Execution_Requested` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `System_Fracture_Detected` | subscriptions, domain-subscriptions | APTO | — | — | muestra | APTO | APTO | fan-out | parcial |
| `System_Immunity_Certified` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `System_Vitality_Probed` | — | — | APTO | — | — | — | — | APTO | N/A |
| `TQM_Dispatch_Discarded` | — | — | — | APTO | — | — | — | APTO | N/A |
| `TelegramCallback_Received` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `TelegramMessage_Received` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `Thought_Persisted` | subscriptions, domain-subscriptions | APTO | — | — | muestra | APTO | APTO | APTO | N/A |
| `Tracker_Issue_Changed` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `Tracker_Sync_Failed` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `User_Preference_Change_Requested` | domain-subscriptions | APTO | — | — | muestra | APTO | gap | fan-out | parcial |
| `Vector_Memory_Indexed` | subscriptions, domain-subscriptions | APTO | — | — | muestra | APTO | gap | APTO | N/A |
| `Work_Initiated` | domain-subscriptions | APTO | — | — | muestra | APTO | APTO | fan-out | parcial |

Leyenda: **APTO** = cubierto por test de integración en rama; **muestra** = subconjunto fractal domain; **parcial** = patrón no exhaustivo; **fan-out** = tiene suscriptores; **N/A** = sin suscriptores (TM-7).
<!-- MATRIX-COVERAGE:END -->


## Definition of Done (matriz)

```text
Done = olas 1–2 mergeadas en main
     + TM-1..TM-8 verdes en validacion.md de eda-store-io-gate (o extensión test-matrix)
     + sin duplicar 48×3 tests manuales: tabla en spec-test-matrix § Cobertura actualizada
```
