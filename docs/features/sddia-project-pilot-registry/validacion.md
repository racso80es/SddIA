---
feature_name: sddia-project-pilot-registry
process: feature
branch: feature/kalma2-workspace-1xn-sequential
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[ARQUITECTURA] Workspace 1×N — contrato de proyecto y registro del piloto.md
---

# Validación — registro piloto

- Contrato `project-config-contract` 1.1.0 (`env_ref`, campos opcionales).
- `project_binding.rs`: admite `1.0.0 | 1.1.0`, valida `env_ref`.
- Índice `.SddIA/projects/barcelonaxplorer.md` + manifiesto en BarcelonaXplorer.
- Tests `project_binding` y `contract_accepts_1_1_0` en verde (cargo test local).
