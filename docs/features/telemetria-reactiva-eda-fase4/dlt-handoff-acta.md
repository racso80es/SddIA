---
feature_name: telemetria-reactiva-eda-fase4
created: "2026-05-27"
document_type: dlt-handoff-acta
decision_ref: D0.1
---

# Acta de transición DLT — Cúmulo → Radamanto (§4.0)

## Propósito

Documentar la **ventana dual criptográfica** de Fase 4: Radamanto asume sellado de gobernanza de herramientas sin retirar el anclaje PR/ECST de Cúmulo hasta cierre explícito post-CI.

## Matriz de jurisdicción DLT

| Evento dominio | Suscriptor DLT | Estado post-Fase 4 |
|----------------|----------------|-------------------|
| `PullRequest_Presented` | **Cúmulo** + `iota-immutable-publisher` | Sin cambio |
| `PullRequest_Merged` | **Cúmulo** + `iota-immutable-publisher` | Sin cambio |
| `Domain_Entity_Created/Updated/Deleted` | **Cúmulo** + `iota-immutable-publisher` | Sin cambio |
| `Tool_Degraded` | **Radamanto** + `iota-immutable-publisher` | Nuevo |
| `Status_Restored` | **Radamanto** + `iota-immutable-publisher` | Nuevo |
| `Tool_Deprecated` | **Radamanto** + `iota-immutable-publisher` | Nuevo |

## Ventana dual CI

| Test / smoke | Witness esperado |
|--------------|------------------|
| `test_eda_bus_v3plus.py` | Cúmulo en PR/ECST — **verde sin cambio** |
| `run-iota-ci-smoke.py` | Cúmulo — **verde sin cambio** |
| `test_radamanto_dlt_tool_status.py` | Radamanto con `SDDIA_LAB_RADAMANTO_DLT=1` |

## Prohibiciones Tekton (T4.2)

- No retirar entradas Cúmulo en `event-domain-subscriptions.json` para PR/ECST.
- No reasignar `PullRequest_*` a Radamanto en esta feature.

## Cierre de ventana (futuro)

Acta de retirada Cúmulo sobre gobernanza herramientas → Kaizen post Done global o feature dedicada; fuera de alcance Fase 4.
