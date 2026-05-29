---
feature_name: inmunidad-caos-fase4
created: "2026-05-29"
purpose: Decisiones Fase 4 y herencia del gate Fase 3
---

# Clarificación — Fase 4 (Estímulo EDA y Gobernanza Autónoma)

## Precondición (gate Fase 3)

Fase 3 cerrada con `validacion.md` APTO (AC3.1–AC3.3): ED `Suite`, `execute-suite`, `core-full-stress`, sub-workspaces aislados y `survival-manifest.md`. No se reabre genoma Suite salvo hallazgo bloqueante durante cableado ECST.

## Decisiones heredadas

| ID | Resolución | Uso en Fase 4 |
|----|------------|---------------|
| D0.4 | DLT inmunidad vía **Radamanto** (no Cúmulo) | F4-O6: cuarto bucket gobernanza |
| D0.7 | `survival-manifest.md` en workspace orquestador | Payload/evidencia de `System_Immunity_Certified` |
| D0.8 | Fase 4 = ECST; Fase 5 = README | Alcance acotado a eventos + DLT |
| D0.9 | PBI en `pending/` | `validacion.md` con `pbi_archived: false` |
| D3.12 | Suscripción ECST diferida a Fase 4 | F4-O3: `event-domain-subscriptions.json` |
| H18–H21 | Eventos y jurisdicción DLT ausentes | Resueltos en esta feature |

## Decisiones cerradas — Fase 4

| ID | Pregunta | Resolución |
|----|----------|------------|
| **D4.1** | ¿Quién emite `Suite_Execution_Requested`? | Acción indexada **`emit-suite-execution-requested`** (paridad `emit-pr-presented-event`); **prohibido** agentes obrero/IDE |
| **D4.2** | ¿Payload mínimo del estímulo? | REQUIRED: `suite_id`; OPTIONAL: `asset_id` (UUID Suite o kebab); FORBIDDEN: `branch`, `pr_url` |
| **D4.3** | ¿Suscriptor del estímulo? | `agent:tekton`, `process:execute-suite`; inputs mapeados desde payload (`suite_id`, opcional `execution_strategy`) |
| **D4.4** | ¿Quién emite `System_Immunity_Certified`? | Handler **`run_execute_suite`** tras `all_pass` y manifiesto escrito; **único** emisor del proceso `execute-suite` |
| **D4.5** | ¿Payload certificación? | REQUIRED: `suite_id`, `survival_manifest_path`, `orchestrator_execution_id`, `nodes_passed`, `nodes_total`; OPTIONAL: `asset_id`, `hash_signature_manifest` |
| **D4.6** | ¿DLT de inmunidad? | Suscriptor **`radamanto`** + `tool:iota-immutable-publisher` en `System_Immunity_Certified`; Cúmulo **no** suscribe |
| **D4.7** | ¿Ventana dual PR/ECST? | **Sin cambio** — acta paridad `telemetria-reactiva-eda-fase4/dlt-handoff-acta.md`; no retirar entradas Cúmulo |
| **D4.8** | ¿Modo lab del fan-out? | Smoke con `SDDIA_LAB_ROUTE_SYNC=1` + `SDDIA_LAB_SIMULATE_IOTA=1` (patrón `test_radamanto_dlt_tool_status.py`) |
| **D4.9** | ¿Suite smoke E2E? | `core-full-stress` vía acción estímulo → watcher/route → execute-suite → immunity |
| **D4.10** | ¿Fase Argos en `execute-suite.md`? | Añadir fase **Certificación inmunidad** con `delegates_to: agent:radamanto` (documental); implementación en handler |
| **D4.11** | ¿EDA coverage? | Upsert clases ECST, acción emisora, entradas suscripción en `eda-coverage.json` |
| **D4.12** | ¿Acta DLT en persist_ref? | `dlt-immunity-acta.md` en feature (síntesis D0.4 + matriz jurisdicción) — opcional en Tekton, recomendado en plan |

## Contrato común EDA Caos

| Campo / regla | Valor |
|---------------|-------|
| Bus padre | `./.events/pending/` (V3+ inmutable) |
| Familia | `domain` |
| Enrutador | `route-domain-event` + `event-watcher` |
| Estímulo → orquestador | Fan-out async; lab sync solo en tests |
| Certificación | Solo si `execute-suite` `success: true` y manifiesto existe |
| Fallo nodo / `fail_fast` | **No** emitir `System_Immunity_Certified` |

## Referencias

- Gate Fase 3: `docs/features/inmunidad-caos-fase3/validacion.md`
- Hallazgos: `docs/features/inmunidad-caos-fase0/impact-analysis.md` (H18–H21)
- PBI: `docs/todos/pending/PBI-INMUNIDAD-CAOS-SISTEMA-NERVIOSO.md` § Fase 4
- Ventana dual DLT: `docs/features/telemetria-reactiva-eda-fase4/dlt-handoff-acta.md`
