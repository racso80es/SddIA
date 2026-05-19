---
index_version: "1.0.0"
entity_family: "events"
maintained_by_agent: "cumulo"
paths_ref: "SddIA/core/cumulo.paths.json"
directories_key: "events"
contracts_key: "events"
indexed_at: "2026-05-19"
synchronization_note: "Cada fila debe coincidir con la cabecera YAML del archivo fuente indicado."
---

# Índice de eventos (Core SddIA)

Contrato normativo de la familia: `events-contract.md` (no constituye una Clase de Evento ejecutable).

## Catálogo de Clases ECST (`{name}.md`)

| Archivo fuente | uuid | name | event_type | version | contract | context | Capabilities |
|----------------|------|------|------------|---------|----------|---------|--------------|
| `domain-entity-deleted.md` | `a7c81b2f-b466-4b18-82c5-84ef0a5941b8` | domain-entity-deleted | Domain_Entity_Deleted | 1.0.0 | events-contract v1.0.0 | ecosystem-evolution | `domain_entity_deleted` |
| `domain-entity-updated.md` | `65dcff67-d392-4ab1-9977-2e320d3c8c34` | domain-entity-updated | Domain_Entity_Updated | 1.0.0 | events-contract v1.0.0 | ecosystem-evolution | `domain_entity_updated` |
| `domain-entity-created.md` | `1f518278-7a3d-4160-b757-a3661d263ec3` | domain-entity-created | Domain_Entity_Created | 1.0.0 | events-contract v1.0.0 | ecosystem-evolution | `domain_entity_created` |
| `pull-request-presented.md` | `5e488ae6-7cb2-4a2c-9725-4a7d4ce239ea` | pull-request-presented | PullRequest_Presented | 1.0.0 | events-contract v1.0.0 | ecosystem-evolution | `pull_request_presented` |
| `pull-request-merged.md` | `cfb8ce66-784e-4826-8a0a-a20c671e3a60` | pull-request-merged | PullRequest_Merged | 1.0.0 | events-contract v1.0.0 | dlt-auditing | `pull_request_merged` |

## Archivos en carpeta no catalogados como Clase

- `events-contract.md` — contrato de familia
- `index.md` — este índice

## Integridad (última pasada)

- **Sincronización:** 5 Clases ECST catalogadas (Fase 4 Ola C); cabeceras alineadas a filas del índice.
- **SSOT rutas:** Clases → `SddIA/events/`; instancias → `docs/events/` (`eda_bus`); personalización → `.SddIA/events/`.
