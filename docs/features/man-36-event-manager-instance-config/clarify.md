---



feature_name: man-36-event-manager-instance-config
created: "2026-11-09"
updated: "2026-11-09"
process: refactorization
purpose: Estabilización de alcance — configuración soberana por instancia para backends Event Manager; bóveda resuelve proveedor/ruta; Cúmulo solo topología lógica ECST
branch_name: refactor/man-36-event-manager-instance-config
persist_ref: docs/features/man-36-event-manager-instance-config
pbi_ref: MAN-36
parent_ref: MAN-35
document_id: PBI-0604fb3c
phase: mayeuta-stabilization
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version: "1.2.0"
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-36.jsonl"
forge_key: "582d8a29c4f772596e7de259a8af20da3b0352e2f1354317c12574eadd31fa16"
execution_id: "4163f7d8-455e-45e3-9600-2ae355b99bc5"
---
# Clarificación — man-36-event-manager-instance-config

Transcript Mayeuta (2026-11-09). Fase **Estabilización de alcance** del proceso `refactorization`.
Canal tracker: **MAN-36** (padre **MAN-35** / historia_ref). Documento semilla **PBI-0604fb3c**. Provider Linear: sin copia nueva bajo `docs/todos/`.

## D0 — Semilla y canal tracker

| Hecho | Laudo |
|-------|-------|
| Tracker `pbi_ref` | **MAN-36** (provider Linear). Canal leído: `.SddIA/state/tracker-outbound/MAN-36.jsonl` (`forge-pbi` → `refine-item` → `tracker-stamp` → `lifecycle-phase-action` / `refactorization`, 2026-11-08…09; última huella ~17:21Z). |
| Padre / historia | **MAN-35** (`event-manager-port-cutover`) — cutover productivo a `EventManagerPort` 1.2 con Redb; este PBI **no** reabre el contrato ni el cutover. |
| Antecedentes | **MAN-32** (puerto/adaptadores), **MAN-33** (composición DI). **MAN-37** (pirámide tests) es paralelo/adyacente, no bloqueante. |
| Documento | **PBI-0604fb3c** — forja sellada `.SddIA/state/forge-pbi/582d8a29c4f772596e7de259a8af20da3b0352e2f1354317c12574eadd31fa16.json` (`event_sealed: true`, `tracker_ref: MAN-36`). |
| Cuerpo Linear en disco | **No materializado**: el jsonl outbound no incluye descripción completa. **No bloquea**: semilla de orquestación + canal + AC/tajos bastan (**L-TRACKER**). |
| Evidencia viva (revalidada esta pasada) | Host: `config.rs` + `productive_factory` resuelven `SDDIA_EVENT_MANAGER_*` con precedencia SO > instancia > global; `events_redb_path` público retirado. Cúmulo **v1.12.0**: sin `event_bus` / `eda_*.store` físicos; solo tablas/ids/suscripciones + `env_hierarchy`. `.dev/.env.example` documenta contrato MAN-36 y depreca `SDDIA_EVENT_STORE`. `runtime_bridge::PublishSeal.target_path` = token lógico. Residuo CA-4: `eda_port::runtime_db_path` aún expone PathBuf vía `resolved_redb_path` (con fallback `DEFAULT_REDB_REL`) — no reabre el *qué*; forja/Argos deben cerrar o acotar consumo. |
| Semilla estabilizada | Desacoplar configuración física Redb/FS del Core; instancia resuelve proveedor/ruta desde bóveda; Cúmulo solo topología/identificadores lógicos ECST; contrato versionado de variables; política de rutas; Redb default productivo; FS solo test/migración. |
| Modo | `branch_pr` / `refactorization` — tajos T0…T5 verificables. |
| `execution_id` | **4fc6eadd-78b9-4022-8991-ed39d911e9af** |
| Continuidad documental | Blueprint Dedalo (`spec.md` / `plan.md`) y forja Tekton (`implementation.md` / `execution.md`) alineados a estos laudos; esta pasada **reafirma** el alcance sin reabrir el qué. |

## D1 — Matriz PBI × alcance estabilizado

