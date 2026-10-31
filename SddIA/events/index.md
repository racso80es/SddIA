---
index_version: "1.2.0"
entity_family: "events"
maintained_by_agent: "cumulo"
paths_ref: "SddIA/core/cumulo.paths.json"
directories_key: "events"
contracts_key: "events"
indexed_at: "2026-07-19"
synchronization_note: "Índice de familias; catálogo ECST en index.md de cada subcarpeta. Recuentos alineados a telemetria-activa-domain-entity-updated."
---

# Índice de eventos (Core SddIA) — Trinidad de Estímulos

Contrato normativo: [`events-contract.md`](events-contract.md) (raíz; no es Clase ECST).

## Familias (Simetría fractal)

| Familia | Códice | Clases ECST | Emisor principal |
|---------|--------|-------------|------------------|
| `telemetry` | [`telemetry/index.md`](telemetry/index.md) | 4 | CLI + centinelas de Clase |
| `orchestration` | [`orchestration/index.md`](orchestration/index.md) | 4 | CLI / auditores / hooks |
| `domain` | [`domain/index.md`](domain/index.md) | 40 | Cúmulo, Cerbero, Radamanto, acciones `emit-*` |

## Integridad

- **Total Clases:** 48 ECST (40 domain + 4 telemetry + 4 orchestration).
- **Raíz:** solo `events-contract.md`, este `index.md` y subcarpetas `{telemetry,orchestration,domain}/`.
- **Runtime:** default `SDDIA_EVENT_STORE=redb` → SSOT `.SddIA/runtime/events.redb` (`events-contract.md` §4). Rutas lógicas: dominio legacy `eda_bus.pending` (`.events/pending/{id}.json` puede no existir en disco); fractal `./.events/{family}/` o store según modo.
- **Telemetría activa:** `Raw_Execution_Finished` → Radamanto → `Domain_Entity_Telemetry_Captured` → `memory-evolution-ingest` (no confundir con CRUD `Domain_Entity_Updated`).
