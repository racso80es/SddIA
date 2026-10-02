---
feature_name: kalma2-agent-runtime-project-mcp
process: feature
branch: feature/kalma2-workspace-1xn-sequential
global: APTO
pbi_archived: true
pbi_ref: docs/todos/done/[ARQUITECTURA] Workspace 1×N — runtime de agentes con proyecto.md
---

# Validación — runtime agente + MCP

- `AGENT_PHASE` con `project_slug`: `project_root`, `mcp_servers` (descriptor con `env_keys`), `cwd` neutro `.SddIA/workspaces/.agent-runtime-neutral`.
- Smoke `initialize` al Workspace Server antes del agente; fallo → `MCP_BACKEND_UNSUPPORTED` (skip lab: `SDDIA_LAB_SKIP_MCP_SMOKE=1`).
- Tests: `agent_runtime_cwd_is_neutral_when_project_bound`, `workspace_mcp_smoke_initialize_when_binary_present`.