| Afirmación (semilla / PBI) | Laudo |
|---------------------------|-------|
| Instancia ubica Redb vía bóveda (relativa o absoluta válida) sin tocar código ni `cumulo.paths.json` | **L-VAULT** + **L-PATH-POLICY** — CA-1. |
| Variables versionadas `SDDIA_EVENT_MANAGER_PROVIDER`, `SDDIA_EVENT_MANAGER_REDB_PATH`, `SDDIA_EVENT_MANAGER_FS_ROOT` | **L-VARS** — contrato de configuración de backend. |
| Precedencia SO > bóveda instancia (`.SddIA/.dev/.env`) > bóveda global (`.dev/.env`) | **L-PRECEDENCE** — CA-6; tests obligatorios. |
| Composition root `sddia-event-manager-host` resuelve, valida y construye adaptador | **L-COMPOSE** — consumidores solo ven `EventManagerPort` (CA-4). |
| Redb default productivo; FS solo roles test/migración; sin fallback FS productivo | **L-REDB-DEFAULT** / **L-FS-LIMIT** — CA-2 / CA-7; no regresión MAN-35. |
| Cero rutas físicas Redb / `.events/*` como almacenamiento en `cumulo.paths.json` | **L-CUMULO-LOGICAL** — CA-3; tablas/ids ECST permanecen lógicos. |
| Token `.events/{family}/{event_id}.json` = compat/observabilidad, no afirmación de fichero | **L-LOGICAL-EVENTS** — alineado con MAN-35 L-LOGICAL-PATH. |
| Sin `.env` ni config física en cápsulas; heredan entorno resuelto | **L-NO-CAPSULE-ENV**. |
| Rutas inválidas (vacía, traversal `..`) fallan antes de abrir backend; absolutas válidas se aceptan | **L-PATH-POLICY** — CA-5. |

## D2 — Laudos de estabilización (cerrados)

