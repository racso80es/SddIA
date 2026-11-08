---
feature_name: man-38-redb-port-lock
created: "2026-11-08"
process: bug-fix
base: main
scope: ciclo-vida-acotado-EventManagerPort-redb-lock
branch_name: fix/man-38-redb-port-lock
persist_ref: docs/fixes/man-38-redb-port-lock
pbi_ref: MAN-38
document_id: PBI-90397f23
tracker_ref: MAN-38
historia_ref: MAN-35
execution_id: "0c5d6ea9-b949-4b2e-859f-6f225504cdf3"
agent: dedalo
phase: diseno-fix
status: design-ready
norm_ref: features-documentation-pattern@1.3.1
related_norma: docs/features/event-manager-port-cutover/norma-redb-exclusiva.md
tracker_channel: ".SddIA/state/tracker-outbound/MAN-38.jsonl"
---

# Spec — man-38-redb-port-lock (MAN-38)

Entrada: `objectives.md` + PBI refinado `PBI-90397f23` / canal **MAN-38** (`historia_ref` **MAN-35**).
No reabre el contrato `eda.event_manager` 1.2 ni la norma Redb exclusiva: solo acota el **lifetime** del adapter productivo.

`plan.md` sí: descomposición T1–T5 para Tekton (un commit/tarea). No hay blueprint de proceso nuevo ni mutación de genoma.

## Hecho

`StoreMode` productivo ya es Redb vía `EventManagerPort` (MAN-35). El lock físico de `.SddIA/runtime/events.redb` vive mientras exista el `RedbEventAdapter` abierto (`redb::Database::create`). Hoy ese adapter se cachea de forma que **procesos largos y el watcher** retienen el descriptor exclusivo aunque no estén en una op EDA.

| ID | Defecto | Evidencia viva |
|----|---------|----------------|
| D1 | `execute-process/src/main.rs` llama `eda_port::bootstrap_composition_root(&repo)` en `run_main`, `run_tool_cli`, `run_action_cli` y flags `--publish-ecst` / query **antes** de ejecutar la lógica | `main.rs` |
| D2 | `engine::run_process` vuelve a hacer `bootstrap_composition_root` al entrar | `engine/mod.rs` |
| D3 | `eda_port` guarda `Arc<dyn EventManagerPort>` productivo en `thread_local!` (`PORT` / `PORT_REPO`); `install_productive_port` / `with_port` / `port_arc` reutilizan el Arc sin dropear al terminar la op | `eda_port.rs` |
| D4 | Un `pull-request-review` (u otro proceso largo) permanece propietario del FD aunque fases no usen eventos → `forge-pbi` y peers agotan `SDDIA_EVENT_MANAGER_OPEN_BUDGET_MS` (~2 s) con `Database already open. Cannot acquire lock.` | incidente PBI + D1–D3 |
| D5 | `event-watcher::run_watcher` abre un puerto al arrancar; `claim_store_pending` clona ese `Arc` en el hilo de `invoke_route_process` y hace `record_delivery` tras la ruta → lock retenido durante toda la hija | `daemons/event-watcher/src/main.rs` |

Cuerpo Linear MAN-38: canal outbound + PBI forjado/refinado bastan (mismo patrón L-TRACKER que MAN-35; `lab-linear-store.json` no materializa el issue).

## Corte

### C1 — Composition lazy (CLI + `run_process`)

1. **Eliminar** llamadas incondicionales a `bootstrap_composition_root` en:
   - `run_main` / `run_tool_cli` / `run_action_cli` (salvo que la ruta invoque ya una op EDA vía helpers scoped).
   - `engine::run_process` (línea de bootstrap al inicio).
2. Las rutas CLI que **sí** son EDA (`--publish-ecst`, `--query-events`, `--inspect-dead-letter`, `--replay-dead-letter`, `--reset-subscriber-delivery`) deben usar el helper scoped de C2 (abrir → op → dropear), **no** instalar thread-local de larga duración.
3. `bootstrap_composition_root` / `install_productive_port` dejan de ser el camino caliente productivo. Si se conservan, quedan documentados como **deprecated / test-only** o se redirigen a no-op / panic en productivo (elegir Tekton: preferible deprecar + tests que fallen si alguien reinstala cache productiva).

