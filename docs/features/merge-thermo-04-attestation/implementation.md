# Implementación

| Componente | Cambio |
|------------|--------|
| `qa_attestation.rs` | Schema `qa-attestation/1.0`, TTL 86400, ancla `tree_sha` |
| `residual_runner.rs` | Escritura tras PPR exitoso con `verdict: aprobado` |
| `accept_pr.rs` | Skip genómico con `attestation_ref` + `merge_already_done`; borrado en Sincronización |
| `hook_common.sh` | `read_qa_attestation_hit`, `qa_attestation_path`, timing `attestation_hit` |
| `pre_push_gate.sh` | Omite `Local_QA_Requested` si atestación válida |
| `post_merge_gate.sh` | Inyecta `attestation_ref` relativo al repo |
| `test-merge-thermo-04-attestation.sh` | Mutaciones fail-closed + R-5 |

R-6 (bumps `pull-request-review` 2.4.0 / `accept-pr` 1.1.0 en codex): pendiente de `entity-manager` en ciclo posterior si el gate exige frontmatter explícito.
