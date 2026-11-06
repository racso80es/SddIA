---
feature_name: eda-event-manager-additive
created: "2026-11-06"
process: feature
branch_name: feat/eda-event-manager-additive
persist_ref: docs/features/eda-event-manager-additive
pbi_ref: MAN-32
tracker_ref: MAN-32
execution_id: "5513eab2-a948-4e5b-9476-3ff561266b95"
document_id: PBI-EDA-001
---

# Implementación — MAN-32

## Código y Genoma

- Contrato JSON Schema: `SddIA/library/norms/capability-contracts/eda.event_manager.v1.schema.json`
- Taxonomía: `SddIA/library/norms/capability-taxonomy.md` (entrada `eda:event-manager`); índice `SddIA/library/norms/index.md`
- Bindings: `SddIA/core/capability-bindings.md` — fila multi-rol `eda:event-manager`
- Puerto: `SddIA/sddia-core/src/ports/event_manager/` — trait, errores, `version_gate`, `InMemoryEventManagerMock`, `run_event_manager_contract_suite`, `EventManagerDiFactory`
- Workspace Cargo: `SddIA/Cargo.toml` — miembros `redb_event_adapter`, `fs_event_adapter`, `external_capsule_event_adapter`
- Adaptadores:
  - `SddIA/infrastructure/adapters/redb_event_adapter/` — `RedbEventAdapter`
  - `SddIA/infrastructure/adapters/fs_event_adapter/` — `FsEventAdapter`
  - `SddIA/infrastructure/adapters/external_capsule_event_adapter/` — `ExternalCapsuleEventAdapter`, bin `eda-event-manager-capsule-dummy`, test `tests/e2e_dummy.rs`
- Censo adapters: `SddIA/infrastructure/adapters/index.md` + fichas `*-event-adapter.md`

## No tocado (AC-1)

- `execute-process`, `event-watcher`, `sddia-event-store` — sin cambios en este ciclo.
