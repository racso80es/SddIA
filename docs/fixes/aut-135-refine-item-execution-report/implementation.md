---
feature_name: aut-135-refine-item-execution-report
created: "2026-11-27"
process: bug-fix
branch_name: fix/aut-135-refine-item-execution-report
persist_ref: docs/fixes/aut-135-refine-item-execution-report
pbi_ref: AUT-135
tracker_ref: AUT-135
execution_id: "68aebeb8-47ed-4a62-842e-13ad36b1643c"
---
# Implementación — AUT-135

## Cambio

`refine_item/run.rs`: helper `phases_for_terminal` + `MayeutaPhaseReport`; `envelope_status` recibe `phases` dinámico; sellado con `seal_mode: handler_deterministic`.

## Commits (Tekton)

Ver `execution.md` (anotaciones git-manager).