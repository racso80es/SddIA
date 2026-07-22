---
feature_name: kalma2-ppr-runtime-gaps-ppr-125
created: "2026-07-22"
process: bug-fix
branch_name: fix/kalma2-ppr-runtime-gaps-ppr-125
persist_ref: docs/fixes/kalma2-ppr-runtime-gaps-ppr-125
pbi_ref: docs/todos/done/[OPERATIVO] Kalma2 PPR runtime — F3 execute-process, git-manager y KM policy (PPR #125).md
document_id: PBI-PPR-125-KALMA2-RUNTIME-GAPS
uuid: 0a24332e-e120-480a-87eb-ec9854d27aaa
---

# Spec — Kalma2 PPR runtime gaps (G1–G4)

## Problema

Aduana PPR vía Kalma2 marcaba NO_APTO no bloqueantes:

| Gap | Causa raíz |
|-----|------------|
| G1 | Fase «Triaje técnico» → `action:execute-process` sin handler nativo → `simulated` |
| G2 | Prep solo `skill:git-manager`; `try_invoke_delegates` salta git-manager; agentes IDE rechazados por Shell |
| G3 | Obreros Tekton materializan `docs/todos/` sin política KM |
| G4 | Bus inyecta `pr_branch`; runtime lee `branch_name` → `None` |

## Solución

| Gap | Fix |
|-----|-----|
| G4 | `route_domain_core` duplica `branch_name`; `agent_runtime` coalesce; prompt Kalma2 lee `pr_branch` |
| G2 | Handler `ppr-prep-branch`: fetch + checkout + status vía `invoke_git_manager` |
| G1 | Handler `ppr-tech-triage`: `verify_process_integrity` in-process · `formal_execute_process: true` |
| G3 | Reglas prompt: seeds Kaizen solo `agent:cumulo` / `Kaizen_Alert_Required` |

## Fuera de alcance

Mutación genoma `pull-request-review.md` / `tekton.md` (entity-manager). Soft-dep kalma2-bridge SSL. Seed ARQUITECTURA revoked_entities.
