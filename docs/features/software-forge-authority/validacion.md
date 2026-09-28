---
feature_name: software-forge-authority
process: feature
branch: feature/kalma2-workspace-1xn-sequential
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[ARQUITECTURA] Workspace 1×N — candado software_forge.md
---

# Validación — candado software_forge

- `domain_authority.rs`: matriz `project_slug` × `software_forge` × `codex_slug` (tests incl. `kalma2_assistant_denied_even_with_project_slug`).
- `instance-creator`: `software_forge` default false; opt-in engineering (`engineering_domain_profile_software_forge_opt_in`).
- `sddia-installer-contract` **1.4.0** documenta opt-in y defaults.
