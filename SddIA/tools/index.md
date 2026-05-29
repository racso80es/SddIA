---
index_version: "1.0.0"
entity_family: "tools"
maintained_by_agent: "cumulo"
paths_ref: "SddIA/core/cumulo.paths.json"
directories_key: "tools"
indexed_at: "2026-05-29"
synchronization_note: "Cada fila debe coincidir con la cabecera YAML del archivo fuente indicado; columna Capabilities obligatoria."
---

# Índice de tools (Core SddIA)

Contrato normativo de la familia: `tools-contract.md` (no constituye una tool catalogada en esta tabla).

## Catálogo de definiciones (`{name}.md`)

| Archivo fuente | uuid | name | version | contract | context | Capabilities |
|----------------|------|------|---------|------------|---------|--------------|
| `eda-lab-smoke-may20.md` | `96679492-4f06-4bbc-ae46-00100fb73c94` | eda-lab-smoke-may20 | 1.0.0 | tools-contract v1.2.0 | ecosystem-evolution | `eda_lab_smoke_may20` |
| `iota-immutable-publisher.md` | `7c8be7da-d080-4ad0-b0b0-df43be376e46` | iota-immutable-publisher | 1.0.0 | tools-contract v1.2.0 | system-operations | `iota-publish`, `immutable-anchor`, `capsule-json-io` |
| `markdown-table-editor.md` | `b2c4e6f8-1a3d-4e5b-9c7d-8e1f2a3b4c5d` | markdown-table-editor | 1.0.0 | tools-contract v1.2.0 | ecosystem-evolution | `markdown-table-parse`, `markdown-table-row-upsert`, `markdown-table-row-delete`, `markdown-table-persist`, `capsule-json-io` |
| `io-choke.md` | `a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5e` | io-choke | 1.0.0 | tools-contract v1.3.0 | chaos-engineering | `io-choke`, `chaos-io-stress`, `capsule-json-io` |
| `schema-corruptor.md` | `b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6f` | schema-corruptor | 1.0.0 | tools-contract v1.3.0 | chaos-engineering | `schema-corruptor`, `chaos-telemetry-stress`, `capsule-json-io` |
| `sandbox-breacher.md` | `c3d4e5f6-a7b8-4c9d-8e1f-2a3b4c5d6e7f` | sandbox-breacher | 1.0.0 | tools-contract v1.3.0 | chaos-engineering | `sandbox-breacher`, `chaos-sandbox-stress`, `capsule-json-io` |
| `send-telegram-notification.md` | `e4f5a6b7-c8d9-4e0f-a1b2-c3d4e5f6a7b8` | send-telegram-notification | 1.0.0 | tools-contract v1.2.0 | ecosystem-evolution | `send-telegram-notification`, `telegram-send-message`, `capsule-json-io` |

## Archivos en carpeta no catalogados como tool

Solo `tools-contract.md` (contrato de familia).

## Integridad (última pasada)

- **Sincronización:** una definición `{name}.md` con identidad atómica; reflejada en fila del catálogo.
- **Metadatos:** valores de la tabla (incl. **Capabilities**) copiados desde el YAML de `iota-immutable-publisher.md` al momento de indexación.
