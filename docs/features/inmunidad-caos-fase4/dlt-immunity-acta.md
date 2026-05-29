---
feature_name: inmunidad-caos-fase4
created: "2026-05-29"
document_type: dlt-immunity-acta
decision_ref: D0.4
---

# Acta DLT — Certificación de inmunidad (Caos Fase 4)

## Propósito

Documentar el **cuarto bucket** de sellado Radamanto: `System_Immunity_Certified`, sin alterar la ventana dual Cúmulo ↔ Radamanto establecida en Telemetría Reactiva Fase 4.

## Matriz de jurisdicción DLT (post-Caos Fase 4)

| Evento dominio | Suscriptor DLT | Estado |
|----------------|----------------|--------|
| `PullRequest_Presented` | **Cúmulo** + `iota-immutable-publisher` | Sin cambio |
| `PullRequest_Merged` | **Cúmulo** + `iota-immutable-publisher` | Sin cambio |
| `Domain_Entity_Created/Updated/Deleted` | **Cúmulo** + `iota-immutable-publisher` | Sin cambio |
| `Tool_Degraded` | **Radamanto** + `iota-immutable-publisher` | Sin cambio |
| `Status_Restored` | **Radamanto** + `iota-immutable-publisher` | Sin cambio |
| `Tool_Deprecated` | **Radamanto** + `iota-immutable-publisher` | Sin cambio |
| **`System_Immunity_Certified`** | **Radamanto** + `iota-immutable-publisher` | **Nuevo (Fase 4)** |

## Emisión y evidencia

| Paso | Responsable |
|------|-------------|
| Estímulo `Suite_Execution_Requested` | Acción `emit-suite-execution-requested` |
| Orquestación + manifiesto | Proceso `execute-suite` |
| Certificación domain | Proceso `execute-suite` (solo si campaña exitosa) |
| Witness Tangle | Radamanto vía fan-out domain |

## Smoke / CI

| Test | Witness esperado |
|------|------------------|
| `test_chaos_immunity_eda.py` | Radamanto con `SDDIA_LAB_SIMULATE_IOTA=1` |
| `test_radamanto_dlt_tool_status.py` | Regresión Tool_* — sin cambio |
| `run-iota-ci-smoke.py` | Cúmulo PR — sin cambio |

## Prohibiciones (Tekton)

- No asignar `System_Immunity_Certified` a Cúmulo.
- No retirar suscripciones Cúmulo en PR/ECST.
- No emitir certificación si `survival-manifest.md` ausente o nodos fallidos.

## Referencia

- `docs/features/inmunidad-caos-fase0/impact-analysis.md` § Jurisdicción DLT (AC0.4)
- `docs/features/telemetria-reactiva-eda-fase4/dlt-handoff-acta.md` (ventana dual base)
