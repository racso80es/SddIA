---
feature_name: refine-item-tracker-ref-remote
created: "2026-10-29"
process: refactorization
branch_name: refactor/refine-item-tracker-ref-remote
persist_ref: docs/features/refine-item-tracker-ref-remote
pbi_ref: AUT-35
execution_id: "3876e61f-1e04-4140-abbd-702c0ebb416f"
---

# Especificación — refine-item remoto sin create_issue (AUT-35)

## Cambio en `refine_item/run.rs`

1. Tras `fetch_issue`, si `tracker.provider` es remoto y el frontmatter no trae `tracker_ref`, usar `snap.identifier`.
2. Si el frontmatter trae `tracker_ref` distinto de `snap.identifier`, devolver `status: conflict` (sin escritura).
3. Eliminar `try_create_hu_issue`, `title_from_item` y la rama `item_type == hu && remote && tracker_ref.is_none()`.
4. `issue_target` permanece el issue leído; solo `update_issue_description` al sellar.

## Fuera de alcance

- Sustrato `file`.
- `forge-pbi` / `parent_ref`.
- Actualización del genoma `refine-item.md` (fase Registro en Tracker) vía `entity-manager` en PR separado o mismo PR si se forja entidad.

## Criterios de aceptación

- CA-1: HU remoto sin `tracker_ref` en FM → sellado en el mismo issue, sin `create_issue`.
- CA-2: FM `tracker_ref` ≠ identifier → `conflict`.
- CA-3: Kaizen con DoR completo → `PBI_Refined` con `tracker_ref` del issue leído.
- CA-4: Tests `refine_item` existentes en verde.
