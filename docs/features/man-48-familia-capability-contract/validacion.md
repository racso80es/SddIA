---
feature_name: man-48-familia-capability-contract
created: "2026-11-22"
updated: "2026-11-22"
process: feature
phase: Verificación
agent: argos
branch: feat/man-48-familia-capability-contract
branch_name: feat/man-48-familia-capability-contract
persist_ref: docs/features/man-48-familia-capability-contract
pbi_ref: MAN-48
document_id: PBI-FAMILIA-CAPABILITY-CONTRACT-POLIMORFISMO
tracker_provider: linear
tracker_ref: MAN-48
norm_ref: features-documentation-pattern@1.3.1
global: APTO
pbi_archived: false
verdict: aprobado
checks:
  CA-1: APTO
  CA-2: APTO
  CA-3: APTO
  CA-4: APTO
  CA-5: APTO
  global: APTO
  branch: APTO
git_changes:
  - SddIA/core/cumulo.paths.json
  - SddIA/engine/execute-process/src/engine/eda_bus_topology.rs
  - SddIA/engine/execute-process/src/engine/eda_coverage.rs
  - SddIA/engine/execute-process/src/engine/entity_manager.rs
  - SddIA/engine/execute-process/src/engine/sync_entity_index.rs
  - SddIA/engine/execute-process/src/forges/capability_contract.rs
  - SddIA/library/norms/capability-contracts/capability-contracts-contract.md
  - SddIA/library/norms/capability-contracts/index.md
  - SddIA/norms/entidades-dominio-ecosistema-sddia.md
  - SddIA/tools/sddia-qa/Cargo.toml
  - SddIA/tools/sddia-qa/src/main.rs
  - SddIA/tools/sddia-qa/src/verify_capability_contracts.rs
  - docs/features/man-48-familia-capability-contract/clarify.md
  - docs/features/man-48-familia-capability-contract/objectives.md
  - docs/features/man-48-familia-capability-contract/plan.md
  - docs/features/man-48-familia-capability-contract/spec.md
  - docs/features/man-48-familia-capability-contract/implementation.md
  - docs/features/man-48-familia-capability-contract/execution.md
  - docs/features/man-48-familia-capability-contract/validacion.md
---

# Validación — man-48-familia-capability-contract

## Veredicto Global: APTO

| Criterio | Estado | Evidencia observada |
|----------|--------|---------------------|
| **CA-1** | APTO | `SddIA/library/norms/capability-contracts/capability-contracts-contract.md` y `index.md` (17 filas) creados; `contracts.capability_contracts` registrado en `SddIA/core/cumulo.paths.json` (`1.12.1`). |
| **CA-2** | APTO | `SddIA/norms/entidades-dominio-ecosistema-sddia.md` actualizado con Sustrato A (`{name}.md`), Sustrato B (`{name}.schema.json` + `"x-sddia"`) y familia `capability-contracts`. |
| **CA-3** | APTO | `sync_contract_index_row` en `forges/capability_contract.rs` (+ test `sync_contract_index_row_inserts_and_updates_deterministically` ok), `dir_by_class` en `entity_manager.rs`, `index_map` en `sync_entity_index.rs` y `dlt_threshold_ok` en `eda_bus_topology.rs`. |
| **CA-4** | APTO | `sddia-qa audit-eda-coverage --scan --json` → `{"indexed_entities":147,"orphan_count":0,"orphans":[]}`. |
| **CA-5** | APTO | `sddia-qa verify-capability-contracts --json` → `{"checked_contracts":17,"errors":[],"indexed_rows":17,"ok":true}`; `cargo test -p sddia-qa` (21 passed); `cargo test -p execute-process` (877 passed en workspace + `ac14_cancelled_moves_and_emits_pbi_cancelled` passed al re-ejecutar sin contención Redb), 0 warnings de compilación propios. |
