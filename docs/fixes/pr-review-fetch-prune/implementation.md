---
feature_name: pr-review-fetch-prune
created: "2026-05-22"
process: bug-fix
branch_name: fix/pr-review-fetch-prune
---

# Implementación — Fix fetch aduana PR

## Touchpoints

| # | Artefacto | Cambio |
|---|-----------|--------|
| H1 | `execute_process_capsules.py` | `fetch` + `prune: True` en `capsule_pr_review_branch_prep` |
| H2 | `execute_process_capsules.py` | `_workspace_task_name`, `_workspace_process_label`, `is_workspace_init_phase(..., process_def)` |
| H3 | `execute_process_capsules.py` | `run_workspace_init` soporta `bug-fix`, `docs/fixes/`, `bug_summary` |
| H4 | `execute_process_capsules.py` | `_infer_persist_ref_from_branch` → `docs/fixes/` para `fix/` |
| H5 | `execute_process_capsules.py` | `_pr_review_required_docs` — sin `plan.md` en fixes |
| H6 | `SddIA/process/bug-fix.md` | § Perfil laboratorio vs runtime IDE |
