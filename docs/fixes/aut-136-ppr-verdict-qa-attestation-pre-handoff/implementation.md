---
feature_name: aut-136-ppr-verdict-qa-attestation-pre-handoff
created: "2026-11-27"
process: bug-fix
branch_name: fix/aut-136-ppr-verdict-qa-attestation-pre-handoff
persist_ref: docs/fixes/aut-136-ppr-verdict-qa-attestation-pre-handoff
tracker_ref: AUT-136
execution_id: "f3c98fff-6b95-4122-85b8-7c64463f3f5d"
---
# Implementación — AUT-136

## Mutaciones

| Archivo | Cambio |
|---------|--------|
| `pull_request_review.rs` | `promote_ppr_verdict_after_veredicto`; tests promote + attestation + handoff |
| `residual_runner.rs` | Hook post-F5: promote → `maybe_emit_qa_attestation_before_handoff` |
| `qa_attestation.rs` | `ppr_verdict_aprobado` → `pub(crate)` |
| `SddIA/evolution/b2a7f91e-….md` | Registro canónico motor |

## Verificación local

```bash
cd SddIA && cargo test -p execute-process promote_verdict ppr_verdict --lib
cd SddIA && cargo build -p execute-process --release
```