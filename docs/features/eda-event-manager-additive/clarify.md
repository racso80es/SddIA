---
feature_name: eda-event-manager-additive
created: "2026-11-06"
process: feature
purpose: Estabilización de requisitos — contrato EventManager aditivo, DI NFT-ready y adaptadores base
branch_name: feat/eda-event-manager-additive
persist_ref: docs/features/eda-event-manager-additive
pbi_ref: MAN-32
document_id: PBI-EDA-001
execution_id: "5513eab2-a948-4e5b-9476-3ff561266b95"
phase: mayeuta-stabilization
related_adr: ADR-EDA-002
execution_mode: ADDITIVE_NON_BREAKING
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version: "1.0.0"
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.1
---

# Clarificación — eda-event-manager-additive

Transcript Mayeuta (2026-11-06). Fase **Estabilización de Requisitos** del proceso `feature`. Revalidación sobre Genoma vivo + semilla PBI-EDA-001.

## D0 — Semilla y canal tracker

| Hecho | Laudo |
|-------|-------|
| Tracker `pbi_ref` | **MAN-32** (provider Linear). |
| `audits/MAN-32.md` | Existe; es **auditoría LLM** (`llm-interaction-audit-MAN-32`), **no** el cuerpo del issue Linear. |
| Cuerpo Linear del issue | **No materializado** bajo `docs/todos/` ni en `persist_ref` en esta sesión; no se inventa lectura remota. |
| SSOT operativo | Semilla refinada **PBI-EDA-001** (`architectural-enabler`, `READY_FOR_FORGE`, `ADDITIVE_NON_BREAKING`) + este `clarify.md` / `objectives.md`. |
| ADR | `Documentacion/informe_de_dise_o_arquitect_nico_desacople_di_del_event_store.md` (`ADR-EDA-002`, APPROVED, dominio `eda.event_store`). |

## D1 — Matriz PBI × realidad del Genoma (revalidada)

| Afirmación del PBI | Estado en repo | Laudo |
|--------------------|----------------|-------|
| Capacidad `eda.event_manager` v1.0.0 | **Ausente** en `capability-taxonomy.md` (v1.0.9, sin `eda:*`), `capability-bindings.md` (v1.7.0) y `capability-contracts/` | Alta aditiva; no tocar claves vigentes. |
| Esquema Draft 2020-12 | Contratos vigentes usan `{contract}.schema.json` (sin `.vN`); ADR/PBI piden `.v1.schema.json` | **L-SCHEMA-V** — filename versionado OK para esta familia (coexistencia MAJOR futura). |
| Puerto en `sddia-core/src/ports/event_manager/` | `sddia-core` = Shared Kernel mínimo (`lib.rs` + reexport `sddia-io`); **sin** árbol `ports/` | **L-PORT-HOST** — forjar árbol nuevo; sin dependencia `redb` en el kernel. |
| Adaptadores bajo `infrastructure/adapters/` | Jurisdicción viva (`adapters-contract.md` + censo LanceDB); **sin** crates event_manager aún | **L-LOCUS** — destino PBI alineado a infra; ADR `SddIA/tools/` queda para cableado posterior. |
| Binding multi-rol (default/fallback/test/`capsule:*`) | SSOT: **una fila** por `capability_id`; `di.binding.schema.json` solo `skill\|action\|tool` | **L-BIND-ROLES** — extensión aditiva del bloque de esta capacidad; sin mutar filas existentes ni el pattern del schema DI este ciclo. |
| Operaciones EventManager ≠ EventStore ADR | ADR: `append_event`/`ack_event`/…; PBI: `publish_ecst`/`record_delivery`/… | **L-OPS** — contrato distinto a propósito; no fusionar con `eda.event_store` este ciclo. |
| No tocar camino caliente | `execute-process`, `event-watcher`, `sddia-event-store` siguen vigentes | **L-NONBREAK** — AC-1; diff limpio en esos frentes. |

## D2 — Laudos de estabilización (cerrados)

