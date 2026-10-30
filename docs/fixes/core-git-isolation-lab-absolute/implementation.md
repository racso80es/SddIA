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

# Implementación — AUT-6

| Archivo | Cambio |
|---------|--------|
| `SddIA/engine/execute-process/src/engine/project_binding.rs` | `core_nested_index_exempt`; hook en `assert_git_isolation_for_slug` para slug `core`; tests AC-1/AC-2 |
