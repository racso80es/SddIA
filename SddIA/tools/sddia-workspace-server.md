---
uuid: "e8f9a0b1-c2d3-4e4f-a5b6-c7d8e9f0a1b2"
name: "sddia-workspace-server"
version: "1.0.0"
contract: "tools-contract v1.6.0"
domain_origin: "SddIA"
context: "filesystem-ops"
io_mode: "mcp-stdio"
capabilities:
  - "workspace-mcp-stdio"
  - "project-resources"
  - "capsule-adapters"
implementation_path_ref: "SddIA/tools/sddia-workspace-server"
telemetry_provided: false
hash_signature: "sha256:31743ae97db8ebdc4a06c03cb7a8f6808884e7ae26014276d5a2a6e9b5241af3"
---

# sddia-workspace-server

## Propósito

Servidor MCP stdio (Cerbero `run_check`, recursos sandbox, delegación a cápsulas allowlisted post AUT-138).

## Operación

Crate `sddia-workspace-server`; binario `SddIA/target/release/sddia-workspace-server`.

## Delivery

- Crate: `SddIA/tools/sddia-workspace-server/`
- Binario nativo: `SddIA/target/{debug,release}/sddia-workspace-server`
- **Desviación tools-contract §8:** subprocess + stdio persistente; no `wasm32-wasip1`.

## Invocación

```text
sddia-workspace-server --root {project_root} [--sddia-repo {core}] [--workspace-path {phase_ws}] [--instance-root {instancia}]
```

`env_ref` del proyecto se carga al arranque y se inyecta **solo** a cápsulas hijas (SO > proyecto > instancia > global; `VAULT_PRECEDENCE_KEYS` exceptuadas).

## Contextos RBAC

`filesystem-ops`, `source-control`, `system-operations` (Cerbero antes de cada `tools/call`).
