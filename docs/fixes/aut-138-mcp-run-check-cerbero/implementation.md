---
feature_name: aut-138-mcp-run-check-cerbero
created: "2026-11-28"
process: bug-fix
branch_name: fix/aut-138-mcp-run-check-cerbero
persist_ref: docs/fixes/aut-138-mcp-run-check-cerbero
tracker_ref: AUT-138
---

# Implementación

## Cambios

| Área | Archivo |
|------|---------|
| Aduana MCP | `SddIA/tools/sddia-workspace-server/src/cerbero.rs`, `server.rs` |
| Shell | `SddIA/skills/shell-executor/src/main.rs` |
| Manifiesto | `.SddIA/project.md` (`mcp.allowed_executables`) |

## Seguridad (DA-2)

- Sin shell genérico: solo `sddia-qa` y rutas `sddia-run.sh` con `--process entity-manager`.
- `manifest_shell_intersection` bloquea entradas muertas del manifiesto (p. ej. `cargo`, `git`).