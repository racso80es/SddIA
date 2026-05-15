---
index_version: "1.0.0"
entity_family: "tools"
maintained_by_agent: "cumulo"
paths_ref: "SddIA/core/cumulo.paths.json"
directories_key: "tools"
indexed_at: "2026-05-15"
synchronization_note: "Cada fila debe coincidir con la cabecera YAML del archivo fuente indicado; columna Capabilities obligatoria."
---

# Índice de tools (Core SddIA)

Contrato normativo de la familia: `tools-contract.md` (no constituye una tool catalogada en esta tabla).

## Catálogo de definiciones (`{name}.md`)

| Archivo fuente | uuid | name | version | contract | context | Capabilities |
|----------------|------|------|---------|------------|---------|--------------|
| `iota-immutable-publisher.md` | `7c8be7da-d080-4ad0-b0b0-df43be376e46` | iota-immutable-publisher | 1.0.0 | tools-contract v1.2.0 | system-operations | `iota-publish`, `immutable-anchor`, `capsule-json-io` |

## Archivos en carpeta no catalogados como tool

Solo `tools-contract.md` (contrato de familia).

## Integridad (última pasada)

- **Sincronización:** una definición `{name}.md` con identidad atómica; reflejada en fila del catálogo.
- **Metadatos:** valores de la tabla (incl. **Capabilities**) copiados desde el YAML de `iota-immutable-publisher.md` al momento de indexación.
