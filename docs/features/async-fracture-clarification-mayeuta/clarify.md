---
feature_name: async-fracture-clarification-mayeuta
created: "2026-09-09"
process: feature
purpose: Estabilización Filtro A PBI v1.2.0; laudos de implementación
version_clarify: "1.0.0"
execution_id: "5f792c9f-722b-4135-bded-dfb0c34db92e"
pbi_ref: docs/todos/pending/[FEATURE] Triaje asíncrono de fracturas inéditas (Mayeuta LLM).md
document_id: PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION
pbi_uuid: "b2c3d4e5-f6a7-4890-b1c2-d3e4f5a6b7c8"
pbi_version: "1.2.0"
---

# Clarificación — async-fracture-clarification-mayeuta

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/DCC + `SDDIA_LAB_ALLOW_DIRTY=1`. `execution_id` `5f792c9f-722b-4135-bded-dfb0c34db92e`. Rama `feat/async-fracture-clarification-mayeuta`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

## Decisiones

| ID | Laudo |
|----|-------|
| L-UNCLASS | `unclassified` se calcula **antes** del fallback. Tras el push, `root_causes` nunca está vacío. Catch-all `colaps` es cubo (no emite evento). |
| L-TRACE | Extraer de `## Traza de error`. No existe `## Traza Literal` ni `friction_id`. `status` = `abierto`. |
| L-H2 | Hipótesis = H2 hermano entre Conclusión y `## Criterio de cierre`. `###` interior lo borra el upsert del enrich. |
| L-INVOKE | `invoke_capsule_json(repo, "mayeuta-llm", {operation: SYNTHESIZE, prompt}, false)`. Prohibido `gemini-http-infer` y `temperature`. |
| L-FAIL | `success: true` siempre en fallos de CLI. Prohibido `Err`/`success: false` (dead-letter). Prohibido `System_Fracture_Detected` desde este flujo. |
| L-DISPATCH | Solo `actions::try_run_native`. Payload ECST = inputs. Routers intactos. |
| L-RBAC | Acción `knowledge-management`. Evento `ecosystem-evolution`. |
| L-FORGE | Clase + acción + update `mayeuta.md` vía `entity-manager`. Suscripción JSON en `SddIA/core/` (no DA-2). Motor Rust directo. UUID minteados, no los del PBI. |
| L-CA | CA1–CA6 = tests lib. Cero oracle LLM vivo. CA-CI de GitHub no APTO sin `run_id`. |
| L-LAUDO | `L-ENRICH-KINTSUGI-DETERMINISTA` intacto. |

## Fuera

Cápsula nueva; proceso `clarify-unknown-fracture`; mutar `mayeuta-llm.md`; SLO de milisegundos como gate.
