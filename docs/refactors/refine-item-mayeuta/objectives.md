---
feature_name: refine-item-mayeuta
created: "2026-10-27"
process: refactorization
branch_name: refactor/refine-item-mayeuta
persist_ref: docs/refactors/refine-item-mayeuta
tracker_ref: AUT-19
document_id: PBI-f9627b0b
historia_ref: AUT-18
execution_id: "51caf6b7-c386-447b-a380-edf5e8c97d73"
---

# Objetivos — refine-item (Mayeuta real)

## Misión (AUT-19)

Sustituir `refine-pbi` y `refine-hu` por `refine-item` en el códice `codex-software-engineering`: refinamiento de contenido (Mayeuta), DoR por tipo, sello condicionado. **Primer paso del refinamiento:** garantizar formato del documento; si no hay cabecera YAML, generarla en Preparación.

## Criterios de aceptación (resumen)

1. Ítem sin cabecera → Preparación genera la mínima; el cierre en Linear la persiste.
2. `pbi` mínimo → `needs_clarification`, `pending`, comentario, sin `PBI_Refined`.
3. `pbi` con DoR → `refined`, evento correcto, `tracker-stamp` a Todo.
4. Tipo por label/frontmatter; `hu` sin `tracker_ref` conserva `create_issue`.
5. Relevo IDE: `awaiting_agents` + reentrada con `verdict.json`.
6. Timeout → `agent_timeout`, sin escritura Linear.
7. `updated_at` distinto → `conflict`.
8. Alias `PROCESS_RENAMED` para procesos retirados.
9. Payload ECST sin cambio; `source_process: refine-item`.

## Alcance técnico

- Norma `backlog-item-refinement` + codex 1.1.0 (`refine-item` en membership).
- Motor `handlers/refine_item/` (prepare, workspace, dor, seal, run).
- Contrato Mayeuta ampliado; emisores de eventos actualizados.
- Consumidores vivos listados en AUT-19 tarea 7.
- Tests unitarios + adaptación `linear_direct_cycle_e2e`.

## Fuera de alcance

ECST, `list_issues` GraphQL, título `forge-pbi`, PR `tracker-stamp`, bucle interactivo, `detached` por defecto.

## Ley aplicada

- Genoma solo vía `entity-manager` / `codex-creator`.
- Git vía `skill:git-manager`.
- Cierre: un PR, `validacion.md` APTO, AUT-19 → Done en Linear.
