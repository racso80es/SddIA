---
feature_name: eda-redb-01-store
created: "2026-10-21"
process: feature
items:
  - crate-sddia-event-store
  - workspace-member
  - tests-ac2-ca01-ca02
branch_name: feat/eda-redb-01-store
persist_ref: docs/features/eda-redb-01-store
document_id: PBI-EDA-REDB-01-STORE
uuid: "3f8c1a62-7d4e-4b91-8a27-6c5e0d9f1b34"
execution_id: "2567be4f-95cf-44fe-8bf8-01a568046cce"
---

# Implementación — eda-redb-01-store

## Touchpoints

| Artefacto | Ruta |
|-----------|------|
| Crate | `SddIA/sddia-event-store/` |
| Workspace | `SddIA/Cargo.toml` |
| Lock | `SddIA/Cargo.lock` (`redb` 2.6.4) |
| Tests | `SddIA/sddia-event-store/tests/store.rs` |

Sin cambios en `execute-process`, daemons, `SddIA/events/`, normas ni `cumulo.paths.json`.

## Propuestas aplicadas

- `EventStore` posee el `Database`. Las tablas no salen del crate.
- `insert_event` escribe evento, índice y N `pending` antes del `commit`.
- `replay` conserva el witness y solo cambia el estado a `pending`.
- CA-02 se afirma en el test `workspace_member_and_no_crates_dir`.
