---
feature_name: core-git-isolation-lab-absolute
created: "2026-10-30"
process: bug-fix
branch_name: fix/core-git-isolation-lab-absolute
persist_ref: docs/fixes/core-git-isolation-lab-absolute
pbi_ref: AUT-6
tracker_ref: AUT-6
execution_id: "652511ad-2740-49f1-8ef5-ef9397fadefe"
---

# Ejecución — AUT-6

1. `SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_ALLOW_DIRTY=1 ./sddia-run.sh --process bug-fix` — workspace-init, rama `fix/core-git-isolation-lab-absolute`, `execution_id` `652511ad-2740-49f1-8ef5-ef9397fadefe`.
2. `core_nested_index_exempt` + hook en `assert_git_isolation_for_slug` (labs `_refine_lab_*` / `_forge_lab_*` y `docs/_fixtures` absolutos anidados).
3. Tests `bind_core_tolerates_absolute_refine_lab_in_index`, `bind_core_still_rejects_absolute_nested_real_project`; alineación test `done_gate`/`provider` (linear+git → contradice).
4. `cargo test -p execute-process project_binding:: --lib` — 17/17 OK.
5. Pendiente: `sddia-qa evolution-register`, `delivery-close-cycle`, PR.