### C2 — Puerto productivo scoped vs mock de test

Invariante de ownership:

| Rol | Lifetime |
|-----|----------|
| Productivo (`resolve_productive_port` / `RedbEventAdapter`) | Abrir → **una** operación del trait (o un lote atómico claim/list acotado) → dropear `Arc` / adapter al salir del scope. **Prohibido** cachear en `thread_local` más allá de la op. |
| Test (`install_port` + mock / `resolve_test_port`) | Puede permanecer instalado en el alcance del test; `clear_port` / `ParallelLabGuard` siguen siendo la higiene de lab. |

Cambios en `eda_port` + host:

1. Añadir helper canónico (nombre orientativo):

```rust
/// Abre puerto productivo, ejecuta `f`, dropea el adapter al salir.
/// No escribe en thread_local productivo.
pub fn with_productive_port<R>(
    repo: &Path,
    op_label: &str, // telemetría CA-8
    f: impl FnOnce(&dyn EventManagerPort) -> Result<R, String>,
) -> Result<R, String>
```

   Implementación: `resolve_productive_port(repo)` (retry acotado existente en host) → `f(port.as_ref())` → drop del `Arc` al final del bloque. Emitir telemetría de contención (C5) en open fail / wait.

2. Reescribir `with_port` / wrappers públicos (`publish_ecst`, `record_ecst`, `load_dispatched`, `append_progress`, `list_*`, `record_delivery_ack`, `move_to_dlq`, `purge_event`, `verify_inline_ecst`, `seal_dlt`, `get_dlt_seal`, …):
   - Si hay puerto **inyectado de test** en `thread_local` (`PORT` con `PORT_REPO == None` tras `install_port`, o flag interno `InstalledKind::Test`) → usar ese Arc sin abrir Redb.
   - Si no → **siempre** `with_productive_port` (scoped); **no** llamar `ensure_port_for_repo` / `install_productive_port`.
3. `port_arc(repo)` productivo: o se elimina de consumidores calientes, o se documenta como anti-patrón y se restringe a test; Tekton debe auditar callers y migrarlos a scoped.
4. Conservar `open_redb_adapter` retry (`SDDIA_EVENT_MANAGER_OPEN_BUDGET_MS`, default 2000) como **protección transitoria**; no subir el presupuesto como «solución».

### C3 — `event-watcher`: scopes claim / route / ACK

1. **Quitar** `let port = Arc::new(resolve_productive_port(...))` de larga duración en `run_watcher`.
2. `claim_store_pending`:
   - Abrir puerto scoped → `claim_pending` → dropear **antes** de spawnear hilos de ruta.
   - En el hilo hijo: `invoke_route_process` **sin** capturar `Arc` de Redb.
   - Tras el resultado: abrir **otro** puerto scoped breve → `record_delivery` ACK/NACK → dropear.
3. `claim_store_fractal` y cualquier otra ruta que reciba `port: Arc<dyn EventManagerPort>` se alinean al mismo patrón (o dejan de recibir Arc).
4. Mantener `routes_block_store_claim` (AUT-67): no claim mientras hay rutas in-flight; eso **complementa** C3, no lo sustituye.
5. Semántica lease/ACK/NACK/DLQ: sin cambios de contrato; solo lifetime del handle. CA-5 exige integración Redb real.

### C4 — Auditoría de poseedores `Arc` productivo

Revisar daemons/tools/hosts que resuelven y guardan `Arc<dyn EventManagerPort>` o llaman `bootstrap_composition_root` / `port_arc`:

| Locus (mínimo) | Acción |
|----------------|--------|
| `daemons/event-watcher` | C3 obligatorio |
| `execute-process` CLI + `eda_port` | C1–C2 |
| Otros daemons (`github-bridge-watcher`, bridges, tools audit/purger) | Migrar a scoped **salvo** justificación documentada: sesión persistente + prueba + presupuesto explícito en este `persist_ref` (sección «Sesiones persistentes excepcionales»). Sin justificación → scoped. |

