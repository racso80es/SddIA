---

feature_name: eda-event-manager-additive
created: "2026-11-06"
process: feature
branch_name: feat/eda-event-manager-additive
persist_ref: docs/features/eda-event-manager-additive
pbi_ref: MAN-32
document_id: PBI-EDA-001
related_adr: ADR-EDA-002
execution_mode: ADDITIVE_NON_BREAKING
capability_id: "eda:event-manager"
contract: "eda.event_manager"
contract_version: "1.0.0"
status: requisitos-estabilizados
mayeuta_verdict: ok
phase: mayeuta-stabilization
norm_ref: features-documentation-pattern@1.3.1
execution_id: "5513eab2-a948-4e5b-9476-3ff561266b95"
---
# Objetivos — eda-event-manager-additive

## Misión

Forjar **en paralelo (side-by-side, sin ruptura)** la capacidad **`eda:event-manager`** / contrato **`eda.event_manager` v1.0.0**: esquema canónico en Genoma, puerto Rust `EventManagerPort`, `version_gate` SemVer de doble versionado, mock en memoria, adaptadores built-in `redb` y `filesystem`, puente `ExternalCapsuleEventAdapter` (NFT-ready vía stdin/stdout) y `EventManagerDiFactory` kernel-safe — con suite de conformidad y tests de versionado/puente que certifiquen la estructura **antes** de cablearla al motor vigente.

**Por qué:** preparar el ecosistema para cápsulas NFT de eventos sin tocar el camino caliente (`execute-process`, `event-watcher`, `sddia-event-store`), alineado a la dirección de **ADR-EDA-002** pero materializando el dominio aditivo `eda.event_manager` (no sustituye aún el plano `eda.event_store` del ADR).

## Contexto estabilizado

| Hecho | Impacto |
|-------|---------|
| Tracker | MAN-32 (Linear); `audits/MAN-32.md` = auditoría LLM, no cuerpo del issue |
| Semilla | PBI-EDA-001 (`architectural-enabler`, `READY_FOR_FORGE`) |
| ADR-EDA-002 | APPROVED; nombra `eda.event_store` y locus `SddIA/tools/` — convergencia diferida |
| Taxonomía vigente | v1.0.9 sin `eda:*`; rigor id = `dominio:verbo` |
| Bindings vigentes | v1.7.0; una fila/capability; sin multi-rol formal en schema DI |
| `sddia-core` | Shared Kernel sin árbol `ports/` aún |
| Infra adapters | Jurisdicción viva (`adapters-contract.md`) — locus elegido para built-ins |

## Alcance (Pasos 1–2 aditivos)

### Paso 1 — Contrato, DI y versionado

- Esquema `SddIA/library/norms/capability-contracts/eda.event_manager.v1.schema.json` (Draft 2020-12; I/O `capsule-json-io`).
- Operaciones: `publish_ecst`, `claim_pending`, `record_delivery`, `move_to_dlq`, `replay_dlq`, `append_trace`, `handshake_version`.
- Alta en `capability-taxonomy.md` (`eda:event-manager`) y binding multi-rol en `capability-bindings.md` **sin alterar claves vigentes** (`default` redb, `fallback` fs, `test` memory-mock, slot `capsule:*`).
- Módulos `SddIA/sddia-core/src/ports/event_manager/`: contrato tipado, `trait EventManagerPort: Send + Sync`, `version_gate`, `InMemoryEventManagerMock` con fault injection.

### Paso 2 — Adaptadores y puente NFT

- `RedbEventAdapter` en `SddIA/infrastructure/adapters/redb_event_adapter/` (ACID + manifiesto dual-version).
- `FsEventAdapter` en `SddIA/infrastructure/adapters/fs_event_adapter/` (write-temp + rename; `pending/`/`claimed/`/`dlq/`/`traces/`).
- `ExternalCapsuleEventAdapter` en `SddIA/infrastructure/adapters/external_capsule_event_adapter/` (JSON contrato → binario/`.wasm` stdin/stdout; handshake; `SDDIA_BYPASS_THERMO_TOLL=1`).
- `EventManagerDiFactory`: binding → `version_gate` → `Arc<dyn EventManagerPort>` (**sin** enlace `redb` en el kernel; constructores inyectados).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC-1** | Integridad del código vigente: sin cambios de lógica activa en `execute-process`, `event-watcher` ni `sddia-event-store`; tests preexistentes en verde. |
| **AC-2** | Contrato S+ en Genoma: schema válido Draft 2020-12 + enlace taxonomía/bindings. |
| **AC-3** | Paridad tripartita: Contract Test Suite 100% (`cargo test`) sobre MemoryMock, Redb(tempdir) y Fs(tempdir). |
| **AC-4** | Certificación NFT-ready: test puente `ExternalCapsuleEventAdapter` + SemVer `version_gate` en verde; `cargo clippy -- -D warnings` sin warnings en alcance forjado. |

## Fuera de alcance

- Cablear plano interno ADR (`EventStorePort`) al motor/centinela.
- Migrar o reemplazar `sddia-event-store` productivo.
- Tokenizar/publicar cápsulas NFT reales (solo puente + slot).
- Dual-adapter de espejo o hot-swap en caliente.
- Reubicar adaptadores a `SddIA/tools/` (convergencia ADR posterior).
- Mutar `di.binding.schema.json` pattern provider (homologación `adapter:` diferida).

## Dependencias

- ADR-EDA-002 (APPROVED) — dirección; no cableado en este ciclo.
- Norma I/O `capsule-json-io`.
- Precedente DI: `capability_di_resolver` / `capability-bindings.md` / `capability-taxonomy.md`.
- Host: crate `sddia-core` (nuevo `ports/event_manager/`).
- Persistencia built-in: crate `redb` **solo** dentro del adaptador redb.
- Tracker: MAN-32.

## Ley aplicada

- Norma `features-documentation-pattern` v1.3.1 (un `.md` por fase + frontmatter).
- Proceso `feature`; Git exclusivamente vía `skill:git-manager`.
- `adapters-contract.md` para fichas/censo de infrastructure adapters.
- Clarificaciones canónicas: `clarify.md` (laudos L-*).
