---
feature_name: aut-138-mcp-run-check-cerbero
created: "2026-11-28"
updated: "2026-11-28T07:30:00Z"
process: bug-fix
branch: fix/aut-138-mcp-run-check-cerbero
branch_name: fix/aut-138-mcp-run-check-cerbero
persist_ref: docs/fixes/aut-138-mcp-run-check-cerbero
tracker_ref: AUT-138
tracker_provider: linear
source_process: bug-fix
global: APTO
pbi_archived: false
checks:
  AC-1: APTO
  AC-2: APTO
  AC-3: APTO
  AC-4: APTO
  AC-5: APTO
  global: APTO
---

# Validación — AUT-138

## Veredicto

`global: APTO` — intersección gobernada Cerbero ∩ shell-executor; tests `sddia-workspace-server` (16) en verde; `gate-evolution` EVOL_OK.

## Evidencia

- `cargo test -p sddia-workspace-server` — regresión intersección + DENY cargo/feature/git.
- `child_runtime_depth` / propagación `SDDIA_AGENT_RUNTIME_DEPTH` en `run_check`.
- `gate-evolution --range origin/main...HEAD` → `EVOL_OK`.

## Seguridad (negativos)

- `cargo`, `--process feature`, `git` → DENY en `validate_governed_run_check`.
- Entradas muertas del manifiesto (`cargo`, `git`) excluidas de intersección efectiva.