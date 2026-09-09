---
feature_name: async-fracture-clarification-mayeuta
created: "2026-09-09"
process: feature
phases: 6
---

# Plan — async-fracture-clarification-mayeuta

## Fases

| # | Ítem | Vehículo | Estado |
|---|------|----------|--------|
| 0 | Init lab | `feature` relay IDE | ✅ `5f792c9f-722b-4135-bded-dfb0c34db92e` |
| 1 | PBI v1.2.0 + docs planificación | persist_ref + pending | ⏳ commit planificación |
| 2 | Clase ECST | `entity-manager` create event | |
| 3 | Acción genoma | `entity-manager` create action | |
| 4 | Suscripción | edit `event-orchestration-subscriptions.json` | |
| 5 | Handler + disparador enrich + tests | motor `execute-process` | |
| 6 | Update `mayeuta.md` | `entity-manager` update agent | |
| 7 | Evolution v1.1.2 + gate-evolution | `directories.evolution` | |
| 8 | `implementation.md` / `execution.md` / `validacion.md` | persist_ref | |
| 9 | DCC → PR | `delivery-close-cycle` | |
| 10 | CI verde → `accept-pr` | proceso `accept-pr` | |

## Orden de forja genoma

Prefijo Raw Kernel. Topología `docs/features/async-fracture-clarification-mayeuta/objectives.md` activa.

1. Evento (índice familia incluido).
2. Acción (índice actions).
3. Código motor + tests (`cargo test -p execute-process --lib --`).
4. Agente Mayeuta (cuerpo; uuid inmutable).
5. Evolution.

## Riesgos

- Firma 3-tuple de `analyze_fracture_kaizen`: actualizar tests existentes.
- Fail-open: nunca `Err` por LLM.
- `event-creator` debe corregir el conteo del índice orchestration (hoy «2» con 3 filas).
