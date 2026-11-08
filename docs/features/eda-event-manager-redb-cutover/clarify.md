---




feature_name: eda-event-manager-redb-cutover
created: "2026-11-07"
process: refactorization
purpose: Estabilización de alcance — cutover productivo a eda.event_manager v1.0.0 vía adapter:redb-event-adapter
branch_name: refactor/eda-event-manager-redb-cutover
persist_ref: docs/features/eda-event-manager-redb-cutover
pbi_ref: MAN-33
tracker_ref: MAN-33
historia_ref: MAN-32
document_id: PBI-e4403517
phase: mayeuta-stabilization
agents: mayeuta
execution_mode: BREAKING_CUTOVER
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version: "1.0.0"
productive_provider: "adapter:redb-event-adapter"
productive_role: default
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.1
execution_id: "6b4f2856-aa17-4a58-b317-c97a6e6c9d49"
---
# Clarificación — eda-event-manager-redb-cutover

Transcript Mayeuta (2026-11-07). Fase **Estabilización de alcance** del proceso `refactorization`. Hijo operativo de MAN-32 (`eda-event-manager-additive`, `validacion.md` **APTO**).

## D0 — Semilla y canal tracker

| Hecho | Laudo |
|-------|-------|
| Tracker `pbi_ref` | **MAN-33** (provider Linear); padre/historia **MAN-32**. |
| Canal outbound | `.SddIA/state/tracker-outbound/MAN-33.jsonl` — log de operaciones (`forge-pbi` / `refine-item` / `tracker-stamp` / `refactorization`); **no** vuelca el cuerpo Markdown del issue. |
| Cuerpo Linear | **No** materializado bajo `docs/todos/` (provider `linear`). |
| Semilla operativa | `.tmp/refactor-man-33.json` (`pbi_body` + `refined_constraints` + `refactor_goal`); `document_id` **PBI-e4403517**. |
| Precondición MAN-32 | Contrato `eda.event_manager` v1.0.0, puerto `EventManagerPort`, `EventManagerDiFactory`, adaptadores redb/fs/external-capsule y binding multi-rol **ya forjados** y validados APTO. |
| SSOT de este ciclo | Semilla MAN-33 + este `clarify.md` / `objectives.md`. |

## D1 — Matriz PBI × realidad del Genoma

| Afirmación del PBI | Estado en repo | Laudo |
|--------------------|----------------|-------|
| Capacidad / contrato v1.0.0 listos | Schema `eda.event_manager.v1.schema.json`; taxonomía + binding `eda:event-manager` (v1.8.0) con `default` → `adapter:redb-event-adapter` | **L-PRE** — cutover **consume** MAN-32; no re-forja el contrato salvo gap de operaciones. |
| `EventManagerDiFactory` + `version_gate` | `SddIA/sddia-core/src/ports/event_manager/` (`event_manager_factory.rs`, builders inyectables) | **L-DI** — resolución `Arc<dyn EventManagerPort>` por rol; builders Redb **solo** en composition roots. |
| Redb = adaptador in-process | Ficha `redb-event-adapter.md` (`type: infrastructure-adapter`, crate `sddia-infrastructure-redb-event`) | **L-INPROC** — **no** es cápsula externa ni slot `capsule:*`. |
| Camino caliente aún legacy | Crate `SddIA/sddia-event-store` presente; consumidores históricos siguen acoplados al store previo | **L-CUTOVER** — este ciclo **rompe** el camino legacy (opuesto a AC-1 de MAN-32). |
| Un solo camino productivo | Binding aún declara `fallback` fs y `capsule:*` | **L-PROD** — slots no productivos **pueden** permanecer en Genoma; el **runtime productivo** no los cablea ni los usa como compat. |
| Ruta canónica Cúmulo | `.SddIA/runtime/events.redb` existe en runtime | **L-PATH** — única ruta productiva resuelta por Cúmulo. |