| ID | Decisión |
|----|----------|
| **L-DOMAIN** | Frontera productiva sigue siendo **`EventManagerPort`** / capacidad **`eda:event-manager`** v1.2. Este ciclo mueve **solo** la soberanía de configuración física a la instancia. |
| **L-VAULT** | Fuente soberana táctica: `.SddIA/.dev/.env`. Defaults de clone: `.dev/.env`. El entorno del SO tiene máxima precedencia. |
| **L-PRECEDENCE** | Orden cerrado: **SO > bóveda de instancia > bóveda global**. Cubierto por matriz de tests (CA-6). |
| **L-VARS** | Contrato versionado (sin secretos en plantillas): `SDDIA_EVENT_MANAGER_PROVIDER`, `SDDIA_EVENT_MANAGER_REDB_PATH`, `SDDIA_EVENT_MANAGER_FS_ROOT`. Retirar/deprecar documentación productiva de `SDDIA_EVENT_STORE` / modos dual/filesystem. |
| **L-COMPOSE** | Solo `sddia-event-manager-host` (y el adaptador que abre) conocen proveedor/ruta. Sustituir `events_redb_path()` cableado y consumidores que construyen/abren la ruta. Helpers que solo *delegan* al host no inventan layout; no deben reintroducir defaults cableados fuera del host. |
| **L-REDB-DEFAULT** | Proveedor productivo por defecto = Redb. Default de ruta tras resolución (si no configurada) = layout canónico bajo `repo_root` (equivalente histórico `.SddIA/runtime/events.redb`), **decidido por el host**, no por Cúmulo. |
| **L-FS-LIMIT** | Adaptador FS recibe raíz **solo** del composition root; disponible exclusivamente en roles `test`/migración. **Prohibido** fallback FS en runtime productivo. |
| **L-PATH-POLICY** | Relativas: respecto a `repo_root`; tras normalización no escapan de `repo_root` (rechazo `..`). Absolutas: permitidas, no vacías, sin segmentos `..`, no se reescriben a relativas. Vacía/inválida: fallo determinista **antes** de abrir el backend. |
| **L-CUMULO-LOGICAL** | Eliminar de `cumulo.paths.json` las claves de almacenamiento físico Redb (`event_bus`, `eda_bus.store`, `eda_fractal.store` y equivalentes). Conservar topología/identificadores lógicos ECST, contratos y suscripciones. `directories.events` (genoma) y tablas lógicas no son rutas de disco del store. |
| **L-LOGICAL-EVENTS** | Representación `.events/{family}/{event_id}.json` puede permanecer como token lógico; no implica I/O ni existencia de fichero. |
| **L-NO-CAPSULE-ENV** | No crear `.env` ni ficheros de configuración de backend dentro de directorios de cápsulas. |
| **L-DOCS** | README, Constitución, contratos EDA, plantillas de bóveda y gates describen el límite Core/instancia y la precedencia (**CA-8**). |
| **L-VERIFY** | Mantener contract suite MemoryMock / FS / Redb; añadir matriz de precedencia, validación de ruta, resolución relativa y aislamiento `tempdir`. |
| **L-NON-GOALS** | No cambiar semántica ECST ni protocolo EventManagerPort 1.2; no rehabilitar `SDDIA_EVENT_STORE` / dual/filesystem productivo; no secretos en Cúmulo/manifiestos/cápsulas; pirámide de tests EDA = MAN-37 (fuera). |
| **L-GIT** | Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`. |
| **L-GENOME** | Mutaciones de entidades del genoma **solo** vía `entity-manager`. |
| **L-TRACKER** | Provider Linear: sin ficheros nuevos de backlog en `docs/todos/`; KM/TODOs solo vía `agent:cumulo` o evento `Kaizen_Alert_Required`. |

## D3 — Tajos canónicos (orden de forja)

| Tajo | Contenido estabilizado |
|------|------------------------|
| **T0** | Contrato de configuración: variables, defaults, plantillas de bóveda (`.dev/.env.example` y guía de instancia) sin secretos; documentar precedencia. |
| **T1** | Resolución en `sddia-event-manager-host`: leer SO/bóvedas, validar rutas (**L-PATH-POLICY**), elegir proveedor, construir adaptador. |
| **T2** | Sustituir `events_redb_path()` y adaptar `runtime_bridge`, `eda_port`, audit/purger/daemons y demás consumidores productivos para no conocer el backend físico. |
| **T3** | Limpiar `cumulo.paths.json` (**L-CUMULO-LOGICAL**); gates arquitectónicos alineados (veto a open directo / rutas cableadas fuera del host). |
| **T4** | Documentación: README, Constitución, contratos EDA, límites Core/instancia (**CA-8**). |
| **T5** | Tests: contract suite MemoryMock/FS/Redb intacta; matriz precedencia; validación ruta; relativa vs absoluta; aislamiento `tempdir`; `validacion` APTO. |

## D4 — Preguntas abiertas → cerradas sin bloqueo

| # | Pregunta | Resolución |
|---|----------|------------|
| 1 | ¿Cuerpo Linear MAN-36 completo en disco? | No bloquea: canal + **PBI-0604fb3c** + semilla + laudos bastan (**L-TRACKER**). |
| 2 | ¿Default de ruta si la bóveda no define `SDDIA_EVENT_MANAGER_REDB_PATH`? | Layout canónico bajo `repo_root` (`.SddIA/runtime/events.redb`) resuelto **por el host**, no declarado como store en Cúmulo (**L-REDB-DEFAULT** / **L-CUMULO-LOGICAL**). |
| 3 | ¿Qué claves Cúmulo salen? | Al menos `event_bus`, `eda_bus.store`, `eda_fractal.store` (rutas físicas Redb). Tablas/ids y suscripciones permanecen. (Evidencia viva: ya ausentes en Cúmulo v1.12.0.) |
| 4 | ¿`SDDIA_EVENT_STORE` legacy? | Fuera de alcance rehabilitarlo; plantillas/docs productivas migran al contrato **L-VARS**. |
| 5 | ¿MAN-37 bloquea? | No: paralelo/adyacente; no es dependencia de forja de este desacoplo. |
| 6 | ¿Absolutas fuera de `repo_root`? | **Permitidas** (soberanía de instancia); solo se rechazan vacías o con `..` (**L-PATH-POLICY**). |
| 7 | ¿`runtime_db_path` en `eda_port` viola CA-4? | Residuo de forja: debe delegar solo al host **sin** reinventar default cableado en el consumidor; no cambia el alcance estabilizado. |

## D5 — Veredicto

**ok** — Alcance revalidado y estabilizado. El *qué* (bóveda soberana, vars versionadas, host único, Cúmulo lógico, política de rutas, CA-1…8, tajos T0…T5) permanece cerrado. Evidencia viva muestra forja avanzada en árbol (host/config, Cúmulo 1.12.0, plantilla bóveda, bridge lógico) con residuo CA-4 en `eda_port::runtime_db_path` para verificar/cerrar en validación — **no** bloquea estabilización. Tracker **MAN-36** / documento **PBI-0604fb3c** / padre **MAN-35** / `execution_id` **4fc6eadd-78b9-4022-8991-ed39d911e9af**.
