---



feature_name: man-48-familia-capability-contract
created: "2026-11-22"
updated: "2026-11-22"
process: feature
purpose: Plan de ejecución por tareas atómicas para MAN-48
branch_name: feat/man-48-familia-capability-contract
persist_ref: docs/features/man-48-familia-capability-contract
pbi_ref: MAN-48
document_id: PBI-FAMILIA-CAPABILITY-CONTRACT-POLIMORFISMO
status: plan-aprobado
execution_id: "9745b97a-4bcd-4993-bc32-200cf79cbc3a"
---
# Plan de Ejecución — man-48-familia-capability-contract

| ID | Tarea | Archivos afectados | Criterio asociado |
|----|-------|--------------------|-------------------|
| **T1** | Actualizar `SddIA/norms/entidades-dominio-ecosistema-sddia.md` con Sustrato A/B y familia `capability-contracts` | `SddIA/norms/entidades-dominio-ecosistema-sddia.md` | CA-2 |
| **T2** | Forjar `capability-contracts-contract.md`, `index.md` (17 contratos) y registrar `contracts.capability_contracts` en `cumulo.paths.json` | `SddIA/library/norms/capability-contracts/capability-contracts-contract.md`, `SddIA/library/norms/capability-contracts/index.md`, `SddIA/core/cumulo.paths.json` | CA-1 |
| **T3** | Implementar sincronización de `index.md` en `forges/capability_contract.rs`, `dir_by_class` en `entity_manager.rs`, `index_map` en `sync_entity_index.rs` y `dlt_threshold_ok` en `eda_bus_topology.rs` + tests unitarios | `SddIA/engine/execute-process/src/forges/capability_contract.rs`, `SddIA/engine/execute-process/src/engine/entity_manager.rs`, `SddIA/engine/execute-process/src/engine/sync_entity_index.rs`, `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs` | CA-3 |
| **T4** | Extender `eda_coverage.rs` para Sustrato B (`*.schema.json`) y añadir aduana `verify-capability-contracts` en `sddia-qa` + tests | `SddIA/engine/execute-process/src/engine/eda_coverage.rs`, `SddIA/tools/sddia-qa/src/verify_capability_contracts.rs`, `SddIA/tools/sddia-qa/src/main.rs`, `SddIA/tools/sddia-qa/Cargo.toml` | CA-4, CA-5 |
