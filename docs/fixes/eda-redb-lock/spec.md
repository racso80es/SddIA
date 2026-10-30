---
feature_name: eda-redb-lock
created: "2026-10-26"
process: bug-fix
base: main
scope: contencion-lock-redb-presented-store-vitalidad-canal
branch_name: fix/eda-redb-lock
persist_ref: docs/fixes/eda-redb-lock
pbi_ref: AUT-16
document_id: PBI-5d3f2b1d
tracker_ref: AUT-16
execution_id: "0aab3583-5c34-40e9-ba20-d8d9a527d2b8"
audit_ref: docs/audits/eda-events-redb-vs-filesystem-20261026.md
---

# Spec — eda-redb-lock (AUT-16)

## Hecho

`StoreMode::Redb` es el default. Tres defectos lo contradicen (PBI `AUT-16` / `PBI-5d3f2b1d`; auditoría `AUDIT-EDA-REDB-20261026`):

| ID | Defecto | Evidencia |
|----|---------|-----------|
| D1 | `EventStore::open` = un solo `Database::create` sin reintento → `StoreError::Backend("Database already open. Cannot acquire lock.")` bajo daemons concurrentes | Sello Presentación ECST / peaje termodinámico (PR #393–#394); sonda `bus.topology` |
| D2 | `accept_pr::scan_presented_for_branch` solo escanea `.events/{pending,processed,dead-letter}`; en `redb` el Presented no deja fichero → `presented_found: false` / `orphan_merge` | PR #395: Presented `2a83c124-…` en store, accept-pr ciego |
| D3 | **Histórico (pre-CA4):** `system_vitality` escribía `.events/` con `write_json_atomic`; watcher en `redb` marcaba residual. **Estado:** emisión vía `write_fractal_event` / gate (`redb_ca4_*`, `eda-store-io-gate`). | Evidencia pre-lock `2a1c1a30-…` |
| D4 | `probe_bus_topology` trata cualquier fallo de `open` como rojo → `F-VITALIDAD-BUS-TOPOLOGY` por lock transitorio | Misma traza |

`tracker_pbi_meta::tracker_ref_in_store` ya lee `Family::Domain` del store; `scan_presented_for_branch` no.

`plan.md` sí: descomposición T1–T4 para Tekton (un commit/tarea). No hay blueprint de proceso nuevo.

## Corte

### C1 — Apertura con reintento (`sddia-event-store`)

1. Añadir `StoreError::Locked(String)` (o mensaje estable). Exportar en el mismo `error`/`lib` que el resto de variantes.
2. `EventStore::open`: bucle solo ante contención de lock (`redb` `DatabaseAlreadyOpen` / literal `Database already open`). Resto de errores → `Backend` (u otra variante existente) **sin** reintento.
3. Backoff exponencial corto + jitter. Presupuesto total por defecto **2000 ms**, override `SDDIA_EVENT_STORE_OPEN_BUDGET_MS` (entero positivo; 0 o ausente inválido → default).
4. Agotado el presupuesto → `Err(StoreError::Locked(...))`, no `Backend` con texto.

### C2 — Presented visible (`accept-pr`)

1. Extraer predicado compartido `presented_in_store(repo, branch) -> bool` desde la lectura de `tracker_pbi_meta` (`Family::Domain`, `event_type == PullRequest_Presented`, `payload.branch == branch`). `pub(crate)`. Sin duplicar el recorrido.
2. `scan_presented_for_branch`: si `writes_store(store_mode())` y `presented_in_store` → `true`; si no, conservar el escaneo legacy de `.events/`.
3. Tests `t_adu_ca7_*` legacy siguen verdes. Nuevo caso: Presented solo en store + modo `redb` → `true`.

### C3 — Vitalidad por el canal del bus (`system_vitality.rs`)

1. `emit_vitality_event`: `fractal::write_fractal_event(repo, &event, <telemetry_dir>)` (familia telemetry → `record_ecst`). En modo `redb` no crear `.events/telemetry/*.json`.
2. `emit_probe_fracture`: emitir `System_Fracture_Detected` vía escritor de dominio que llama `record_ecst(repo, "domain", …)` (mismo corte que `actions::write_pending_event` / `fractal` con familia domain). En modo `redb` no crear `.events/pending/*.json`.
3. Modo `filesystem`/`dual`: el helper existente ya escribe JSON cuando `writes_filesystem`; no reinventar dual-write.

### C4 — Sonda sin falso rojo

Tras C1, si `EventStore::open` devuelve `Locked`: `probe_bus_topology` → `ok: true`, `cause: "store ocupado"` (sin fractura). Cualquier otro error de open → rojo + `F-VITALIDAD-BUS-TOPOLOGY` como hoy.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| REDB-CA1 | Hilo A mantiene el store abierto; hilo B `open` obtiene `Ok` si A libera dentro del presupuesto |
| REDB-CA2 | Sin liberar: B obtiene `Locked` dentro del presupuesto; error no-lock sin reintento |
| REDB-CA3 | Modo `redb`, Presented solo en store → `scan_presented_for_branch` true; sin `merge_huérfano` por ceguera de FS; `t_adu_ca7_*` pasan |
| REDB-CA4 | Modo `redb`, sonda roja de vitalidad: cero ficheros nuevos en `.events/telemetry/` ni `.events/pending/`; filas en store |
| REDB-CA5 | Store bloqueado hasta `Locked`: no se emite `System_Fracture_Detected` por `bus.topology` |
| REDB-CA6 | Lab: `delivery-close-cycle` + `accept-pr` con daemons arriba → `presented_found: true` |

## Fuera de alcance

- Proceso único propietario del store / cambio de motor `redb`.
- Purga de ~20k ficheros legados en `.events/` (otro PBI).
- Retirar la ruta lógica `.events/pending/{id}.json` que el watcher pasa a `route-domain-event`.
- Cambiar el default de `SDDIA_EVENT_STORE`.
- Cierre automático de issues Linear por título de PR.
- Mutación de genoma (`SddIA/process/`, `norms/`, etc.): solo crates de implementación citados.

## Touchpoints (Tekton)

| Módulo | Acción |
|--------|--------|
| `SddIA/sddia-event-store/src/{lib,error}.rs` (+ tests) | C1 |
| `SddIA/engine/execute-process/src/engine/{tracker_pbi_meta,accept_pr}.rs` (+ tests) | C2 |
| `SddIA/engine/execute-process/src/engine/handlers/system_vitality.rs` (+ tests) | C3, C4 |
