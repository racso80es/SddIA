---
feature_name: refine-item-tracker-ref-remote
created: "2026-10-29"
process: refactorization
branch_name: refactor/refine-item-tracker-ref-remote
persist_ref: /home/racso/Proyectos/SddIA/docs/features/refine-item-tracker-ref-remote
pbi_ref: AUT-35
execution_id: "3876e61f-1e04-4140-abbd-702c0ebb416f"
---

# Objetivos — refine-item-tracker-ref-remote

## Misión

En Linear, `refine-item` debe sellar el issue ya leído (`fetch_issue`), rellenar `tracker_ref` desde el identificador remoto cuando falte en el frontmatter, y no crear issues duplicados vía `create_issue`.

## Alcance

- Motor `refine_item/run.rs` (AUT-35).
- Sin cambios en sustrato `file`.
- Documentación de ciclo bajo `docs/features/refine-item-tracker-ref-remote/`.

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
