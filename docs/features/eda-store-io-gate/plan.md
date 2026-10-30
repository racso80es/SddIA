---
feature_name: eda-store-io-gate
created: "2026-10-30"
process: feature
branch_name: Kaicen_Auditoria_Eventos_261030
persist_ref: docs/features/eda-store-io-gate
status: IMPLEMENTED
phases:
  - fachada publish/load
  - escritores residuales
  - lectores residuales
  - consumo fractal desde store
  - installer y scripts
  - pruebas AC
related:
  - docs/audits/eda-events-redb-vs-filesystem-20261026.md
  - docs/features/eda-redb-02-writers/plan.md
  - docs/features/eda-redb-04-readers/plan.md
  - docs/fixes/event-bus-audit-kaizen-redb-emit/plan.md
---

# Plan — Cerrar I/O de eventos fuera del gate

Continúa EDA Redb 02 (escritores) y 04 (lectores). Esos PBI cubrieron el tronco (`record_ecst`, `claim_store_pending`, `load_dispatched_event` de dominio). Este plan cierra los **call sites que aún ignoran `StoreMode`**.

Alta de trabajo: `tracker.provider=linear` → `./sddia-run.sh --process forge-pbi` (no fichero en `docs/todos/`). Genoma (`*.md` de tool/process) solo vía `entity-manager` si una ficha cambia de contrato.

## Fase 0 — Fachada

Extraer el patrón ya repetido en `actions.rs`, `domain_mutation.rs`, `fractal.rs`, `linear_bridge::materialize_pending`:

- `publish_ecst`
- reutilizar `load_dispatched_event` como `load_ecst`
- `scan_domain(pred)` store ∪ FS según modo

Ubicación preferente: `sddia-event-store` solo si no necesita `cumulo.paths.json` (el crate no lo lee). Resolución de directorios FS permanece en el llamador; la fachada recibe `family` + `fs_dir: Option<&Path>`.

Tests de crate: tres modos, assert fichero sí/no y fila sí/no.

## Fase 1 — Escritores (AC-W1..W3)

Sustituir escritura directa por `publish_ecst`:

| Call site | Familia | Notas |
|-----------|---------|--------|
| `handlers/daemon_heartbeat.rs` `emit_system_fracture`, `emit_orphan_lock_fracture` | domain | Hoy `write_json_atomic` pending. Stamp + store. |
| `kalma2-bridge` `emit_system_fracture` | domain | Igual. El resto del bridge que ya usa store no se reabre. |
| `capability_di_reactor::emit_di_requested` | domain | Piloto DI. |
| `lancedb_thought_repo` persistencia `Thought_Persisted` | domain | El adapter no debe conocer `.events/pending` fijo. |

No tocar emisores que ya ramifican `writes_*` salvo para migrarlos a la fachada si el diff es mecánico y los tests existentes pasan.

## Fase 2 — Lectores (AC-R1, AC-R2)

| Call site | Cambio |
|-----------|--------|
| `eda_bus::find_existing_domain_event` | Si `writes_store`, `list_family(Domain)` y predicar `entity_uuid` + `lifecycle_operation`. FS solo si `writes_filesystem`. |
| `tracker_pbi_meta` (`tracker_ref_*`, `presented_event_id_for_branch`) | Barrido `.events/pending|processed` solo con `writes_filesystem` o `dual`. En `redb` puro, solo store (ya existe `visit_presented_in_store`). |
| `accept_pr::scan_presented_for_branch` | Misma regla. El bucle FS actual corre también en `redb` y debe quedar detrás del gate. |

## Fase 3 — Fractal (AC-F1, AC-F2)

Estado actual: `write_fractal_event` persiste en store; `event-watcher::watch_targets` hace `read_dir` de telemetry/domain/orchestration; `route_fractal_event` aborta si no hay fichero.

1. `claim_store_family(repo, family)` análogo a `claim_store_pending`: filas con entrega `Pending` de esa familia, path lógico, sin leer el ECST en el claim.
2. En `StoreMode::Redb`, el watcher **no** enruta por `read_dir` de esas carpetas. Un JSON nuevo ahí se trata como residual (mismo código de fricción o hermano `F-EDA-RESIDUAL-FRACTAL`), sin reinyectar.
3. `route_fractal_event`: si `!is_file` y `writes_store`, `load_dispatched_event`. Stamp de `delivery_state` en el JSON solo si `writes_filesystem` (ya parcialmente en `fractal_bus::stamp_fractal_delivery_state`); en `redb`, `record_witness` / `set_delivery`.
4. Suscriptores que hoy exigen `event_file_path` real (`radamanto-batch`, `audit_telemetry_file`): o cargan por id desde store, o el router les pasa el ECST ya materializado en memoria. No regenerar JSON en pending para “darles un path”.

Latidos: el side-channel sigue siendo la vía de `daemon-heartbeat-audit` (`start-sddia.sh`). Fase 3 no vuelve a emitir `Daemon_Heartbeat` a `.events/telemetry` en `redb`.

## Fase 4 — Installer y scripts (AC-W4)

| Artefacto | Acción |
|-----------|--------|
| `sddia-installer.sh` `_emit_instance_event` | Dejar de `mkdir` + JSON ciego. Invocar `execute-process` / binario que use `publish_ecst`, o leer `SDDIA_EVENT_STORE` y en `redb` no escribir FS. |
| `SddIA/tools/sddia-qa` `iota_ci_smoke::write_pending` | Mismo gate (herramienta de lab: en `redb` inserta store). |
| Scripts bajo `docs/` y `SddIA/scripts/qa/*` que fabrican `.events/pending` | Marcarlos **lab-only** con `SDDIA_EVENT_STORE=filesystem` en el shebang/comentario de invocación. No son runtime de instancia. |

## Fase 5 — Pruebas

Por AC, `cargo test` acotado (no suite completa de `execute-process`):

- `event-watcher`: claim fractal redb sin fichero; residual FS no enruta.
- `daemon_heartbeat`: fractura sin pending JSON (`override_store_mode(Redb)`).
- `eda_bus` idempotencia dominio solo-store.
- `route_fractal_core`: carga sin fichero.
- Regresión `SDDIA_EVENT_STORE=filesystem` en los tests que cuentan JSON (patrón ya usado en fracture-provider-sink).

`validacion.md` cuando la implementación esté en la rama: `global: APTO`, `branch` coherente. Con Linear, el issue en `done` al cerrar; sin fichero nuevo en `docs/todos/`.

## Orden de merge sugerido

Fase 0 → 1 y 2 en paralelo → 3 (depende de `load_ecst`) → 4. Cada fase puede ser un commit en `Kaicen_Auditoria_Eventos_261030` o PRs hijos si el diff de la fase 3 crece.

## Riesgos

- **Lock redb:** más `EventStore::open` en claim fractal. Reutilizar el presupuesto `SDDIA_EVENT_STORE_OPEN_BUDGET_MS`; no abrir una conexión por fichero.
- **Doble despacho en `dual`:** claim store y `read_dir` no deben enrutar el mismo `event_id` dos veces (libro `processing` / `routed_ok` ya existe; la clave es el uuid, no el path).
- **Suscriptores fractales acoplados a path:** inventario corto en fase 3 antes de cambiar la firma; si un suscriptor no puede vivir sin fichero, queda detrás de `writes_filesystem` y en `redb` se marca skipped-topology hasta portarlo (no silenciar el fallo).
