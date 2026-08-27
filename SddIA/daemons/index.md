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
| `iota-publish-relay.md` | `78e94d53-0445-4394-b399-3e594cabc511` | iota-publish-relay | 1.0.0 | daemons-contract v1.0.0 | ecosystem-evolution | `iota-relay-supervise`, `dlt-publish-http` | 30 |
| `event-watcher.md` | `f995cc89-22a7-488d-9b25-ddb1e5e3a4a4` | event-watcher | 1.1.0 | daemons-contract v1.0.0 | ecosystem-evolution | `eda-bus-watch`, `route-domain-event-delegate` | 30 |
| `event-sweeper.md` | `3eafa012-2b71-47e5-b47e-467b59a3fd52` | event-sweeper | 1.0.0 | daemons-contract v1.0.0 | ecosystem-evolution | `eda-pending-sweep`, `kaizen-dead-letter-alert` | 30 |
| `telegram-watcher.md` | `89a10029-d4bd-4abc-bb08-ff59f6faf17f` | telegram-watcher | 1.1.0 | daemons-contract v1.0.0 | peripheral-sensing | `telegram-long-poll` | 30 |
| `github-bridge-watcher.md` | `456a3d9b-70b6-4113-9283-16ba6d142793` | github-bridge-watcher | 1.1.0 | daemons-contract v1.0.0 | source-control | `github-pr-bridge`, `dlt-oracle` | 60 |
| `email-watcher.md` | `773a11e7-3a42-4eba-a383-79dd6ef8c263` | email-watcher | 1.0.0 | daemons-contract v1.0.0 | peripheral-sensing | `imap-mailbox-poll`, `email-stimulus-injection` | 30 |

## Archivos en carpeta no catalogados como Centinela

`daemons-contract.md` es el contrato de familia, no una fila del catálogo anterior.

## Integridad (última pasada)

- **Sincronización:** cinco Centinelas catalogados (event-watcher, event-sweeper, telegram-watcher, github-bridge-watcher, email-watcher).
- **Metadatos:** columnas alineadas a `daemons-contract.md` §11.

