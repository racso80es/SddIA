---
audit_id: AUDIT-EDA-REDB-20261026
date: "2026-10-26"
scope: "Gestión de eventos SddIA: events.redb vs .events/ y fricciones de concurrencia"
related_issues:
  - AUT-7
  - AUT-16
related_prs:
  - "https://github.com/racso80es/SddIA/pull/393"
  - "https://github.com/racso80es/SddIA/pull/394"
status: FINDINGS_OPEN
---

# Auditoría — Situación del Bus de Eventos (`events.redb` vs `.events/`)

## 1. Resumen Ejecutivo

| Pregunta de auditoría | Veredicto | Síntesis |
| :--- | :--- | :--- |
| **¿Se usa `events.redb`?** | **SÍ (por defecto)** | `SDDIA_EVENT_STORE` sin definir resuelve a `StoreMode::Redb`. La topología SSOT (`cumulo.paths.json` v1.11.1) declara `.SddIA/runtime/events.redb` como bus principal y fractal (~21,8 MB activos). |
| **¿Se comporta adecuadamente?** | **DEGRADADO BAJO CONCURRENCIA** | `redb` aplica bloqueo exclusivo a nivel de descriptor de fichero al abrir `Database::create`. Sin cola, *retry/backoff* ni proceso único propietario, los procesos concurrentes colisionan con `Database already open. Cannot acquire lock.`. |
| **¿Sigue usándose `.events/`?** | **SÍ (asimetría / split-brain)** | Aunque `writes_filesystem(StoreMode::Redb)` es `false` en el crate `sddia-event-store`, persisten rutas en `execute-process` que **escriben o leen directamente `.events/`**, provocando falsos `merge_huérfano`, eventos residuales y pérdida de correlación. |

---

## 2. Arquitectura Nominal (`sddia-event-store` y `cumulo.paths.json`)

1. **SSOT Topológico (`SddIA/core/cumulo.paths.json`):**
   - `event_bus`: `".SddIA/runtime/events.redb"`
   - `eda_bus.store` y `eda_fractal.store`: `".SddIA/runtime/events.redb"`
   - Tablas: `events`, `family_index`, `subscriber_delivery`, `dead_letter_index`, `progress_traces`, `dlt_audit_seals`.
2. **Corte de modo (`SddIA/sddia-event-store/src/gate.rs`):**
   - `mode_from_name(None | "") -> StoreMode::Redb`.
   - En `StoreMode::Redb`: `writes_store = true`, `writes_filesystem = false`.
3. **Intención de diseño en `event-watcher`:**
   - En modo `Redb`, reclama entregas pendientes desde `events.redb` (`claim_store_pending`).
   - Si encuentra un `.json` nuevo en `.events/pending/` estando en modo `Redb`, **no lo enruta**: lo considera anomalía y emite `F-EDA-RESIDUAL-FILESYSTEM` (`SddIA/daemons/event-watcher/src/main.rs:604-634`).

---

## 3. Evidencia Empírica Recolectada en el Workspace (2026-10-26)

### 3.1 Estado físico en disco y procesos activos
- `.SddIA/runtime/events.redb`: **21.807.104 bytes** (modificado continuamente por daemons y CLI).
- `.events/`: **20.091 ficheros** acumulados (`dead-letter`, `telemetry`, `processed`, `pending`, `domain`, `orchestration`).
- Daemons concurrentes observados en ejecución simultánea contra el mismo repo:
  - `event-watcher` (con workers asíncronos hasta `MAX_IN_FLIGHT_ROUTES`)
  - `kalma2-bridge`
  - `email-watcher`, `telegram-watcher`, `github-bridge-watcher`, `iota-publish-relay`
  - Subprocesos `execute-process --process route-domain-event` y sondas periódicas (`system-vitality-probe`, `daemon-heartbeat-audit`).

