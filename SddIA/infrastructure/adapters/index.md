---
index_version: "1.0.0"
entity_family: "infrastructure-adapters"
maintained_by_agent: "cumulo"
paths_ref: "SddIA/core/cumulo.paths.json"
directories_key: "infrastructure_adapters"
indexed_at: "2026-08-29"
synchronization_note: "Cada fila debe coincidir con la cabecera YAML del archivo fuente indicado; columna status obligatoria."
---

# Índice de adaptadores de infraestructura (Core SddIA)

Contrato normativo de la familia: `adapters-contract.md` (no constituye un adaptador catalogado en esta tabla).

## Catálogo de definiciones (`{name}.md`)

| Archivo fuente | uuid | name | version | status | crate_name | impl_dir |
|----------------|------|------|---------|--------|------------|----------|
| `lancedb-thought-repo.md` | `0a22c260-2c5a-4aaa-a632-2c9a78e983e4` | lancedb-thought-repo | 1.0.0 | placeholder | sddia-infrastructure-lancedb-thought | lancedb_thought_repo |
| `lancedb-evolution-repo.md` | `ab9bef02-c2c1-426b-a2b2-ca1cc170f21c` | lancedb-evolution-repo | 1.0.0 | placeholder | sddia-infrastructure-lancedb-evolution | lancedb_evolution_repo |

## Archivos en carpeta no catalogados como adaptador

`adapters-contract.md` es el contrato de familia, no una fila del catálogo anterior.

## Integridad (última pasada)

- **Sincronización:** 2 adaptadores catalogados (lancedb-thought-repo, lancedb-evolution-repo).
- **Metadatos:** columnas alineadas a `adapters-contract.md` §1.
- **Estado:** ambos `placeholder` hasta integración física LanceDB (`PBI-CORE-LANCEDB-REAL-001`).
