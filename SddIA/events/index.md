---
index_version: "1.1.0"
entity_family: "events"
maintained_by_agent: "cumulo"
paths_ref: "SddIA/core/cumulo.paths.json"
directories_key: "events"
contracts_key: "events"
indexed_at: "2026-05-27"
synchronization_note: "Índice de familias; catálogo ECST en index.md de cada subcarpeta."
---

# Índice de eventos (Core SddIA) — Trinidad de Estímulos

Contrato normativo: [`events-contract.md`](events-contract.md) (raíz; no es Clase ECST).

## Familias (Simetría fractal)

| Familia | Códice | Clases (Fase 1) | Emisor principal |
|---------|--------|-----------------|------------------|
| `telemetry` | [`telemetry/index.md`](telemetry/index.md) | 1 | Solo CLI |
| `orchestration` | [`orchestration/index.md`](orchestration/index.md) | 0 | CLI / auditores (Fase 3) |
| `domain` | [`domain/index.md`](domain/index.md) | 7 | Cúmulo, Cerbero, acciones `emit-*` |

## Integridad

- **Total Clases:** 8 ECST (7 dominio migradas + `Raw_Execution_Finished`).
- **Raíz:** solo `events-contract.md`, este `index.md` y subcarpetas `{telemetry,orchestration,domain}/`.
- **Runtime:** instancias V3+ siguen en `eda_bus.pending` para dominio legacy (D0.2); rutas fractal `./.events/{family}/` en Fase 3.
