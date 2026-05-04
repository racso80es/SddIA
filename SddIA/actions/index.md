---
index_version: "1.0.0"
entity_family: "actions"
maintained_by_agent: "cumulo"
paths_ref: "cumulo.paths.json"
directories_key: "actions"
indexed_at: "2026-05-04"
synchronization_note: "Cada fila debe coincidir con la cabecera YAML del archivo fuente indicado."
---

# Índice de actions (Core SddIA)

Contrato normativo de la familia: `actions-contract.md` (no constituye una acción catalogada en esta tabla).

## Catálogo de definiciones (`{name}.md`)

| Name | UUID | Versión | Context | Descripción | Capabilities |
|------|------|---------|---------|-------------|--------------|
| execute-process | f1e2d3c4-b5a6-4789-b012-cdef34567890 | 1.0.0 | ecosystem-evolution | Orquestación maestra para que Tekton ejecute un proceso del Core con resolución SSOT, fases ordenadas y gate Cerbero antes de cada cápsula. | `process-load-ssot`, `phase-graph-resolution`, `cerbero-policy-gate`, `capsule-fanout-skills-tools` |

## Archivos en carpeta no catalogados como acción

Ninguno. `actions-contract.md` es el contrato de familia, no una fila del catálogo anterior.

## Integridad (última pasada)

- **Sincronización:** una definición de acción con identidad atómica; reflejada en una fila.
- **Metadatos:** valores de la tabla (incl. **Capabilities**) copiados desde el YAML de `execute-process.md` al momento de indexación.
