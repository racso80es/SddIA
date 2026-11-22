---



feature_name: man-48-familia-capability-contract
created: "2026-11-22"
updated: "2026-11-22"
process: feature
purpose: Objetivos estabilizados — consolidación de la familia capability-contract (contrato legal, índice, motor y aduanas QA)
branch_name: feat/man-48-familia-capability-contract
persist_ref: docs/features/man-48-familia-capability-contract
pbi_ref: MAN-48
document_id: PBI-FAMILIA-CAPABILITY-CONTRACT-POLIMORFISMO
status: requisitos-estabilizados
execution_id: "9745b97a-4bcd-4993-bc32-200cf79cbc3a"
---
# Objetivos — man-48-familia-capability-contract

## Misión

Completar la consolidación de `SddIA/library/norms/capability-contracts/` como Familia de Entidades de Dominio de primera clase (Sustrato B de la Cicatriz Digital, `{name}.schema.json` con `"x-sddia"`) sobre el baseline de **MAN-43 O13**, dotándola de Contrato Legal, Índice Soberano, registro en Cúmulo, sincronización automática en motor (`forges/capability_contract.rs`, `entity_manager.rs`, `sync_entity_index.rs`, `eda_bus_topology.rs`) y aduanas QA ejecutables (`eda_coverage.rs` y `sddia-qa`).

Tracker **MAN-48** (parent **MAN-40**).

## Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| **CA-1** | Existen `SddIA/library/norms/capability-contracts/capability-contracts-contract.md` y `SddIA/library/norms/capability-contracts/index.md`, y `SddIA/core/cumulo.paths.json` declara `contracts.capability_contracts`. Las 17 filas de `index.md` coinciden exactamente en `uuid`, `name`, `version` y `hash_signature` con el `"x-sddia"` de cada `{name}.schema.json` en disco, sin alterar los 17 UUIDs/hashes de MAN-43 O13. | `sddia-qa verify-capability-contracts --json` + inspección de `cumulo.paths.json`. |
| **CA-2** | `SddIA/norms/entidades-dominio-ecosistema-sddia.md` documenta los dos sustratos canónicos de la Cicatriz Digital (Sustrato A `{name}.md` y Sustrato B `{name}.schema.json` con `"x-sddia"`) e incluye la familia `capability-contracts`. | Revisión de `SddIA/norms/entidades-dominio-ecosistema-sddia.md`. |
| **CA-3** | `run_capability_contract_forge` (`create`, `adopt`, `schema_json_patch`) sincroniza la fila en `index.md` si existe; `entity_manager.rs` mapea `capability-contract` en `dir_by_class`; `sync-entity-index` resuelve `capability-contract` hacia `SddIA/library/norms/capability-contracts/index.md` (`success: true`); `dlt_threshold_ok` acepta `capability-contract` y `daemon`. | Tests unitarios en `execute-process`. |
| **CA-4** | `sddia-qa audit-eda-coverage --scan --json` escanea `SddIA/library/norms/capability-contracts/index.md`, resuelve los 17 `{name}.schema.json` y reporta `orphan_count: 0`. | Ejecución de `sddia-qa audit-eda-coverage --scan --json`. |
| **CA-5** | `sddia-qa verify-capability-contracts --json` pasa en verde (paridad disco ↔ `index.md`, Draft 2020-12, `x-sddia`, paridad con `capability-taxonomy.md` + auxiliares) y `cargo test --workspace` compila y pasa sin warnings (`rust-warning-gate`). | `cargo test --workspace` y `sddia-qa verify-capability-contracts --json`. |
