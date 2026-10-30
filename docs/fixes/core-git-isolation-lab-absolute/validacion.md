---
feature_name: core-git-isolation-lab-absolute
created: "2026-10-30"
process: bug-fix
branch_name: fix/core-git-isolation-lab-absolute
persist_ref: docs/fixes/core-git-isolation-lab-absolute
pbi_ref: AUT-6
tracker_ref: AUT-6
execution_id: "652511ad-2740-49f1-8ef5-ef9397fadefe"
global: APTO
pbi_archived: false
---

# Validación — AUT-6

| Check | Resultado |
|-------|-----------|
| AC-1 | `bind_core_tolerates_absolute_refine_lab_in_index` OK |
| AC-2 | `bind_core_still_rejects_absolute_nested_real_project` OK |
| `cargo test -p execute-process project_binding:: --lib` | 17/17 OK |
| `gate-evolution --range` | Pendiente post-commit |

Cierre Linear vía merge de PR (`done_gate: linear`).
