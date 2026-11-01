---
feature_name: refine-item-legacy-create-issue
created: "2026-11-01"
process: refactorization
branch_name: aut-35
persist_ref: docs/features/refine-item-legacy-create-issue
pbi_ref: AUT-35
tracker_ref: AUT-35
document_id: PBI-1ba6c13c
execution_id: "27ab6def-6796-4baa-a3e6-c569f96efd72"
historia_ref: MAN-14
---

# Objetivos — refine-item-legacy-create-issue

## Misión

Cerrar el residuo de AUT-35 en HEAD `c008493b`: `handlers/refine.rs` sigue creando un issue HU cuando falta `tracker_ref`. El handler productivo `refine_item/run.rs` ya sella el issue leído y no se toca en ese contrato.

## Alcance

- `SddIA/engine/execute-process/src/engine/handlers/refine.rs`: retirar `try_create_hu_issue` y la rama que lo llama.
- Test `ac_0c_creates_tracker_ref_when_missing` y cualquier aserción de `linear_direct_cycle_e2e` que exija ese alta.
- Tests del predicado de `tracker_ref` en `refine_item` (ausente → identifier; divergente → conflict) sin mock Linear.

## Fuera

- Reabrir el sello de `refine_item/run.rs` (PR 409, ya en `main`).
- El fallo Mayeuta de AUT-34 en ese mismo fichero.
- `forge-pbi` / `PARENT_REF_REQUIRED`.
- Mover AUT-35 a Done. El adapter no expone cambio de título.

## Ley aplicada

- Git vía `skill:git-manager` en la entrega. Esta sesión no hace checkout: `SDDIA_LAB_SKIP_GIT=1`. HEAD es `aut-35`.
- El motor canónico nombra `refactor/refine-item-legacy-create-issue`. No se creó esa rama.
