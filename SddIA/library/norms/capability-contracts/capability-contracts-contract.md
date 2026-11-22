---
name: "capability-contracts-contract"
version: "1.0.0"
nature: "contract"
target_entity: "capability-contract"
substrate: "B"
description: "Contrato legal y estructural obligatorio para la familia capability-contracts (Sustrato B: {name}.schema.json con identidad x-sddia)."
---

# Contrato de Entidad: Capability Contract (Sustrato B)

Toda definición de contrato de capacidad bajo `directories.capability_contracts` (`SddIA/library/norms/capability-contracts/`) debe cumplir estrictamente este contrato legal para ser considerada válida por `entity-manager`, `eda_coverage` y las aduanas `sddia-qa`.

## 1. Reglas de Identidad Atómica (Sustrato B)

Cada contrato vive en un único fichero `{name}.schema.json` (JSON Schema Draft 2020-12 válido) en `directories.capability_contracts`.

El objeto JSON raíz DEBE incluir la clave de identidad gestionada `"x-sddia"` con exactamente los siguientes campos:

- `uuid`: (String) Identificador único universal UUID v4, único en todo el catálogo.
- `version`: (String) Versión semántica (`MAJOR.MINOR.PATCH`, SemVer estricto).
- `hash_signature`: (String) `sha256:<hex>` calculado sobre el JSON canónico (claves ordenadas lexicográficamente, sin espacios innecesarios) del objeto raíz **excluyendo** la clave `"x-sddia"`.

El stem `{name}` no puede contener `/`, `\`, `..`, comenzar por `.` ni terminar en `.schema` o `.json`.

## 2. Ciclo de Vida y Gobernanza (`entity-manager`)

El ciclo de vida de `capability-contract` está gobernado exclusivamente por la forja nativa (`SddIA/engine/execute-process/src/forges/capability_contract.rs`) invocada vía `entity-manager`:

- **`create`**: Requiere `schema_json` (objeto JSON Schema compilable sin `"x-sddia"`) y `schema_version` (SemVer, por defecto `1.0.0`). Genera UUID v4, calcula `hash_signature`, persiste `{name}.schema.json` y sincroniza `index.md`.
- **`update` (`adopt: true`)**: Adopta un `{name}.schema.json` preexistente sin `"x-sddia"` inyectando el bloque `"x-sddia"` por texto (preservando byte a byte el resto del schema) y sincroniza `index.md`.
- **`update` (`schema_json_patch`)**: Aplica parche RFC 6902 (`add` | `replace` | `remove`; prohibido parchear `/x-sddia`), exige bump SemVer obligatorio de `schema_version` respecto a `x-sddia.version`, recompila Draft 2020-12, recalcula `hash_signature` y sincroniza `index.md`.
- **`delete`**: **Prohibido** (`delete de capability-contract no soportado`).
- **Edición manual**: **Prohibida.** Cualquier desalineación entre el contenido del schema y `x-sddia.hash_signature` es detectada por `sddia-qa verify-capability-contracts` y por la forja.

## 3. Alineación Taxonómica y Contratos Auxiliares

1. Toda capacidad declarada en `SddIA/library/norms/capability-taxonomy.md` (`catalog[].contract`) DEBE tener su correspondiente `{contract}.schema.json` cuya `x-sddia.version` coincida exactamente con `catalog[].version`.
   - **Mapeo explícito:** La capacidad `eda:event-manager` (`contract: "eda.event_manager"`) se materializa en el fichero `eda.event_manager.v1.schema.json` (con `x-sddia.version` alineada a `catalog[].version`, actualmente `2.0.0`).
2. **Lista cerrada de contratos auxiliares fuera del catálogo taxonómico:**
   - `di.binding` (`di.binding.schema.json`) — contrato estructural de `SddIA/core/capability-bindings.md`.
   - `llm-registry` (`llm-registry.schema.json`) — contrato estructural del registro LLM de instancia.
   - Cualquier otro `{name}.schema.json` fuera de `capability-taxonomy.md` y de esta lista cerrada falla la aduana `sddia-qa verify-capability-contracts`.

## 4. Reglas del Índice de Familia (`index.md`)

En `directories.capability_contracts` DEBE existir `index.md` (mantenido por `agent:cumulo` y sincronizado automáticamente por `forges/capability_contract.rs`):

- Cabecera YAML con: `index_version`, `entity_family: "capability-contracts"`, `maintained_by_agent: "cumulo"`, `paths_ref: "SddIA/core/cumulo.paths.json"`, `directories_key: "capability_contracts"`.
- Sección `## Catálogo de definiciones (\`{name}.schema.json\`)` con tabla de columnas exactas:
  `Archivo fuente | uuid | name | version | hash_signature`.
- **Exclusiones:** `capability-contracts-contract.md` e `index.md` son artefactos de gobernanza de la familia y **no** figuran como filas del catálogo.
