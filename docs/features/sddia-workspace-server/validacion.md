---
feature_name: sddia-workspace-server
process: feature
branch: feature/kalma2-workspace-1xn-sequential
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[ARQUITECTURA] Workspace 1×N — Workspace Server MCP.md
---

# Validación — Workspace Server MCP

- `tools-contract` **1.6.0** (`io_mode: mcp-stdio`).
- Tool `sddia-workspace-server` 1.0.0 + crate nativo en `SddIA/tools/sddia-workspace-server/`.
- MCP: `initialize`, `resources/list`, `resources/read`, `tools/list`, `tools/call` (adaptadores fs/git/shell).
- Cerbero: deny fuera de contexto / whitelist `run_check`.
- Sandbox lectura + `env_ref` solo en hijos (tests unitarios).
- `cargo test -p sddia-workspace-server` y `cargo build --release` en verde.