### 3.2 Fricción 1 — Fallo de `delivery-close-cycle` (PR #393 y #394) y Peaje Termodinámico
Durante el cierre de `feat/llm-interaction-audit` (PR #393) y la rama documental (PR #394), `delivery-close-cycle` falló en:
- **Fase «Sello Presentación ECST»** (`emit-pr-presented-event` → `write_pending_event` → `record_ecst`):
  ```text
  "phase_name": "Sello Presentación ECST",
  "status": "failed",
  "error": "Database already open. Cannot acquire lock."
  ```
- **Canal de telemetría termodinámica** (`thermodynamic.rs` → `write_fractal_event` → `record_ecst`):
  ```text
  [THERMODYNAMIC-TOLL-EMERGENCY] process=delivery-close-cycle channel=telemetry: Database already open. Cannot acquire lock.
  ```

### 3.3 Fricción 2 — Falsa fractura de vitalidad (`F-VITALIDAD-BUS-TOPOLOGY`)
Evidencia en `.events/pending/2a1c1a30-2851-40f8-a49e-96803c81a5a3.json` (`2026-10-26T14:47:26Z`):
```json
{
  "event_type": "System_Fracture_Detected",
  "payload": {
    "attempted_action": "system-vitality-probe",
    "error_trace": "sonda bus.topology en rojo: events.redb no abre: Database already open. Cannot acquire lock.",
    "friction_id": "F-VITALIDAD-BUS-TOPOLOGY"
  }
}
```
La sonda `probe_bus_topology` (`system_vitality.rs:131`) hace `EventStore::open(&db)` para verificar salud. Si en ese instante `event-watcher` o `route-domain-event` tiene abierto `events.redb`, la sonda interpreta el lock como **caída de topología** y emite una fractura de sistema.

---

## 4. Hallazgos Arquitectónicos: Por qué sigue usándose `.events/` y dónde se rompe

### Hallazgo A — Contención de lock exclusivo en `redb` sin backoff (`sddia-event-store`)
En `SddIA/sddia-event-store/src/lib.rs:41-44`:
```rust
pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
    let db = Database::create(path).map_err(backend)?;
    Ok(Self { db })
}
```
`redb::Database::create` adquiere un lock exclusivo inmediato (`flock` no bloqueante) sobre `.SddIA/runtime/events.redb` durante toda la vida del `EventStore`. En una arquitectura con **6 daemons + N subprocesos `execute-process`**, abrir sin un bucle de reintento con *jitter* (p. ej. 20–50 intentos de 5–25 ms) garantiza fallos espurios bajo carga normal.

### Hallazgo B — Asimetría Lectura/Escritura en `PullRequest_Presented` (`accept_pr.rs`)
1. **Al emitir (`actions.rs:36-49`):** En `StoreMode::Redb`, `write_pending_event` escribe en `events.redb` y **hace `return Ok(...)` antes de escribir en `.events/pending/`** (`!writes_filesystem(mode)`), aunque devuelve en el JSON un `target_path` sintético `".events/pending/{id}.json"`.
2. **Al verificar en `accept-pr` (`accept_pr.rs:35-68`):** `scan_presented_for_branch` **solo escanea directorios del filesystem** (`.events/pending`, `.events/processed`, `.events/dead-letter`) y **nunca consulta `events.redb`**:
   - Consecuencia 1: Si `emit-pr-presented-event` falla por lock de `redb`, no queda evento en ningún sitio → `orphan_merge: true`.
   - Consecuencia 2 (**más grave**): Incluso cuando `emit-pr-presented-event` **tiene éxito** en `StoreMode::Redb`, como no escribe fichero en `.events/`, `scan_presented_for_branch` devolverá `false` y **marcará `merge_huérfano` en todos los merges**, a diferencia de `tracker_pbi_meta::tracker_ref_from_presented_for_branch` (`tracker_pbi_meta.rs:78-129`), que sí consulta `.events/` **y** `tracker_ref_in_store` (`events.redb`).

### Hallazgo C — Escritores que hacen bypass de `events.redb` y escriben directo en `.events/`
En `SddIA/engine/execute-process/src/engine/handlers/system_vitality.rs`:
- **`emit_vitality_event` (líneas 324-345):** escribe `System_Vitality_Probed` directamente con `write_json_atomic` en `.events/telemetry/{id}.json` (evidencia: `.events/telemetry/e9c85731-….json` creado hoy a las 16:18), sin pasar por `write_fractal_event` ni `record_ecst`.
- **`emit_probe_fracture` (líneas 347-374):** escribe `System_Fracture_Detected` directamente en `.events/pending/{id}.json` (evidencia: `2a1c1a30-….json` hoy a las 15:47).
  - **Paradoja destructiva:** Como el sistema está en `StoreMode::Redb`, cuando `event-watcher` ve ese archivo en `.events/pending/`, su bloque de las líneas 604-634 **se niega a enrutarlo** y lo marca como `F-EDA-RESIDUAL-FILESYSTEM` («residual con store redb; no reinyectado»). Es decir: la fractura queda atrapada en `.events/pending/` y nunca se procesa.

### Hallazgo D — Acoplamiento de interfaz por rutas ficticias `.events/pending/{id}.json`
Aunque el evento viva solo en `events.redb`, `event-watcher` (`main.rs:252`) invoca:
```text
execute-process --process route-domain-event --inputs {"event_file_path":".events/pending/<id>.json"}
```
y `route-domain-event` extrae el `event_id` del nombre de fichero y llama a `load_dispatched_event` (`gate.rs:165`), que en modo `Redb` ignora la ruta y lee la tabla `events`. Funciona, pero mantiene la ilusión de que `.events/pending/` sigue siendo el transporte.

---

## 5. Matriz de Uso Real: `events.redb` vs `.events/`

| Componente / Flujo | ¿Usa `events.redb`? | ¿Usa `.events/` en modo `Redb`? | Estado |
| :--- | :--- | :--- | :--- |
| `actions::write_pending_event` (`Work_Initiated`, `PullRequest_Presented`, `PullRequest_Merged`) | Sí (`record_ecst`) | No (solo devuelve ruta lógica) | Falla si hay lock concurrente en `redb` |
| `fractal::write_fractal_event` (peaje termodinámico, orquestación) | Sí (`record_ecst`) | No | Falla con `[THERMODYNAMIC-TOLL-EMERGENCY]` si hay lock |
| `event-watcher` (`claim_store_pending`) | Sí (`list_pending_deliveries`) | Rechaza `.events/pending/*.json` como fractura residual | Correcto en diseño, sufre lock si coincide con CLI |
| `tracker_pbi_meta::tracker_ref_from_presented_for_branch` | Sí (`tracker_ref_in_store`) | Sí (fallback previo en `.events/`) | Coherente (híbrido) |
| `accept_pr::scan_presented_for_branch` (ADU-CA7) | **NO** | **SÍ (exclusivamente)** | **BUG:** en modo `Redb` puro no ve `PullRequest_Presented` del store |
| `system_vitality::emit_vitality_event` | **NO** | **SÍ (escribe `.events/telemetry/*.json`)** | **BUG:** bypass de `sddia-event-store` |
| `system_vitality::emit_probe_fracture` | **NO** | **SÍ (escribe `.events/pending/*.json`)** | **BUG:** genera residual bloqueado por `event-watcher` en modo `Redb` |
| `system_vitality::probe_bus_topology` | Abre `EventStore::open` | No | **BUG:** un lock transitorio se reporta como fractura `F-VITALIDAD-BUS-TOPOLOGY` |

---

## 6. Recomendaciones Correctivas (Kaizen / PBIs sugeridos)

1. **Backoff + Jitter en `sddia_event_store::EventStore::open` (Prioridad P0):**
   - Envolver `Database::create(path)` en un bucle de reintentos ante error de lock (`Database already open` / `StorageError::DatabaseAlreadyOpen`), p. ej. backoff exponencial corto hasta ~1,5–2 s antes de devolver `StoreError::Backend`. Esto elimina de raíz los fallos en `delivery-close-cycle`, `thermodynamic` y `system-vitality-probe`.
2. **Actualizar `accept_pr::scan_presented_for_branch` para consultar `events.redb` (Prioridad P1):**
   - Igualar su lógica a `tracker_pbi_meta::tracker_ref_from_presented_for_branch`: si `writes_store(store_mode())`, buscar `PullRequest_Presented` con `payload.branch == branch` en `Family::Domain` de `events.redb` además de los directorios legacy `.events/`.
3. **Unificar `system_vitality.rs` con `sddia-event-store` (Prioridad P1):**
   - Sustituir la escritura manual en `.events/telemetry/` por `fractal::write_fractal_event`.
   - Sustituir la escritura manual en `.events/pending/` de `emit_probe_fracture` por el canal canónico que invoca `record_ecst(&repo, "domain", &event)`.
   - En `probe_bus_topology`, no emitir fractura si el error de `EventStore::open` es únicamente contención transitoria de lock (especialmente tras aplicar el punto 1).
4. **Higiene del directorio `.events/` legacy (Prioridad P2):**
   - Archivar o purgar los ~20.000 ficheros históricos previos al corte `Redb` (octubre 2026) y limpiar `.events/pending/2a1c1a30-….json` y los `e2e-*.json` residuales para evitar ruido en auditorías y en `event-watcher`.
