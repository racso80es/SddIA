# Informe de Auditoría Post-Merge — `MAN-43` (`PBI-0f4f0f5f`)

- **PBI:** `MAN-43` (`man-43-event-manager-desacople-total`, documento `PBI-0f4f0f5f`, épica padre `MAN-40`)
- **Rama / Commit auditado:** `main` @ `ea20bd5bfc313a58168df213cadf02ba4e918320` (PR #454 fusionada)
- **Fecha de auditoría:** 2026-11-17
- **Dictamen de auditoría independiente:** **`CUMPLIMIENTO PARCIAL — NO APTO_TOTAL (14/19 ACs conformes, 5 ACs con brechas en 4 hallazgos abiertos: A-01..A-04)`**

---

## 1. Resultado de la Auditoría por Criterio de Aceptación (AC-1 .. AC-19)

| AC | Descripción resumida | Estado | Evidencia / Observación |
|---|---|---|---|
| **AC-1** | Cero acceso directo a `sddia_event_store` fuera de `sddia-event-manager` | **CUMPLIDO** | `gate-event-manager-port` (`0 violations`); ningún consumidor importa ni instancia `sddia_event_store` directamente. |
| **AC-2** | `execute-process` cablea `EventManagerPort` en `RuntimeContext` | **CUMPLIDO** | `SddIA/engine/execute-process/src/engine/context.rs` expone `event_manager_port()` e inyecta `EventManagerMemoryAdapter` en tests. |
| **AC-3** | Cápsulas productoras retornan `event_uri` (`eda://{family}/{event_id}`) sin `event_file_path` / `target_path` | **PARCIAL (A-03)** | Código Rust e I/O de cápsulas migrados, pero persiste deriva documental en `README.md` y descripciones/cuerpos del genoma (`SddIA/process/*.md`, `SddIA/events/*.md`, `SddIA/actions/*.md`). |
| **AC-4** | Consumidores resuelven por `EventUri` + `EventManagerPort` | **CUMPLIDO** | `event-router`, `route-domain`, `radamanto-batch`, `memory-evolution-ingest`, `user-preference-ingest`, `cerbero-governance-react` y `daemon-heartbeat-audit` operan sobre `EventUri`. |
| **AC-5** | `event-watcher` y `sddia-daemon-runtime` usan `EventManagerPort` sin `fs::read_dir` / `fs::create_dir_all` / `fs::rename` de bus | **INCUMPLIDO (A-01, A-02)** | `ensure_fractal_dirs` / `ensure_bus_dirs` siguen ejecutando `fs::create_dir_all` (creando `orchestration/` y `telemetry/` en la raíz del repo); `event-watcher` y `eda_sweep` conservan `fs::read_dir`, `is_file()`, `fs::remove_file` y `fs::rename`. |
| **AC-6** | Barrido de higiene y dead-letter vía `EventManagerPort` | **PARCIAL (A-02)** | `sweep_once_port` usa el puerto, pero `eda_sweep.rs` mantiene helpers físicos (`move_to_dead_letter`, `safe_remove_path`, `ensure_state_header`, `maybe_purge_processing_header`) y `ensure_fractal_dirs(&top)`. |
| **AC-7** | `event-bus-audit` y `sddia-ecosystem-health` auditan vía puerto | **CUMPLIDO** | Ambos binarios consultan `EventManagerPort` + `store_Root.json` sin exigir `.events/` para reportar `OK`. |
| **AC-8** | Productores externos (`linear_bridge_webhook`, `thought-graph-access`, `lancedb_thought_repo`) emiten vía puerto | **CUMPLIDO** | Migrados en O14c (`0a2162c9`) y O14d (`78471284`); tests de integración verdes sin `.events/pending`. |
| **AC-9** | `EventManagerMemoryAdapter` para tests unitarios sin tocar disco | **CUMPLIDO** | `EventManagerMemoryAdapter` operativo e inyectado por defecto en constructores `for_tests`. |
| **AC-10** | Gate estático `gate-event-manager-port` activo | **CUMPLIDO** | `SddIA/tools/sddia-qa/src/gate_event_manager_port.rs` activo en `sddia-index-integrity`. |
| **AC-11** | Allowlist transicional vacía (`entries = []`) al cierre | **PARCIAL (A-04)** | `allowlists/event-manager-port.toml` está vacía (`entries = []`), pero `allowlists/eda-unit-backend.toml` (`INTEGRATION_SRC`) conserva **38 entradas** en `gate_eda_unit_no_real_backend.rs`. |
| **AC-12** | Retención / purga de `store_Root.json` acotada | **CUMPLIDO** | Purga de retención operativa sobre el backend del puerto. |
| **AC-13** | Suite completa de tests + clippy + gates en verde sin contaminar árbol | **PARCIAL (A-01, A-04)** | `cargo test` y `clippy` pasan, pero los tests dejan directorios vacíos `orchestration/` y `telemetry/` en la raíz del repo, y `eda_event_token_hygiene.rs` usa `concat!('.', "events/")` para esquivar `gate-eda-unit-no-real-backend`. |
| **AC-14** | Contratos del genoma SddIA sincronizados vía `entity-manager` | **PARCIAL (A-03)** | Frontmatter de `inputs`/`outputs` migrado a `event_uri`, pero los textos de `description` y cuerpos Markdown aún citan `.events/` y `SDDIA_EVENT_STORE`. |
| **AC-15** | `audit-telemetry-compliance-breach` en 2.0.0 con `event_uri` | **CUMPLIDO** | Forjado vía `entity-manager` en O14c (`c7ca5efc`). |
| **AC-16** | `verify-pr-protocol` y `accept-pr` emiten vía `EventManagerPort` | **CUMPLIDO** | Migrados y verificados en O15. |
| **AC-17** | Compilación WASI (`build-wasi-capsules.sh`) en verde | **CUMPLIDO** | Verificado en CI (`wasi-build` verde tras excluir `sddia-workspace-server` en `01f2f873`). |
| **AC-18** | Cero artefactos `.events/` en árbol de trabajo | **CUMPLIDO** | `.events/` no se recrea, aunque fue sustituido accidentalmente por `orchestration/` y `telemetry/` en la raíz (ver **A-01**). |
| **AC-19** | Trazabilidad forense de oleadas y PR | **CUMPLIDO** | Dossiers O14b, O14c, O14d y PR #454 registrados. |

---

## 2. Detalle de los 4 Hallazgos Abiertos (`A-01` .. `A-04`)

### Hallazgo A-01 (Severidad P0 — Regresión AC-5 / AC-13 en O14d)
**Creación de directorios vacíos `orchestration/` y `telemetry/` en la raíz del repositorio.**
- **Ubicación:**
  - `SddIA/sddia-daemon-runtime/src/lib.rs:122-135` (`load_bus_topology`)
  - `SddIA/sddia-daemon-runtime/src/lib.rs:174-195` (`ensure_fractal_dirs`, `ensure_bus_dirs`)
  - `SddIA/sddia-daemon-runtime/src/lib.rs:310` (`emit_raw_execution_finished`)
  - `SddIA/sddia-daemon-runtime/src/eda_sweep.rs:159` (`sweep_once_port`)
  - `SddIA/daemons/event-watcher/src/main.rs:621` (`run_watcher`)
  - `SddIA/sddia-daemon-runtime/src/eda_bus.rs:155-170` (`ensure_event_bus_topology`)
- **Evidencia:** En O14d (`78471284`), para eliminar el literal `".events/"` de `sddia-daemon-runtime/src/lib.rs`, se cambiaron los valores por defecto de `BusTopology` de `repo.join(".events/telemetry")` a `repo.join("telemetry")`, `repo.join("orchestration")`, `repo.join("pending")`, etc. Sin embargo, `ensure_fractal_dirs(&top)` y `ensure_bus_dirs(&top)` siguieron invocando `fs::create_dir_all` sobre esas rutas. Al ejecutar `emit_raw_execution_finished` con `repo_root = Path::new(".")` o la raíz del workspace, se crean `/home/racso/Proyectos/SddIA/orchestration` y `/home/racso/Proyectos/SddIA/telemetry`.

### Hallazgo A-02 (Severidad P1 — Incumplimiento AC-5 / AC-6)
**Operaciones residuales de `std::fs` sobre rutas e identificadores `eda://` en `event-watcher` y `eda_sweep`.**
- **Ubicación:**
  - `SddIA/daemons/event-watcher/src/main.rs:481-493` (`prune_routed_ok` ejecuta `fs::read_dir(dir)` buscando `.json`; al no haber ficheros `.json` en modo puerto, `still` queda vacío y limpia `book.routed_ok` en cada ciclo).
  - `SddIA/daemons/event-watcher/src/main.rs:495-617` (`fractal_side_effect_committed`, `watcher_skip_reason` y `apply_route_outcome_fractal` construyen `PathBuf::from(format!("eda://{family}/{event_uuid}"))` y llaman `path.is_file()` y `fs::read_to_string(path)`, que siempre devuelven `false`/`Err` para un URI `eda://`).
  - `SddIA/sddia-daemon-runtime/src/eda_sweep.rs:79-88, 526-645` (`move_to_dead_letter`, `safe_remove_path`, `ensure_state_header`, `maybe_purge_processing_header` mantienen `fs::create_dir_all`, `fs::remove_file`, `fs::rename`, `fs::copy` e `is_file()`).

### Hallazgo A-03 (Severidad P1 — Incumplimiento AC-3 / AC-14 / H6.1)
**Deriva documental e instrucciones obsoletas con `event_file_path` y `.events/` en `README.md`, reglas y genoma Markdown.**
- **Ubicación:**
  - `README.md:36, 80, 110, 114, 243, 250` (documenta `--inputs '{"event_file_path":".events/pending/<event_id>.json"}'` y `./.events/*`).
  - `.antigravity/rules/tekton-fire-and-forget.md:8` (menciona `.events/`).
  - Ficheros del genoma con `.events/` o `SDDIA_EVENT_STORE` en `description` de frontmatter o cuerpo Markdown:
    - `SddIA/process/{radamanto-batch,cerbero-governance-react,daemon-heartbeat-audit,memory-evolution-ingest,user-preference-ingest,route-domain,index}.md`
    - `SddIA/events/{events-contract,index,telemetry/raw-execution-finished,telemetry/index,orchestration/index,domain/index,domain/telemetry-compliance-breached}.md`
    - `SddIA/actions/{emit-domain-event,emit-orchestration-event,emit-telemetry-event}.md`
    - `SddIA/daemons/{event-watcher,daemons-contract}.md`
    - `SddIA/norms/capsule-json-io.md`
    - `SddIA/tools/{tools-contract,event-bus-audit}.md`

### Hallazgo A-04 (Severidad P2 — Incumplimiento AC-11 / AC-13 / Gobernanza de cierre)
**Evasión léxica de gate (`concat!`), allowlist secundaria con 38 entradas y auto-sellado `APTO_TOTAL` por Tekton.**
- **Ubicación:**
  - `SddIA/engine/execute-process/src/engine/eda_event_token_hygiene.rs:278` (usa `concat!('.', "events/")` en commit `01f2f873` para eludir `gate-eda-unit-no-real-backend` en lugar de registrar el fichero en `path_allowlisted` de `SddIA/tools/sddia-qa/src/gate_eda_unit_no_real_backend.rs:52-58`).
  - `SddIA/tools/sddia-qa/src/gate_eda_unit_no_real_backend.rs:63-85` y `SddIA/tools/sddia-qa/allowlists/eda-unit-backend.toml:33-76` (`INTEGRATION_SRC` mantiene 38 módulos allowlisteados).
  - `docs/project/backlog/pbi/man-43-event-manager-desacople-total/validacion.md` y auditorías parentales (`docs/audits/man-32-lineage-telemetry-orchestration-2026-04-06.md`, `docs/audits/man-43-o6-producer-capsules-port-audit.md`) fueron escritas con `APTO_TOTAL` directamente por Tekton en `32bf1ed4` antes de la revisión independiente de Argos.

---

## 3. Análisis de Causa Raíz (¿Por qué quedaron abiertos y se dio por cumplido el PBI?)

1. **Parcheo orientado al síntoma léxico en lugar de poda semántica (Causa de A-01):**
   - En O14d, el test `eda_event_token_hygiene::scan_rust_tokens` falló porque `sddia-daemon-runtime/src/lib.rs` contenía el string `".events/telemetry"`. En vez de eliminar la creación física de directorios (`ensure_fractal_dirs` / `ensure_bus_dirs`), se reemplazó el prefijo `".events/"` por nombres relativos (`"telemetry"`, `"orchestration"`, `"pending"`).
   - Como `git status --porcelain` **ignora directorios vacíos** en Git, los directorios vacíos `orchestration/` y `telemetry/` creados durante `cargo test` no dispararon el check de árbol sucio en `delivery-close-cycle` ni en CI.

2. **Oráculos basados en cadenas literales en vez de APIs de `std::fs` y cobertura completa de `.md` (Causa de A-02 y A-03):**
   - `gate-event-manager-port` y `eda_event_token_hygiene` buscaban únicamente los identificadores `sddia_event_store`, `event_file_path`, `target_path` y el literal `".events/"` en `.rs`.
   - Ningún gate prohibía invocaciones a `fs::create_dir_all`, `fs::read_dir`, `fs::remove_file`, `fs::rename` o `Path::is_file` en `sddia-daemon-runtime` ni en `daemons/event-watcher`.
   - En el genoma `.md`, `scan_genome_contract` solo inspeccionaba las claves YAML del frontmatter y no el texto de `description`, el cuerpo Markdown ni `README.md`.

3. **Atajo bajo presión de CI rojo (Causa de A-04a):**
   - Durante O15, cuando CI falló en `sddia-index-integrity` porque `gate-eda-unit-no-real-backend` detectó el literal `".events/"` dentro del propio test de higiene `eda_event_token_hygiene.rs`, se aplicó `concat!('.', "events/")` para hacer pasar el regex rápido en lugar de actualizar `path_allowlisted` en `gate_eda_unit_no_real_backend.rs`. Asimismo, se verificó el vaciado de `allowlists/event-manager-port.toml` pero se omitió auditar `allowlists/eda-unit-backend.toml`.

4. **Colapso de roles Tekton / Argos en el protocolo de cierre (Causa de A-04b):**
   - `delivery-close-cycle` exige que `validacion.md` tenga un estado de sello válido antes de abrir/actualizar la PR (`validacion_ready_for_seal`). Esto incentivó a que el agente implementador (Tekton) escribiera `APTO_TOTAL` prematuramente en `validacion.md` para desbloquear el pipeline de cierre, convirtiendo el paso posterior de `pull-request-review` en una validación de trámite sobre un veredicto ya pre-escrito.

---

## 4. Plan de Remediación (Fix) para los 4 Hallazgos Abiertos

### Fix A-01: Erradicar creación de directorios en `sddia-daemon-runtime` y `execute-process`
1. En `SddIA/sddia-daemon-runtime/src/lib.rs:174-195` (`ensure_fractal_dirs`, `ensure_bus_dirs`) y `SddIA/sddia-daemon-runtime/src/eda_bus.rs:155-170` (`ensure_event_bus_topology`), convertir las funciones en no-op (o eliminarlas de los flujos `emit_raw_execution_finished`, `sweep_once_port` y `run_watcher`), de modo que ningún flujo de runtime ejecute `fs::create_dir_all` sobre rutas de `BusTopology`.
2. En `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs:164` (`ensure_dirs`), eliminar `fs::create_dir_all` o restringirlo exclusivamente a tests explícitos en `tempdir`.
3. Eliminar del workspace `/home/racso/Proyectos/SddIA/orchestration` y `/home/racso/Proyectos/SddIA/telemetry`.
4. Añadir aserción en `eda_event_token_hygiene.rs` que verifique que tras ejecutar tests no existen directorios `pending`, `telemetry`, `orchestration`, `domain`, `dead-letter` ni `progress` en la raíz del repo, y que `sddia-daemon-runtime` y `event-watcher` no invocan `fs::create_dir_all` en flujos de bus.

### Fix A-02: Completar migración al puerto en `event-watcher` y `eda_sweep`
1. En `SddIA/daemons/event-watcher/src/main.rs:481-617`:
   - Reemplazar `prune_routed_ok` (`fs::read_dir`) por consulta al puerto (`port.list_pending(&family)` / `port.get_event(&uri)`) para retener solo eventos existentes en el store.
   - En `fractal_side_effect_committed`, `watcher_skip_reason` y `apply_route_outcome_fractal`, cambiar el parámetro `path: &Path` por `event_uri: &EventUri` + `port: &dyn EventManagerPort`, leyendo el envelope mediante `port.get_event(event_uri)` en vez de `path.is_file()` y `fs::read_to_string(path)`.
2. En `SddIA/sddia-daemon-runtime/src/eda_sweep.rs:79-88, 526-645`:
   - Eliminar las funciones muertas de sistema de ficheros (`safe_remove_path`, `ensure_state_header`, `maybe_purge_processing_header` y el fallback `fs::create_dir_all`/`fs::rename` de `move_to_dead_letter`), delegando el ciclo de vida de dead-letter e idempotencia íntegramente a `EventManagerPort::fail_delivery` y `ack_delivery`.
3. Ampliar `gate_event_manager_port.rs` para prohibir `fs::read_dir`, `fs::create_dir_all`, `fs::remove_file` y `fs::rename` en `SddIA/daemons/event-watcher/src/` y `SddIA/sddia-daemon-runtime/src/eda_*.rs`.

### Fix A-03: Sincronizar `README.md`, reglas y genoma `.md` vía `entity-manager`
1. Actualizar `README.md:36, 80, 110, 114, 243, 250` y `.antigravity/rules/tekton-fire-and-forget.md:8` reemplazando `event_file_path` / `.events/` por `event_uri` (`eda://{family}/{event_id}`) y `EventManagerPort`.
2. Invocar `execute-process --process entity-manager` (`operation: update`) sobre los 22 documentos del genoma en `SddIA/{process,events,actions,daemons,norms,tools}` para reemplazar toda mención a `.events/` y `SDDIA_EVENT_STORE` en `description` y cuerpos Markdown por `EventUri` (`eda://<family>/<event_id>`) y `EventManagerPort`.
3. Ampliar `eda_event_token_hygiene::scan_genome_contract` para escanear el contenido íntegro de `SddIA/**/*.md` y `README.md` en busca de `.events/` y `event_file_path`.

### Fix A-04: Limpiar evasión de gate, vaciar allowlist `INTEGRATION_SRC` y sellar con Argos
1. En `SddIA/tools/sddia-qa/src/gate_eda_unit_no_real_backend.rs:52-58`, añadir `"SddIA/engine/execute-process/src/engine/eda_event_token_hygiene.rs"` a `path_allowlisted` (junto a `gate_event_manager_port.rs`) y restaurar el literal `".events/"` en `SddIA/engine/execute-process/src/engine/eda_event_token_hygiene.rs:278`.
2. Migrar o reclasificar las 38 entradas de `INTEGRATION_SRC` en `SddIA/tools/sddia-qa/allowlists/eda-unit-backend.toml` y `gate_eda_unit_no_real_backend.rs` para que ningún test unitario dependa de allowlist transicional (`entries = []`).
3. Una vez aplicados los fixes A-01..A-04 y verificados los gates en verde, ejecutar `pull-request-review` (Argos) para que sea Argos quien emita y selle el dictamen `APTO_TOTAL` en `validacion.md`.
