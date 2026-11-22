---






feature_name: man-48-familia-capability-contract
created: "2026-11-22"
updated: "2026-11-22"
process: feature
purpose: Registro de implementación para MAN-48
branch_name: feat/man-48-familia-capability-contract
persist_ref: docs/features/man-48-familia-capability-contract
pbi_ref: MAN-48
document_id: PBI-FAMILIA-CAPABILITY-CONTRACT-POLIMORFISMO
status: implementado
execution_id: "a5fba947-f8c0-406c-b61f-9118bd175466"
---
# Implementación — man-48-familia-capability-contract

## Resumen de cambios por tarea

- **T1 (`71ed9b46`):** Actualizado `SddIA/norms/entidades-dominio-ecosistema-sddia.md` con la Norma de Identidad Polimórfica (Sustrato A `{name}.md` y Sustrato B `{name}.schema.json` con `"x-sddia"`), erradicando la referencia obsoleta a `spec.md` e incorporando la familia `capability-contracts`.
- **T2 (`05e96ac4`):** Forjados `SddIA/library/norms/capability-contracts/capability-contracts-contract.md` y `SddIA/library/norms/capability-contracts/index.md` con los 17 contratos vigentes (sin alterar UUIDs ni hashes de MAN-43 O13), y registrada `"capability_contracts"` en `contracts` de `SddIA/core/cumulo.paths.json` (`1.12.1`).
- **T3 (`97024954`):**
  - `SddIA/engine/execute-process/src/forges/capability_contract.rs`: añadido `sync_contract_index_row` para mantener sincronizada la fila en `index.md` tras `create`, `adopt` y `schema_json_patch` (+ test unitario).
  - `SddIA/engine/execute-process/src/engine/entity_manager.rs`: añadido `"capability-contract" => Some("SddIA/library/norms/capability-contracts")` en `dir_by_class`.
  - `SddIA/engine/execute-process/src/engine/sync_entity_index.rs`: añadido `("capability-contract", "SddIA/library/norms/capability-contracts/index.md")` en `index_map()`.
  - `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs`: añadidos `"daemon"` y `"capability-contract"` a `allowed` en `dlt_threshold_ok`.
- **T4 (`5c854a01`):**
  - `SddIA/engine/execute-process/src/engine/eda_coverage.rs`: añadido `("capability-contract", "SddIA/library/norms/capability-contracts")` a `ENTITY_DIRS` y soporte Sustrato B (`*.schema.json`, lectura de `/x-sddia/hash_signature`) en `parse_index_uuids` y `scan_orphans`.
  - `SddIA/tools/sddia-qa`: añadido `src/verify_capability_contracts.rs` y subcomando `verify-capability-contracts [--json]` en `src/main.rs` (+ `jsonschema` en `Cargo.toml`).
