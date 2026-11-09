---















feature_name: man-40-eda-consumers-logical-seal
created: "2026-11-09"
updated: "2026-11-09"
process: feature
purpose: Estabilización de requisitos — consumidores EDA sellan materialización con token lógico (PublishSeal/event_id); cero path Redb ni canal FS `.events/*` como SSOT fuera del host
branch_name: feat/man-40-eda-consumers-logical-seal
persist_ref: docs/features/man-40-eda-consumers-logical-seal
pbi_ref: MAN-40
parent_ref: MAN-36
historia_ref: MAN-36
document_id: PBI-1b004e99
phase: mayeuta-stabilization
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version: "1.2.0"
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-40.jsonl"
forge_key: "bd5f4db27f549584d61e515fe85571b8fb2d5f3279161d9aa982ff2851471498"
execution_id: "be6cdeb9-09cb-4eac-a0b6-8100846f5507"
---
# Clarificación — man-40-eda-consumers-logical-seal

Transcript Mayeuta (2026-11-09). Fase **Estabilización de Requisitos** del proceso `feature`.
Canal tracker: **MAN-40** (historia/padre **MAN-36**). Documento semilla **PBI-1b004e99**. Provider Linear: sin copia nueva bajo `docs/todos/`.

## D0 — Semilla y canal tracker

| Hecho | Laudo |
|-------|-------|
| Tracker `pbi_ref` | **MAN-40** (provider Linear). Canal leído: `.SddIA/state/tracker-outbound/MAN-40.jsonl` (`forge-pbi` → `refine-item` → `tracker-stamp` → `lifecycle-phase-action` / `feature`, 2026-11-09; última huella ~20:48Z). |
| Padre / historia | **MAN-36** (`man-36-event-manager-instance-config`) — bóveda soberana + composition root. Este ítem **no** reabre configuración ni vars `SDDIA_EVENT_MANAGER_*`. |
| Antecedente cutover | **MAN-35** — `EventManagerPort` 1.2; Redb productivo vía host. |
| Documento | **PBI-1b004e99** — forja sellada `.SddIA/state/forge-pbi/bd5f4db27f549584d61e515fe85571b8fb2d5f3279161d9aa982ff2851471498.json` (`event_sealed: true`, `tracker_ref: MAN-40`). |
| Semilla orquestación | Proceso declarado `bug-fix` en frontmatter del PBI; el ciclo runtime actual es **`feature`** (`execution_id` d5f40…). **L-PROCESS**: documentar como feature sin reabrir el defecto. |
| Cuerpo Linear en disco | **No materializado** en jsonl (solo operaciones). **No bloquea**: semilla de orquestación + AC + inventario estabilizado bastan (**L-TRACKER**). |
| Evidencia viva (esta pasada) | `SddIA/sddia-daemon-runtime/src/github_bridge.rs` — `materialize_to_bus` publica con `record_ecst_on_port` y **devuelve** `sddia_event_manager_host::resolved_redb_path`; el log `Materializado` imprime el path físico. Inventario de sello físico + canal FS/nombres cableados confirmado en semilla refinada (2026-11-09 / mayeuta). |
| Contraste sano | `linear_bridge::materialize_pending` / `record_ecst_productive` (token `.events/pending/{event_id}.json`); `eda_bus::write_fractal_event`, `fractal::write_fractal_event`, `iter_bus_event_files` (no-test), `ecst_input_loader` (rechaza JSON físico) — **no** son deuda de este ciclo. |
| Fuera del defecto | Dead-letter local `github_bridge::write_fallback_dead_letter` (JSON de fallo del bridge, no backend del puerto). |
| Modo | `branch_pr` / `feature` — tajos T0…T6 verificables. |
| `execution_id` | **d5f40be9-1866-4813-a5b2-28a26faed56c** |

## D1 — Matriz PBI × alcance estabilizado

| Afirmación (semilla / PBI) | Laudo |
|---------------------------|-------|
| Residuo **CA-4** de MAN-36: consumidores no deben conocer path Redb ni layout histórico como sello | **L-CA4-RESIDUE** — este ciclo cierra el *sello* y el *canal FS/nombres*; no la bóveda. |
| Sello de materialización = `PublishSeal` o `event_id` | **L-LOGICAL-SEAL** |
| Solo `sddia-event-manager-host` posee `resolved_redb_path` / `is_resolved_redb_path` / `DEFAULT_REDB_REL` | **L-HOST-ONLY** |
| Cero fallback `.SddIA/runtime/events.redb` fuera del host | **L-NO-LAYOUT-FALLBACK** |
| Detección de path de store solo vía API del host (sin `file_name == events.redb`) | **L-NO-NAME-DETECT** |
| Gate veta `resolved_redb_path`, `runtime_db_path`, `DEFAULT_REDB_REL` fuera de host + allowlist tests/legado | **L-GATE** |
| `event-watcher` opera pending/telemetría/dominio/orquestación por `EventManagerPort`; cierre sin `is_file`/`read_dir` de testigos como SSOT | **L-WATCHER-PORT** |
| `terminal_witness_exists` no cae a fichero si el puerto no marca terminal | **L-TERMINAL-PORT** |
| Hooks/installer/limpieza no leen `SDDIA_EVENT_STORE` ni asumen `events.redb` único | **L-SCRIPTS** |
| Lab `forge-doc-drift-actions.py` deja contrato filesystem/dual y pending físico | **L-LAB-DRIFT** |
| `productive_store_present` (purger / kalma2) no ramifica modo por existencia de fichero | **L-NO-FILE-MODE** |