| ID | Decisión |
|----|----------|
| **L-DOMAIN** | Este enabler materializa capacidad aditiva **`eda.event_manager`**. No implementa ni sustituye el plano interno ADR (`EventStorePort` / `eda.event_store`). Convergencia de nombres/rutas = PBI hijo de cableado. |
| **L-TAXON** | Catalog id (rigor `dominio:verbo`): **`eda:event-manager`**. Contract: **`eda.event_manager`**. Schema URI/file: **`eda.event_manager.v1.schema.json`**. Prohibido registrar el id con puntos (`eda.event_manager`) en `catalog` (mismo rigor que `tracker:lifecycle`). |
| **L-OPS** | Operaciones v1.0.0 cerradas: `publish_ecst`, `claim_pending`, `record_delivery`, `move_to_dlq`, `replay_dlq`, `append_trace`, `handshake_version`. Sobre JSON conforme a `capsule-json-io`. |
| **L-LOCUS** | Crates built-in: `SddIA/infrastructure/adapters/{redb_event_adapter,fs_event_adapter,external_capsule_event_adapter}/` + fichas `{name}.md` / censo `index.md` según `adapters-contract`. |
| **L-PORT-HOST** | Trait `EventManagerPort: Send + Sync`, tipos de error (`ContractVersionMismatch` / semilla `F-DI-CONTRACT-VERSION-MISMATCH`), `version_gate`, `InMemoryEventManagerMock` (+ fault injection) bajo `SddIA/sddia-core/src/ports/event_manager/`. |
| **L-BIND-ROLES** | Entrada nueva en `capability-bindings.md` para `eda:event-manager` con roles: `default` → redb, `fallback` → filesystem, `test` → memory-mock, slot `capsule:*` → puente externo. **Prohibido** alterar bindings previos. Prefijos `adapter:`/`mock:` son locales al bloque + factory; no mutar `di.binding.schema.json` este ciclo. |
| **L-FACTORY** | `EventManagerDiFactory`: rol → binding → `version_gate` → `Arc<dyn EventManagerPort>`. **Kernel-safe:** `sddia-core` no enlaza `redb`/crates de adapters; constructores built-in se inyectan vía callbacks/`Box<dyn Fn>` desde host/tests. `capsule:*` → `ExternalCapsuleEventAdapter` (stdin/stdout; hijo con `SDDIA_BYPASS_THERMO_TOLL=1`). |
| **L-PROVIDER-KIND** | Este ciclo **no** obliga a reclasificar adaptadores como `tool:` bajo `SddIA/tools/` (ADR §4). El slot `capsule:*` es el camino NFT-ready; built-ins viven como infrastructure-adapters. Homologación provider-pattern ADR = fuera. |
| **L-VERIFY** | Criterio principal: `run_event_manager_contract_suite` 100% sobre MemoryMock + Redb(tempdir) + Fs(tempdir); tests `version_gate` (`^1.0.0` acepta minor/patch, rechaza MAJOR); E2E puente con binario dummy; `cargo clippy -- -D warnings` en alcance forjado. |
| **L-NONBREAK** | Fuera: mutar lógica activa de `execute-process` / `event-watcher` / `sddia-event-store`; cablear plano ADR al motor; tokenizar NFT reales; dual-adapter espejo / hot-swap. |
| **L-GIT** | Git solo vía `skill:git-manager` / `./sddia-run.sh --tool git-manager`. |

## D3 — Preguntas abiertas → cerradas sin bloqueo

| # | Pregunta | Resolución |
|---|----------|------------|
| 1 | ¿`eda.event_manager` vs `eda.event_store`? | Dominios **paralelos**. Este PBI no cablea el store productivo. |
| 2 | ¿Taxonomía con puntos o dos puntos? | **`eda:event-manager`** en catalog; contract/schema con puntos. |
| 3 | ¿Tools (ADR) o infrastructure (PBI)? | **Infrastructure** ahora; tools = convergencia posterior. |
| 4 | ¿MAN-32 cuerpo Linear? | No bloquea: semilla PBI-EDA-001 + laudos bastan para handoff a Dedalo/Tekton. |
| 5 | ¿Factory contaminaría el kernel con `redb`? | **No** — **L-FACTORY** kernel-safe (callbacks de construcción). |

## D4 — Veredicto

**ok** — Requisitos estabilizados y revalidados. Handoff a planificación/forja (Dedalo/Tekton) con AC-1…AC-4 y laudos L-* como contrato de alcance.
