---
feature_name: aut-138-mcp-run-check-cerbero
created: "2026-11-28"
process: bug-fix
branch_name: fix/aut-138-mcp-run-check-cerbero
persist_ref: docs/fixes/aut-138-mcp-run-check-cerbero
tracker_ref: AUT-138
execution_id: "748ad646-3b21-4d71-a161-62c911652d97"
---

# Spec — AUT-138

## Problema

`run_check` denegaba T4 headless por intersección vacía entre `mcp.allowed_executables` y la allowlist de `shell-executor`.

## Solución

1. Allowlist gobernada: `sddia-qa` y `sddia-run.sh` en shell-executor y manifiesto.
2. Validación por intención en `sddia-workspace-server` (solo `--process entity-manager`; veto `git`/`cargo`/otros).
3. Comprobación explícita manifiesto ∩ shell-executor.
4. Propagación `SDDIA_AGENT_RUNTIME_DEPTH` al hijo shell-executor.

## AC

| ID | Criterio |
|----|----------|
| AC-1 | `sddia-qa` ejecutable vía `run_check` |
| AC-2 | `sddia-run.sh --process entity-manager` ejecutable |
| AC-3 | Otros ejecutables/procesos DENY; `git` vetado |
| AC-4 | Depth propagado (+1) en env del hijo |
| AC-5 | Tests regresión intersección en `sddia-workspace-server` |