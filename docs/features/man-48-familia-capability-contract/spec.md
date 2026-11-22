---
feature_name: man-48-familia-capability-contract
created: "2026-11-22"
updated: "2026-11-22"
process: feature
purpose: Especificación técnica para MAN-48 (consolidación de la familia capability-contract)
branch_name: feat/man-48-familia-capability-contract
persist_ref: docs/features/man-48-familia-capability-contract
pbi_ref: MAN-48
document_id: PBI-FAMILIA-CAPABILITY-CONTRACT-POLIMORFISMO
status: diseno-aprobado
---

# Especificación Técnica — man-48-familia-capability-contract

## 1. Alineamiento Normativo (`SddIA/norms/entidades-dominio-ecosistema-sddia.md`)

- Documentar la **Norma de Identidad Polimórfica** (`.cursorrules` §2):
  - **Sustrato A:** `{name}.md` con YAML Frontmatter (`uuid`, `name`, `version`, `hash_signature`, etc.). Se erradica la referencia obsoleta a `spec.md` en subcarpeta.
  - **Sustrato B (`capability-contract`):** `{name}.schema.json` con cabecera raíz `"x-sddia": { "uuid", "version", "hash_signature" }`, donde `hash_signature` es el SHA-256 del JSON canónico (claves ordenadas) sin `"x-sddia"`.
- Incluir **Capability Contracts** (`directories.capability_contracts`, `contracts.capability_contracts` → `SddIA/library/norms/capability-contracts/capability-contracts-contract.md`) en el listado de familias de entidades de dominio.

## 2. Contrato Legal, Índice Soberano y SSOT (`cumulo.paths.json`)

### 2.1 `SddIA/library/norms/capability-contracts/capability-contracts-contract.md`
- Definir el contrato legal de la familia `capability-contracts`:
  - Identidad atómica Sustrato B: `"x-sddia": { "uuid", "version", "hash_signature" }`, JSON Schema Draft 2020-12, canon de `hash_signature` sin `"x-sddia"`.
  - Ciclo de vida gobernado exclusivamente por `entity-manager` (`create` con `schema_json` + `schema_version`, `update` con `adopt: true` o `schema_json_patch` RFC 6902 con bump SemVer obligatorio; prohibición de `delete` y de edición manual).
  - Alineación obligatoria de `x-sddia.version` con `SddIA/library/norms/capability-taxonomy.md` (`catalog[].contract` ↔ `{name}.schema.json`), declarando explícitamente los contratos auxiliares fuera de catálogo taxonómico (`di.binding`, `llm-registry`) y el mapeo `eda.event_manager` → `eda.event_manager.v1.schema.json`.
  - Estructura obligatoria de `index.md` y exclusión de `capability-contracts-contract.md` e `index.md` como filas del catálogo.

### 2.2 `SddIA/library/norms/capability-contracts/index.md`
- Cabecera YAML: `index_version: "1.0.0"`, `entity_family: "capability-contracts"`, `maintained_by_agent: "cumulo"`, `paths_ref: "SddIA/core/cumulo.paths.json"`, `directories_key: "capability_contracts"`.
- Tabla `## Catálogo de definiciones (\`{name}.schema.json\`)` con columnas `Archivo fuente | uuid | name | version | hash_signature` poblada con los 17 contratos vigentes sin alterar ningún UUID, versión ni `hash_signature`.

### 2.3 `SddIA/core/cumulo.paths.json`
- Añadir `"capability_contracts": "SddIA/library/norms/capability-contracts/capability-contracts-contract.md"` en `"contracts"` y realizar bump SemVer de `"version"` (`1.12.0` → `1.12.1`).

## 3. Cierre del Ciclo en Motor (`execute-process`)

### 3.1 `SddIA/engine/execute-process/src/forges/capability_contract.rs`
- Añadir helper `sync_contract_index_row(dir: &Path, name: &str, uuid: &str, version: &str, hash: &str) -> Result<(), String>`:
  - Si `dir.join("index.md")` existe: leer el contenido, buscar la fila de `` `{name}.schema.json` `` (o columna `name == name`) dentro de la tabla del catálogo, reemplazarla si existe o insertarla ordenadamente (manteniendo orden lexicográfico por nombre) si no existe, y escribir atómicamente con `write_atomic`.
  - Si `index.md` no existe, no-op (compatibilidad con tests aislados en tempdir).
- Invocar `sync_contract_index_row` al completar `forge_create` (nuevo) y `forge_update` (`adopt` y `schema_json_patch`).

### 3.2 `SddIA/engine/execute-process/src/engine/entity_manager.rs`
- Añadir `"capability-contract" => Some("SddIA/library/norms/capability-contracts")` en `dir_by_class`.

### 3.3 `SddIA/engine/execute-process/src/engine/sync_entity_index.rs`
- Añadir `("capability-contract", "SddIA/library/norms/capability-contracts/index.md")` en `index_map()`.

### 3.4 `SddIA/engine/execute-process/src/engine/eda_bus_topology.rs`
- En `dlt_threshold_ok`: incluir `"capability-contract"` y `"daemon"` en `allowed`.

## 4. Aduanas QA (`eda_coverage.rs` y `sddia-qa`)

### 4.1 `SddIA/engine/execute-process/src/engine/eda_coverage.rs`
- Añadir `("capability-contract", "SddIA/library/norms/capability-contracts")` a `ENTITY_DIRS`.
- En `parse_index_uuids`: soportar tanto `` `{name}.md` `` como `` `{name}.schema.json` `` (preservando el stem `{name}` o sufijo `.schema.json` para distinguir el artefacto en disco).
  - Para resolver el artefacto en `scan_orphans`: si `entity_class == "capability-contract"` (o si `{name}.schema.json` existe), comprobar `base.join(format!("{name}.schema.json"))` y leer `x-sddia.hash_signature` del JSON (fallback a `hash_from_artifact` si falta); para Sustrato A, mantener `base.join(format!("{name}.md"))` + `parse_frontmatter`.

### 4.2 `SddIA/tools/sddia-qa`
- Añadir módulo `verify_capability_contracts.rs` y subcomando `verify-capability-contracts [--json]` en `main.rs`:
  1. Verificar existencia de `capability-contracts-contract.md`, `index.md` y `contracts.capability_contracts` en `cumulo.paths.json`.
  2. Enumerar todos los `*.schema.json` en `SddIA/library/norms/capability-contracts/` y todas las filas de `index.md`: comprobar paridad 1:1 (sin faltantes ni sobrantes).
  3. Para cada `*.schema.json`:
     - Compilar con `jsonschema::Validator::new` (Draft 2020-12).
     - Validar `"x-sddia"`: `uuid` UUID v4 único, `version` SemVer, `hash_signature` igual a `contract_hash(repo, &schema)` recalculado.
  4. Verificar paridad de versiones con `SddIA/library/norms/capability-taxonomy.md`: cada entrada `catalog[].contract` (mapeando `eda.event_manager` → `eda.event_manager.v1`) debe existir y coincidir en `version` con `x-sddia.version`; los únicos contratos permitidos fuera del catálogo taxonómico son los declarados como auxiliares (`di.binding`, `llm-registry`).
