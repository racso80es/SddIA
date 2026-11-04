---
feature_name: aut-75-lifecycle-state-actions
created: "2026-11-04"
purpose: decision
process: bug-fix
branch_name: Fix_Events_Multiple_03
sighting_branch: Fix_Events_Multiple_03
persist_ref: docs/fixes/aut-75-lifecycle-state-actions
pbi_ref: AUT-75
document_id: PBI-38a1cf1a
uuid: bd2a24b3-9800-46e5-9724-303359242323
parent: AUT-20
execution_id: "e7138ac5-71fe-4407-b025-ab1eca978f35"
---

# Clarificaciones — AUT-75

## Decisiones

1. Rama de trabajo: `Fix_Events_Multiple_03`. El init canónico nombra `fix/aut-75-lifecycle-state-actions`; `SDDIA_LAB_SKIP_GIT=1` no hizo checkout. La cascada ancla la rama de sighting.
2. Principio: contenido, estado y metadatos del artefacto se escriben en la fase. El evento puede seguir existiendo para el bus; no escribe el estado.
3. Mapa cerrado por Racso en el refino `6e09811f-6177-4f9d-8fcd-b54f5a637b0d`:

| Hito | Estado | Metadatos |
| --- | --- | --- |
| Refino sellado | Todo | `refinement: refined`, fecha, agente |
| Fin de forja (objectives, clarify, spec, plan, execution, implementation) | In Validation | `forge_date`, `forge_by` |
| Argos APTO | Validated (nuevo) | `validated_status: apto`, fecha, agente |
| Argos no APTO | In Progress | `validated_status: no apto`, fecha, agente |
| Apertura de PR | In Review | fecha y agente de review |
| CI verde | Done | `done_status: ok`, fecha, agente |
| CI rojo | In Progress | `done_status: false`, fecha, agente |

4. Los «tags» del laudo son campos del frontmatter de la descripción (mismo canal que `refinement`, `refined`, `refined_by`). No se crean etiquetas Linear nuevas para esos metadatos.
5. La fecha de refino ya persistida en la clave `refined` no se renombra en este kaizen. Los campos nuevos usan los nombres de la tabla.
6. Durante la forja el ítem está In Progress. La entrada a ese estado sigue siendo el arranque de workspace (hoy `Work_Initiated`); el writer pasa a la fase.
7. `emit-validation-started-event` no es el cierre. In Validation ocurre al cerrar la forja, también cuando Ejecución quedó `simulated` o `skipped` y la cascada documental ya está.
8. El estado Linear `Validated` no existe en `state_map`. Hay que darlo de alta en el workflow del equipo y en el manifiesto antes de transicionar. El motor no crea estados de workflow.
9. Testigo AUT-74 (PR #434, sin In Validation ni In Review) explica la fricción. No se reabre el defecto MCP ni se fuerza ese merge.
10. Provider `linear`: sin ficheros nuevos en `docs/todos/`.

## Evidencia de partida

- Refino AUT-75: `status: refined`, `execution_id` `6e09811f-6177-4f9d-8fcd-b54f5a637b0d`, `item_type: kaizen`.
- `tracker-stamp`: `PBI_Refined` remoto ya es no-op («sellado en origen»). `Work_Initiated`, `Validation_Started`, `PullRequest_Presented` y `PullRequest_Merged` sí escriben estado.
- `maybe_emit_validation_started_after_tekton_agent` no emite si el status no es `executed`.
- Auditoría vigente: `docs/audits/ciclo-vida-software-sddia.md` v1.2.0 atribuye el movimiento al bus y no nombra Validated.
