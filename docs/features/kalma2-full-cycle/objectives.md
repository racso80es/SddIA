---
feature_name: kalma2-full-cycle
created: "2026-07-20"
process: feature
branch_name: feat/kalma2-full-cycle
persist_ref: docs/features/kalma2-full-cycle
document_id: PBI-KALMA2-FULL-CYCLE-RUNTIME
uuid: 527007fa-7200-41ee-84bb-202737f4f983
pbi_ref: docs/todos/pending/[FEATURE] kalma2-full-cycle — runtime de agentes y semántica de cierre (527007fa).md
depends_on:
  - docs/features/kalma2-process-dispatch
  - docs/features/kalma2-event-bus-integration
  - docs/features/kalma2-mayeuta-llm-router
execution_id_init: 956100c7-c03f-488b-af1e-2624f84bd0b0
evidence_event_id: e022814f-fc3a-441f-88c5-d60cb5e47e48
---

# Objetivos — kalma2-full-cycle

## Misión

Cerrar el hueco entre el **arranque EDA** desde Kalma2 (`workspace-init` + acuse + PEC) y la **gestión completa** del ciclo de vida (`bug-fix` \| `feature` \| `refactorization`: agentes → artefactos → cierre), con semántica de status que no confunda peaje orquestador con cierre de negocio.

## Punto objetivo

1. **Slice A:** Tras solo init + fases `simulated`/`skipped` L2, UI/status ≠ `completed` de negocio (`initialized` o `awaiting_agents`).
2. **Slice B:** Contrato + camino para ejecutar (o handoff auditable de) Dedalo→Tekton→Argos post-init.
3. **Slice C:** El hijo consume el cuerpo del PBI referenciado, no solo el path en el prompt.

## Evidencia ancla

| Campo | Valor |
|-------|--------|
| `event_id` | `e022814f-fc3a-441f-88c5-d60cb5e47e48` |
| Resultado engañoso | UI `completed` + solo `docs/fixes/7ad3175957d4/objectives.md` |

## Restricciones

- Bridge permanece ciego (sin write al bus).
- No reabrir emisión `Kalma2_Process_Requested`.
- No activar `SDDIA_TQM_FULL_CYCLE` sin runtime de agentes (slice B).
- Git solo vía `skill:git-manager`.

## Ley aplicada

- Jerarquía Acción → Agente → Skill → Tools.
- Cascada documental `features-documentation-pattern`.