## D2 — Laudos de estabilización (cerrados)

| ID | Decisión |
|----|----------|
| **L-DOMAIN** | Frontera productiva = **`EventManagerPort`** / capacidad **`eda:event-manager`** v1.2. El plano físico Redb/FS del adapter **no** es API de consumidores ni sello de éxito. |
| **L-CA4-RESIDUE** | MAN-40 = cierre del residuo CA-4 de MAN-36 **ampliado** al segundo canal (FS/nombres cableados). No reabre L-VAULT / L-VARS / L-PRECEDENCE de MAN-36. |
| **L-LOGICAL-SEAL** | Tras publicar, el resultado/log/cabecera expone **`PublishSeal`** (token lógico `.events/{family}/{id}.json`) o **`event_id`**. Prohibido devolver/registrar `resolved_redb_path` / `PathBuf` al `.redb` como sello. |
| **L-HOST-ONLY** | Único dueño de `resolved_redb_path`, `is_resolved_redb_path`, `DEFAULT_REDB_REL` y defaults de layout: **`sddia-event-manager-host`**. Consumidores delegan vía puerto/`with_productive_port`; no reexportan el físico como API pública estable. |
| **L-NO-LAYOUT-FALLBACK** | Prohibido `repo.join(".SddIA/runtime/events.redb")` / `DEFAULT_REDB_REL` en gate `sddia-event-store`, `event-bus-audit`, u otros consumidores. |
| **L-NO-NAME-DETECT** | Prohibido `file_name == "events.redb"` (p. ej. `is_runtime_db_event_path`) como detección de store; solo API del host. |
| **L-NO-FILE-MODE** | Prohibido ramificar modo productivo por existencia de fichero (`productive_store_present` y equivalentes). El modo lo decide el host/bóveda, no el FS. |
| **L-SEAL-INVENTORY** | Inventario cerrado de sello físico a corregir: `github_bridge::materialize_to_bus`; `eda_bus_topology::write_processing_witness`; `route_domain_core` (`processing_header`); `eda_port::runtime_db_path` / `path_is_resolved_redb` / `store_present` / `port_arc`; `is_runtime_db_event_path`; `sddia-event-store` gate; `event-bus-audit::audit_state_from_store`; purger + `kalma2-bridge/eda_productive.rs`. |
| **L-FS-INVENTORY** | Inventario cerrado de canal FS/nombres: `event-watcher` (`read_dir` / `is_file` / claim fractal ausente / log `"redb"`); `terminal_witness_exists` fallback `is_file`; `hook_common.sh`; `clean-project-ephemeral.sh`; `sddia-installer.sh`; lab `docs/features/eda-store-io-gate/_lab/forge-doc-drift-actions.py`. |
| **L-WATCHER-PORT** | Pending, telemetría, dominio y orquestación se reclaman/orquestan por puerto. `claim_store_fractal` debe tener equivalente por puerto. Cierre de tick **sin** `pending_path.is_file()` ni `read_dir` de testigos/dead-letter como SSOT. |
| **L-TERMINAL-PORT** | Si el ECST vía puerto no marca estado terminal, **no** hay fallback a `witness_exists`/`is_file` en build productivo (tests pueden aislarse con `cfg(test)` si hace falta). |
| **L-SCRIPTS** | Hooks, installer y scripts de limpieza dejan de gobernar el store con `SDDIA_EVENT_STORE` / nombre fijo `events.redb`. Alineación documental/operativa con `SDDIA_EVENT_MANAGER_*` (sin reabrir resolución del host). |
| **L-LAB-DRIFT** | El lab forge-doc-drift actualiza contrato al puerto/sello lógico; deja de asumir pending físico dual/filesystem. |
| **L-GATE** | Gate arquitectónico veta símbolos/paths cableados (`resolved_redb_path`, `runtime_db_path`, `DEFAULT_REDB_REL`, open/layout histórico) **fuera** del host y allowlist explícita de tests/legado. |
| **L-HEALTHY** | No tocar caminos ya sanos: linear_bridge con token lógico; writers fractal/eda_bus; `ecst_input_loader` estricto; dead-letter local del bridge. |
| **L-NON-GOALS** | No reabrir bóveda/precedencia MAN-36; no cambiar semántica ECST ni contrato EventManagerPort 1.2; no rehabilitar `SDDIA_EVENT_STORE` productivo; no migrar dead-letter local del bridge a backend del puerto; pirámide tests EDA = MAN-37 (fuera salvo lo que el gate/tests de este residuo exijan). |
| **L-PROCESS** | Ciclo documental/runtime = `feature` / `branch_pr`; el origen PBI `bug-fix` queda como metadato de forja, no como proceso de fase. |
| **L-GIT** | Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`. |
| **L-GENOME** | Mutaciones de entidades del genoma **solo** vía `entity-manager`. |
| **L-TRACKER** | Provider Linear: sin ficheros nuevos de backlog en `docs/todos/`; KM/TODOs solo vía `agent:cumulo` o evento `Kaizen_Alert_Required`. |

## D3 — Tajos canónicos (orden de forja)

| Tajo | Contenido estabilizado |
|------|------------------------|
| **T0** | Contrato de sello lógico: `PublishSeal` / `event_id` como único resultado de materialización; retirar returns/logs/cabeceras con PathBuf Redb en puente GitHub y caminos análogos. |
| **T1** | Desacoplar `eda_port` / `eda_bus_topology` / `route_domain_core`: sin `runtime_db_path` público como sello; `write_processing_witness` y `processing_header` lógicos; `terminal_witness_exists` solo puerto. |
| **T2** | Eliminar fallbacks de layout y detección por nombre: gate `sddia-event-store`, `event-bus-audit`, `is_runtime_db_event_path`, `productive_store_present` (purger/kalma2). |
| **T3** | `event-watcher`: claim/orquestación por `EventManagerPort` (pending + fractal/familias); cero SSOT `read_dir`/`is_file` de colas/testigos; telemetría de claim sin etiqueta store `"redb"` como SSOT. |
| **T4** | Hooks / installer / limpieza / lab forge-doc-drift: retirar `SDDIA_EVENT_STORE` y asunción `events.redb` único; alinear a gobernanza host. |
| **T5** | Gate arquitectónico + allowlist tests/legado; vetos a símbolos/paths fuera del host. |
| **T6** | Tests de no-regresión (sello lógico, sin path en logs/returns, watcher por puerto, terminal sin FS) + `validacion` APTO. |

## D4 — Preguntas abiertas → cerradas sin bloqueo

| # | Pregunta | Resolución |
|---|----------|------------|
| 1 | ¿Proceso `bug-fix` (PBI) vs `feature` (runtime)? | Documentar y ejecutar como **`feature`** (**L-PROCESS**); el defecto permanece el mismo. |
| 2 | ¿Cuerpo Linear MAN-40 completo en disco? | No bloquea: canal + **PBI-1b004e99** + semilla refinada + laudos bastan (**L-TRACKER**). |
| 3 | ¿Reabrir bóveda MAN-36? | **No** — solo sello lógico + canal FS/nombres (**L-CA4-RESIDUE** / **L-NON-GOALS**). |
| 4 | ¿Qué es el sello canónico? | `PublishSeal` (token `.events/{family}/{id}.json`) o `event_id` (**L-LOGICAL-SEAL**); contraste sano = linear_bridge. |
| 5 | ¿Dead-letter local del github_bridge? | **Fuera** — no es backend del puerto (**L-HEALTHY** / **L-NON-GOALS**). |
| 6 | ¿`eda_port::runtime_db_path` se elimina o se privatiza? | Deja de ser API/sello de consumidores; cualquier resolución física queda encapsulada en el host (**L-HOST-ONLY**). Forma exacta (delete vs crate-private) = decisión de Dedalo/Tekton sin cambiar el *qué*. |
| 7 | ¿Allowlist del gate? | Host + tests/legado explícitos; cero consumidores productivos en allowlist por comodidad (**L-GATE**). |

## D5 — Veredicto

**ok** — Requisitos estabilizados. El *qué* queda cerrado: sello lógico (`PublishSeal`/`event_id`), host único del físico, inventarios L-SEAL-INVENTORY + L-FS-INVENTORY, CAs 1…7, tajos T0…T6, sin reabrir MAN-36. Evidencia viva confirma síntoma en `github_bridge::materialize_to_bus`. Tracker **MAN-40** / documento **PBI-1b004e99** / padre **MAN-36** / `execution_id` **d5f40be9-1866-4813-a5b2-28a26faed56c**.
