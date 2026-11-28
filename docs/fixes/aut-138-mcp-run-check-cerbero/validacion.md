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
validated_head_sha: 0849d47be2855c3efdca0f86752d0fb7039e58af
validated_tree_sha: 5082a4a935b905b97d3116068fc6c41968799b53
diff_digest: sha256:b128d5cbbc43f1ed068d0fe1f0a60a984165c417a63e23a546b8cda9c928ac71
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