No inventar poseedores: Tekton lista evidencia grep/`rg` en execution.

### C5 — Documentación de ownership + telemetría de contención

1. Documento corto bajo este persist (recomendado: `docs/fixes/man-38-redb-port-lock/port-lifetime.md`) o sección en implementation:
   - Quién puede abrir Redb (solo host/adapter).
   - Lifetime = una op (o lote claim acotado).
   - Mock vs productivo.
   - Retry = escudo, no diseño.
2. Telemetría (stderr estructurado o canal existente del host/watcher) en contención de open:
   - campos mínimos: `process` (nombre proceso/daemon), `operation` (`op_label` / trait op), `wait_ms`, `budget_ms`, `outcome` (`ok`\|`locked`).
   - Debe permitir correlacionar víctima (`forge-pbi`) con poseedor residual si aún aparece tras el fix.

## Criterios de aceptación

| ID | Criterio | Corte |
|----|----------|-------|
| **CA-1** | `execute-process --process pull-request-review` sin op EDA en curso **no** mantiene abierto `events.redb` | C1–C2 |
| **CA-2** | Tras publicación/consulta EDA puntual en proceso largo, el descriptor se libera antes de la siguiente fase larga | C2 |
| **CA-3** | Watcher no mantiene puerto durante `invoke_route_process`; claim y ACK/NACK = scopes separados | C3 |
| **CA-4** | Con PPR largo + watcher activo, `forge-pbi` crea/sella sin agotar presupuesto normal de apertura | C1–C3 |
| **CA-5** | ACK/NACK, lease, DLQ y consistencia de entregas bajo integración Redb real | C3 |
| **CA-6** | Unit: productivo no queda cacheado en thread-local; mock sigue inyectable | C2 |
| **CA-7** | Sin `SDDIA_EVENT_STORE`, sin `EventStore` directo en consumidores, sin fallback FS productivo, sin relajar MAN-35 | todos |
| **CA-8** | Ownership/lifetime documentado + telemetría de contención con proceso y operación | C5 |

## Fuera de alcance

- Sustituir Redb por servidor externo o cambiar contrato EventManagerPort 1.2.
- Aumentar a ciegas el presupuesto de reintento como solución al bloqueo.
- Matar PPR/watcher como recuperación normal.
- Reintroducir `EventStore` / FS / dual / `SDDIA_EVENT_STORE` productivo.
- Blueprint de proceso nuevo o mutación Genoma (`SddIA/process/`, normas de contrato).
- Semillas Kaizen en `docs/todos/` (solo Cumulo / `Kaizen_Alert_Required`).

## Touchpoints (Tekton)

| Módulo | Acción |
|--------|--------|
| `SddIA/engine/execute-process/src/main.rs` | Quitar bootstrap incondicional; rutas EDA → scoped |
| `SddIA/engine/execute-process/src/engine/mod.rs` (`run_process`) | Quitar bootstrap incondicional |
| `SddIA/engine/execute-process/src/engine/eda_port.rs` (+ tests) | Scoped productivo; mock intacto; deprecar cache productiva |
| `SddIA/sddia-event-manager-host/src/lib.rs` (+ tests) | Helper `with_productive_port` / telemetría open; conservar retry |
| `SddIA/daemons/event-watcher/src/main.rs` (+ tests integración) | Claim / route / ACK scopes |
| Daemons/tools con `resolve_productive_port` / `port_arc` / bootstrap | Auditoría C4 |
| `docs/fixes/man-38-redb-port-lock/port-lifetime.md` (o equiv.) | CA-8 |

## Remisiones

| Ítem | Destino |
|------|---------|
| Norma Redb exclusiva | `docs/features/event-manager-port-cutover/norma-redb-exclusiva.md` |
| Cutover puerto 1.2 | MAN-35 / `docs/features/event-manager-port-cutover/` |
| Retry histórico store | AUT-16 / `docs/fixes/eda-redb-lock` (presupuesto; no lifetime) |
| Contención claim vs rutas | AUT-67 (`routes_block_store_claim`) |
| Semilla | **PBI-90397f23** / tracker **MAN-38** |
