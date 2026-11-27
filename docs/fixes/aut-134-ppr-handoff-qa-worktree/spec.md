---
feature_name: aut-134-ppr-handoff-qa-worktree
created: "2026-11-27"
process: bug-fix
phase: design
branch_name: fix/aut-134-ppr-handoff-qa-worktree
persist_ref: docs/fixes/aut-134-ppr-handoff-qa-worktree
pbi_ref: AUT-134
document_id: PBI-62095c41
uuid: ff0bfa1e-9de5-4841-940b-0f7496144737
tracker_ref: AUT-134
historia_ref: AUT-131
execution_id: "b6b262ed-c04e-429e-b23e-cb908eb3d70d"
---

# Spec — AUT-134 PPR handoff / qa-attestation / worktree

## Misión

Corregir tres defectos acoplados en `pull-request-review` cuando `accept_pr_handoff: true`: (D1) `author` omitido en handoff a `accept-pr`; (D2) `qa-attestation/1.0` emitida solo tras F7; (D3) worktree de `persist_ref` sucio tras auditoría.

## AC

| ID | Criterio | Oráculo |
|----|----------|---------|
| AC-1 | Handoff incluye `author` | `resolve_accept_pr_author` + payload child en `handoff_accept_pr` |
| AC-2 | Atestación antes de Handoff | `maybe_emit_qa_attestation_before_handoff` tras fase Veredicto |
| AC-3 | Worktree limpio post-PPR | `restore_persist_ref_worktree` + no `reconcile` en PPR |
| AC-4 | No regresión testigo | Re-ejecución lab PPR con handoff (manual/post-merge) |
| AC-5 | Tests motor verdes | `cargo test -p execute-process --lib` filtros PPR |

## Superficies

- `pull_request_review.rs`: `handoff_accept_pr`, `maybe_emit_qa_attestation_before_handoff`, `restore_persist_ref_worktree`
- `residual_runner.rs`: hook post-Veredicto; cierre PPR
- `agent_runtime.rs`: omitir `reconcile_persist_execution_id_for_reentry` si `process_name == pull-request-review`