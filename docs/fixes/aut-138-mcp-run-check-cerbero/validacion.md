---
feature_name: aut-138-mcp-run-check-cerbero
created: 2026-11-28
updated: 2026-11-28T07:30:00Z
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
validated_base_sha: 20962e48163973354c2e9211a261b045805d4839
validated_head_sha: 7cc135ef4b15868edc7f2d2b9cbc299b23d2b4d9
validated_tree_sha: f63a01318cc991716c216c18dc1b67aebee4e09b
diff_digest: sha256:e59eb49ed4e5dfe692b1af2444081eea704e195537285d2f6b49a6d9083bf8cc
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