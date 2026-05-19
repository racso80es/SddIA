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

*(Vacío — Hito 1 Ola C. Las Clases se forjarán en Fase 4 del plan de feature.)*

## Archivos en carpeta no catalogados como Clase

- `events-contract.md` — contrato de familia
- `index.md` — este índice

## Integridad (última pasada)

- **Sincronización:** genoma inicializado; catálogo de Clases pendiente de Fase 4 (Tekton).
- **SSOT rutas:** Clases → `SddIA/events/`; instancias → `docs/events/` (`eda_bus`); personalización → `.SddIA/events/`.
