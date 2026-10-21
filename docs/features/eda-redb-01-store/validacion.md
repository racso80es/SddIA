---
feature_name: eda-redb-01-store
created: "2026-10-21"
process: feature
branch: feat/eda-redb-01-store
global: APTO
pbi_archived: true
document_id: PBI-EDA-REDB-01-STORE
execution_id: "2567be4f-95cf-44fe-8bf8-01a568046cce"
checks:
  AC-2: APTO
  CA-01: APTO
  CA-02: APTO
git_changes:
  - SddIA/sddia-event-store/
  - SddIA/Cargo.toml
  - SddIA/Cargo.lock
  - SddIA/evolution/b8dfedbb-d68d-47c9-a983-9e27f96d3d90.md
  - SddIA/evolution/Evolution_log.md
  - docs/features/eda-redb-01-store/
  - "docs/todos/done/[ARQUITECTURA] EDA Redb 01 — Store transaccional sddia-event-store.md"
---

# Validación — eda-redb-01-store

| ID | Evidencia | Veredicto |
|----|-----------|-----------|
| AC-2 | `ac2_two_subscribers_survive_reopen` y `concurrent_readers_and_distinct_writers`: dos testigos del mismo `event_id` siguen distintos tras reapertura y tras dos write-tx concurrentes | APTO |
| CA-01 | `cargo test -p sddia-event-store`: 9 passed, 0 failed. `key_dimension_does_not_corrupt_neighbor` deja intactos evento, índice, testigo, dead letter, sello y progress. `cargo clippy -p sddia-event-store --all-targets -- -D warnings` limpio | APTO |
| CA-02 | `SddIA/Cargo.toml` incluye `"sddia-event-store"`. El test `workspace_member_and_no_crates_dir` no encuentra `SddIA/crates/` | APTO |

Criterios locales. Ningún CA de este PBI depende de un check de GitHub Actions.

El diff de la HU y de los PBI 02–07 no entra en esta entrega.
