---
index_version: "1.0.0"
entity_family: "tools"
maintained_by_agent: "cumulo"
paths_ref: "cumulo.paths.json"
directories_key: "tools"
indexed_at: "2026-05-04"
synchronization_note: "Cada fila debe coincidir con la cabecera YAML del archivo fuente indicado; columna Capabilities obligatoria."
---

# Índice de tools (Core SddIA)

Contrato normativo de la familia: `tools-contract.md` (no constituye una tool catalogada en esta tabla).

## Catálogo de definiciones (`{name}.md`)

| Archivo fuente | uuid | name | version | contract | context | Capabilities |
|----------------|------|------|---------|------------|---------|--------------|

## Archivos en carpeta no catalogados como tool

Solo `tools-contract.md` (contrato de familia). No hay tools `{name}.md` catalogadas aún.

## Integridad (última pasada)

- **Sincronización:** tabla preparada con columna **Capabilities**; sin filas de catálogo hasta la primera forja de tool.
- **Metadatos:** al añadir una tool, copiar `uuid`, `name`, `version`, `contract`, `context` y `capabilities` desde el YAML fuente.
