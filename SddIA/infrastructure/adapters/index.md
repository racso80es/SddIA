---
index_version: "1.0.0"
entity_family: "infrastructure-adapters"
maintained_by_agent: "cumulo"
paths_ref: "SddIA/core/cumulo.paths.json"
directories_key: "infrastructure_adapters"
indexed_at: "2026-08-31"
synchronization_note: "Cada fila debe coincidir con la cabecera YAML del archivo fuente indicado; columna status obligatoria."
---

# Índice de adaptadores de infraestructura (Core SddIA)

Contrato normativo de la familia: `adapters-contract.md` (no constituye un adaptador catalogado en esta tabla).

## Catálogo de definiciones (`{name}.md`)

| Archivo fuente | uuid | name | version | status | crate_name | impl_dir |
|----------------|------|------|---------|--------|------------|----------|
| `lancedb-thought-repo.md` | `0a22c260-2c5a-4aaa-a632-2c9a78e983e4` | lancedb-thought-repo | 1.1.0 | active | sddia-infrastructure-lancedb-thought | lancedb_thought_repo |
| `lancedb-evolution-repo.md` | `ab9bef02-c2c1-426b-a2b2-ca1cc170f21c` | lancedb-evolution-repo | 1.1.0 | active | sddia-infrastructure-lancedb-evolution | lancedb_evolution_repo |
| `lancedb-preferences-repo.md` | `4c0103c3-5683-4219-aafd-1370c2b63c69` | lancedb-preferences-repo | 1.0.0 | active | sddia-infrastructure-lancedb-preferences | lancedb_preferences_repo |
| `external-capsule-event-adapter.md` | `d2c6a9b8-1f0e-4d30-85a4-c3b2a1098765` | external-capsule-event-adapter | 1.0.0 | active | sddia-infrastructure-external-capsule-event | external_capsule_event_adapter |
| `fs-event-adapter.md` | `e3a7b0c1-2d3e-4f50-96a5-b4c3d2e1f0a9` | fs-event-adapter | 1.0.0 | active | sddia-infrastructure-fs-event | fs_event_adapter |
| `redb-event-adapter.md` | `f4a8b1c2-3d4e-5f60-a7b8-c9d0e1f2a3b4` | redb-event-adapter | 1.0.0 | active | sddia-infrastructure-redb-event | redb_event_adapter |

## Archivos en carpeta no catalogados como adaptador

`adapters-contract.md` es el contrato de familia, no una fila del catálogo anterior.

## Integridad (última pasada)

- **Sincronización:** 6 adaptadores catalogados (LanceDB ×3 + EventManager aditivo ×3 / MAN-32).
- **Metadatos:** columnas alineadas a `adapters-contract.md` §1.
- **Estado:** LanceDB `active`; EventManager redb/fs/external `active` (PBI-EDA-001).
