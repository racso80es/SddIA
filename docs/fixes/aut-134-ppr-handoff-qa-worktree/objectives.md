---

feature_name: aut-134-ppr-handoff-qa-worktree
created: "2026-11-27"
process: bug-fix
branch_name: fix/aut-134-ppr-handoff-qa-worktree
persist_ref: docs/fixes/aut-134-ppr-handoff-qa-worktree
tracker_ref: AUT-134
document_id: PBI-62095c41
uuid: ff0bfa1e-9de5-4841-940b-0f7496144737
pbi_ref: AUT-134
execution_id: "71b46eba-9aea-4564-b0c9-602fe70e993d"
---
# Objetivos — aut-134-ppr-handoff-qa-worktree

## Misión

AUT-134: PPR handoff_accept_pr omite author; qa-attestation emitida post-F7; worktree persist_ref sucio. Ver cuerpo refinado en Linear AUT-134 (criterios ac1–ac5 en spec). Superficies: pull_request_review.rs handoff_accept_pr, residual_runner.rs try_write_from_ppr, agent_runtime reconcile_persist_execution_id_for_reentry.

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
