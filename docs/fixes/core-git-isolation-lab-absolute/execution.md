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
5. `sddia-qa evolution-register` alta `d5fbe8a2-0b49-458d-bc5f-5d3f1e2025ef`.
6. `delivery-close-cycle` — PR https://github.com/racso80es/SddIA/pull/423; evolution `d5fbe8a2`; re-push `58192e4a`.
7. `SDDIA_CLI_FOREGROUND=1 SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF=1 ./sddia-run.sh --process pull-request-review` — `execution_id` `60a6e3a7`; triaje técnico APTO; atestación `.SddIA/proofs/qa-attestations/core-git-isolation-lab-absolute.json` (relevo IDE: veredicto Argos simulated → sellado manual post-PPR).
