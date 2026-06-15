---
index_version: "1.0.0"
entity_family: "daemons"
maintained_by_agent: "cumulo"
paths_ref: "SddIA/core/cumulo.paths.json"
directories_key: "daemons"
indexed_at: "2026-06-15"
synchronization_note: "Cada fila debe coincidir con la cabecera YAML del archivo fuente indicado."
---

# Índice de Centinelas / Daemons (Core SddIA)

Contrato normativo de la familia: `daemons-contract.md` (no constituye un Centinela ejecutable catalogado).

## Catálogo de definiciones (`{name}.md`)

| Archivo fuente | uuid | name | version | contract | context | Capabilities | heartbeat_interval_seconds |
|----------------|------|------|---------|----------|---------|--------------|----------------------------|
| `event-watcher.md` | `f995cc89-22a7-488d-9b25-ddb1e5e3a4a4` | event-watcher | 1.0.0 | daemons-contract v1.0.0 | ecosystem-evolution | `eda-bus-watch`, `route-domain-event-delegate` | 30 |
| `telegram-watcher.md` | `89a10029-d4bd-4abc-bb08-ff59f6faf17f` | telegram-watcher | 1.0.0 | daemons-contract v1.0.0 | peripheral-sensing | `telegram-long-poll` | 30 |
| `github-bridge-watcher.md` | `456a3d9b-70b6-4113-9283-16ba6d142793` | github-bridge-watcher | 1.0.0 | daemons-contract v1.0.0 | source-control | `github-pr-bridge`, `dlt-oracle` | 60 |

## Archivos en carpeta no catalogados como Centinela

`daemons-contract.md` es el contrato de familia, no una fila del catálogo anterior.

## Integridad (última pasada)

- **Sincronización:** tres Centinelas legacy refactorizados CEN-04.
- **Metadatos:** columnas alineadas a `daemons-contract.md` §11.