## D2 — Laudos de estabilización (cerrados)

| ID | Decisión |
|----|----------|
| **L-MISSION** | Cutover definitivo: consumidores productivos resuelven `Arc<dyn EventManagerPort>` vía `EventManagerDiFactory`, rol **`default`**, proveedor **`adapter:redb-event-adapter`**, contrato **`eda.event_manager` v1.0.0**. |
| **L-INPROC** | Redb es adaptador **in-process** (infrastructure). Prohibido tratarlo como cápsula externa o exigir `capsule:*` en el runtime productivo. |
| **L-DI** | Factory + `version_gate` obligatorios. Consumers **no** importan `RedbEventAdapter` ni abren `events.redb`/tablas a mano. Builders Redb registrados solo en composition roots (host/daemon/binarios). |
| **L-SINGLE** | Un único camino productivo. **Prohibido:** compatibilidad, shims, feature-flags de store, ramas duales, `SDDIA_EVENT_STORE=dual\|filesystem`, rollback legacy, código muerto del store previo en el alcance migrado. |
| **L-GAP** | Si una operación del camino activo **no** cabe en v1.0.0: evolucionar primero contrato/puerto (MINOR/MAJOR según semántica); **no** conservar acceso legacy «mientras tanto». |
| **L-INVENTORY** | Inventario cerrado sin pendientes: `execute-process`, `event-watcher`, `sddia-daemon-runtime`, `sddia-eda-catalog`, `kalma2-bridge`, `event-bus-audit`, `sddia-qa`, `repo-retention-purger` y **demás dependientes directos** de `sddia-event-store`. Cada punto: migrado al puerto **o** eliminado. |
| **L-CLEAN** | En el alcance migrado: eliminar imports/uso de `sddia-event-store`, aperturas directas de `events.redb`, colas filesystem productivas, factories antiguas y configuración legacy/dual/filesystem. |
| **L-CONFIG** | Configuración mínima **provider-neutral** + ruta Cúmulo `.SddIA/runtime/events.redb`. |
| **L-OUT-RUNTIME** | Fuera del runtime productivo: puente `capsule:*`, fallback filesystem, cualquier dual-path o hot-swap. Pueden quedar como slots DI/test; **no** como camino activo. |
| **L-VERIFY** | Tests obligatorios: resolución `default`→Redb; fallo cerrado ante rol inválido; ausencia de `RedbEventAdapter` en consumidores; paridad funcional del camino migrado; búsqueda residual de símbolos/paths legacy; builds sandbox aislado + debug + release. |
| **L-GENOMA** | Bindings/evolution/docs de arquitectura final solo vía procesos autorizados (`entity-manager` / normas vigentes); describir el **único** camino final, no estados intermedios duales. |
| **L-GIT** | Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`. |

## D3 — Preguntas abiertas → cerradas sin bloqueo

| # | Pregunta | Resolución |
|---|----------|------------|
| 1 | ¿Reutilizar MAN-32 o forjar otro contrato? | **Reutilizar** v1.0.0; evolucionar solo si **L-GAP**. |
| 2 | ¿Redb es cápsula NFT? | **No** — **L-INPROC**. |
| 3 | ¿Mantener dual/filesystem «por si acaso»? | **No** — **L-SINGLE** / **L-CLEAN**. |
| 4 | ¿Borrar filas `fallback`/`capsule:*` del binding? | **No requerido** este ciclo; solo asegurar que el runtime productivo no las usa (**L-PROD** / **L-OUT-RUNTIME**). |
| 5 | ¿Cuerpo Linear ausente bloquea? | **No** — semilla `.tmp/refactor-man-33.json` + canal outbound + laudos bastan. |

## D4 — Veredicto

**ok** — Alcance estabilizado para cutover breaking. Handoff a Dedalo/Tekton con laudos **L-*** y AC de `objectives.md` como contrato de refactorization.
