---
feature_name: refine-item-mayeuta
created: "2026-10-27"
process: refactorization
purpose: Laudos de relevo IDE (Mayeuta bloqueada sin MCP workspace) y cierre de alcance AUT-19
execution_id: "51caf6b7-c386-447b-a380-edf5e8c97d73"
document_id: PBI-f9627b0b
tracker_ref: AUT-19
historia_ref: AUT-18
---

# Clarificación — refine-item-mayeuta

Relevo: subproceso `cursor-agent` sin `sddia-workspace-server` en catálogo MCP; cascada sellada en IDE (Tekton) tras `refactorization` tarea 0.

## Decisiones

| ID | Laudo |
|----|-------|
| L-FORM | Primer objetivo del refinamiento de ítem: garantizar cabecera YAML (`---` o fence normalizado). Si falta, Preparación genera cabecera mínima y conserva el cuerpo. No equivale a `needs_clarification`. |
| L-PROC | Un solo proceso `refine-item` en códice software; baja de `refine-pbi` / `refine-hu` en Core con alias `PROCESS_RENAMED` una versión. |
| L-DOR | DoR por tipo en norma `backlog-item-refinement`; validador nativo por encabezados canónicos no vacíos. |
| L-MAY | Fase Mayeuta real vía `invoke_agent_phase`; relevo IDE → `awaiting_agents` + workspace; reentrada con `execution_id` y `verdict.json`. |
| L-SEAL | Sello condicionado: `refined` solo si DoR + `verdict.json` OK; si no, `pending` + `create_comment`, sin evento. |
| L-TIME | `SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_REFINAMIENTO` (default 900) solo para fase «Refinamiento Mayeuta». |
| L-FUERA | Esquema ECST `PBI_Refined`/`HU_Refined`; `list_issues`+`project_id`; título `forge-pbi`; diff `tracker-stamp` en PR previo; bucle interactivo en proceso. |
| L-FORGE | `refactorization` con `project_slug: core` exige `execution_profile.software_forge: true`. |

## Trazabilidad

- HU padre: AUT-18.
- PBI: AUT-19 (`refinement: refined` en tracker; cuerpo ampliado 2026-10-27 con L-FORM).
