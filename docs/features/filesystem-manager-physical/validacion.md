---
feature_name: filesystem-manager-physical
process: feature
branch: feature/kalma2-workspace-1xn-sequential
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[ARQUITECTURA] Workspace 1×N — filesystem-manager físico 2.0.0.md
---

# Validación — filesystem-manager 2.0.0

- Skill `filesystem-manager` 2.0.0: cápsula Rust en `SddIA/skills/filesystem-manager/`; sin sección LLM-Native; `PATCH_FILE` en enum.
- Norma `skill-io-filesystem-manager-frozen` + clave `skill_io_filesystem_manager_frozen` en `cumulo.paths.json`.
- Sandbox `PROJECT_SCOPE_ESCAPE`; hunk inválido sin escritura parcial (tests unitarios).
- `cargo test -p filesystem-manager` y `cargo build -p filesystem-manager --release` en verde.
