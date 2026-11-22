---

feature_name: man-48-familia-capability-contract
created: "2026-11-22"
updated: "2026-11-22"
process: feature
purpose: Clarificación de alcance y baseline para MAN-48 (consolidación de la familia capability-contract)
branch_name: feat/man-48-familia-capability-contract
persist_ref: docs/features/man-48-familia-capability-contract
pbi_ref: MAN-48
document_id: PBI-FAMILIA-CAPABILITY-CONTRACT-POLIMORFISMO
status: clarificado
execution_id: "6a6c425a-eeab-4bee-8024-5cd9a4ab37f7"
---
# Clarificación — man-48-familia-capability-contract

## Contexto y refinamiento previo

El refinamiento de **MAN-48** (`.SddIA/workspaces/refine-item/8ea6c05b-acdb-47f1-8268-2ecccaf47ffb/item.refined.md`) estableció la separación explícita entre:

1. **Baseline ya materializado en `main` tras MAN-43 O13** (`commit 6fbf5845`, `SddIA/evolution/35ec5355-a3c8-48be-9358-336026684320.md`):
   - Polimorfismo en `.cursorrules` §2/§3 y `README.md`: Sustrato A (`{name}.md` con YAML Frontmatter) y Sustrato B (`{name}.schema.json` con `"x-sddia": { "uuid", "version", "hash_signature" }`).
   - Forja nativa `SddIA/engine/execute-process/src/forges/capability_contract.rs` (`create`, `update` con `adopt: true` / `schema_json_patch`, rechazo de `delete`).
   - Los 17 ficheros `*.schema.json` en `SddIA/library/norms/capability-contracts/` ya poseen bloque `"x-sddia"` válido y sus 17 UUIDs están registrados en `SddIA/core/eda-coverage.json`.

2. **Delta efectivo a implementar en MAN-48**:
   - **Alineamiento normativo:** `SddIA/norms/entidades-dominio-ecosistema-sddia.md` aún refiere únicamente a `spec.md` en subcarpeta y no documenta el Polimorfismo (Sustrato A / Sustrato B) ni la familia `capability-contracts`.
   - **Contrato Legal, Índice Soberano y SSOT:** Faltan `SddIA/library/norms/capability-contracts/capability-contracts-contract.md`, `SddIA/library/norms/capability-contracts/index.md` (con los 17 contratos actuales) y la clave `"capability_contracts"` dentro de `"contracts"` en `SddIA/core/cumulo.paths.json`.
   - **Sincronización en motor y DLT:**
     - `forges/capability_contract.rs`: sincronizar fila en `index.md` tras `create` / `update` (`adopt` o `schema_json_patch`) si `index.md` existe.
     - `entity_manager.rs`: añadir `"capability-contract" => Some("SddIA/library/norms/capability-contracts")` en `dir_by_class`.
     - `sync_entity_index.rs`: añadir `("capability-contract", "SddIA/library/norms/capability-contracts/index.md")` en `index_map()`.
     - `eda_bus_topology.rs`: añadir `"capability-contract"` y `"daemon"` a `allowed` en `dlt_threshold_ok`.
   - **Aduanas QA:**
     - `eda_coverage.rs`: añadir `("capability-contract", "SddIA/library/norms/capability-contracts")` a `ENTITY_DIRS` y extender `parse_index_uuids` + `scan_orphans` para soportar artefactos Sustrato B (`{name}.schema.json`, leyendo `x-sddia.hash_signature`).
     - `sddia-qa`: exponer subcomando `verify-capability-contracts [--json]` (y ejecutarlo en la aduana QA) verificando paridad disco `*.schema.json` ↔ `index.md`, compilación JSON Schema Draft 2020-12, `x-sddia` (UUID v4 único, SemVer, `hash_signature` canónica recalculable) y paridad de versión con `capability-taxonomy.md` más la lista cerrada de contratos auxiliares (`di.binding`, `llm-registry`, mapeo `eda.event_manager` → `eda.event_manager.v1.schema.json`).

## Decisiones de alcance

- **No alterar** ninguno de los 17 `*.schema.json` existentes (UUIDs, versiones y `hash_signature` sellados en MAN-43 O13 permanecen intactos).
- En `sync_entity_index.rs`, verificar cómo opera `row_exists` sobre `index.md` cuando la primera columna es `` `{name}.schema.json` `` y la columna `name` tiene `{name}` (p. ej. `agenda.persist`): `index.md` debe usar `Archivo fuente | uuid | name | version | hash_signature` con columna `name` igual al stem `{name}` para compatibilidad con `sync-entity-index` (`key_column: "name"`).
