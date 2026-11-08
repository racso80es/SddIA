---




feature_name: eda-event-manager-redb-cutover
created: "2026-11-07"
process: refactorization
branch_name: refactor/eda-event-manager-redb-cutover
persist_ref: docs/features/eda-event-manager-redb-cutover
pbi_ref: MAN-33
tracker_ref: MAN-33
historia_ref: MAN-32
document_id: PBI-e4403517
phase: mayeuta-stabilization
execution_mode: BREAKING_CUTOVER
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version: "1.0.0"
productive_provider: "adapter:redb-event-adapter"
productive_role: default
status: requisitos-estabilizados
mayeuta_verdict: ok
norm_ref: features-documentation-pattern@1.3.1
execution_id: "6b4f2856-aa17-4a58-b317-c97a6e6c9d49"
---
# Objetivos — eda-event-manager-redb-cutover

## Misión

Ejecutar el **cutover definitivo** del camino activo de eventos al contrato **`eda.event_manager` v1.0.0** (MAN-32): los consumidores productivos resuelven `Arc<dyn EventManagerPort>` mediante **`EventManagerDiFactory`**, rol **`default`**, proveedor **`adapter:redb-event-adapter`**. Redb es adaptador **in-process** (no cápsula externa). Queda **un único camino productivo**, sin compatibilidad, shims, flags, ramas duales ni rollback legacy.

**Por qué:** MAN-32 forjó el plano aditivo sin tocar el camino caliente; MAN-33 liquida el acoplamiento a `sddia-event-store` / colas filesystem / configuración dual y deja el Genoma y el runtime alineados al binding productivo ya declarado.

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | MAN-33 (Linear); canal outbound sin cuerpo; semilla `.tmp/refactor-man-33.json` (`PBI-e4403517`) |
| Padre | MAN-32 `validacion.md` **APTO** — puerto, factory, schema, adapters y binding multi-rol listos |
| Binding vigente | `eda:event-manager` → `default` = `adapter:redb-event-adapter` (v1.8.0) |
| Deuda caliente | Crate `sddia-event-store` y dependientes directos aún en el camino activo |
| Ruta Cúmulo | `.SddIA/runtime/events.redb` |

## Alcance

### 1 — Cableado DI productivo

- Composition roots registran el builder Redb y resuelven rol `default` con `version_gate`.
- Consumidores dependen solo del puerto (`Arc<dyn EventManagerPort>`); **prohibido** importar el adaptador concreto fuera de composition roots.

### 2 — Migración / eliminación del inventario

Inventario sin pendientes (migrar al puerto **o** eliminar):

- `execute-process`
- `event-watcher`
- `sddia-daemon-runtime`
- `sddia-eda-catalog`
- `kalma2-bridge`
- `event-bus-audit`
- `sddia-qa`
- `repo-retention-purger`
- demás dependientes **directos** de `sddia-event-store`

### 3 — Limpieza del alcance migrado

- Sin imports/uso de `sddia-event-store`.
- Sin aperturas directas de `events.redb` / tablas / colas filesystem productivas.
- Sin factories antiguas ni `SDDIA_EVENT_STORE` dual/filesystem.
- Sin código muerto legacy en el diff de cutover.

### 4 — Configuración y Genoma

- Config mínima provider-neutral; ruta canónica Cúmulo `.SddIA/runtime/events.redb`.
- Bindings/evolution/docs describen el **único** camino final (procesos autorizados).
- Si falta operación en v1.0.0: evolucionar contrato/puerto **antes** de conservar legacy (**L-GAP**).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC-1** | Runtime productivo resuelve `default` → Redb vía `EventManagerDiFactory` + `version_gate`; consumidores tipados al puerto. |
| **AC-2** | Inventario L-INVENTORY sin pendientes: cada dependiente directo migrado o eliminado. |
| **AC-3** | Alcance migrado limpio: sin `sddia-event-store`, sin dual/filesystem, sin shims/flags/rollback legacy; búsqueda residual en verde. |
| **AC-4** | Tests: resolución default=Redb; fallo cerrado ante rol inválido; ausencia de `RedbEventAdapter` en consumidores; paridad funcional del camino migrado. |
| **AC-5** | Compilación: sandbox aislado + debug + release en verde sobre el alcance tocado. |
| **AC-6** | Genoma/bindings/evolution coherentes con el único camino final (sin documentar dual-path productivo). |

## Fuera de alcance

- Puente productivo `capsule:*` / NFT real.
- Usar fallback filesystem como camino activo.
- Compatibilidad, shims, feature-flags de store, ramas duales o rollback legacy.
- Re-forjar MAN-32 «desde cero» (salvo evolución justificada por **L-GAP**).
- Semillas Kaizen en `docs/todos/` (solo Cúmulo / `Kaizen_Alert_Required`).

## Dependencias

- MAN-32 / `docs/features/eda-event-manager-additive/` (contrato, puerto, factory, adapters).
- Binding `SddIA/core/capability-bindings.md` (`eda:event-manager`).
- Schema `SddIA/library/norms/capability-contracts/eda.event_manager.v1.schema.json`.
- Adaptador `SddIA/infrastructure/adapters/redb_event_adapter/`.
- Norma `features-documentation-pattern` v1.3.1.
- Tracker: MAN-33.

## Ley aplicada

- Norma `features-documentation-pattern` v1.3.1 (un `.md` por fase + frontmatter).
- Proceso `refactorization`; Git exclusivamente vía `skill:git-manager`.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
- Materialización piloto solo vía Workspace Server MCP (aduana 1×N).
