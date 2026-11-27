---
feature_name: aut-136-ppr-verdict-qa-attestation-pre-handoff
created: "2026-11-27"
process: bug-fix
branch_name: fix/aut-136-ppr-verdict-qa-attestation-pre-handoff
persist_ref: docs/fixes/aut-136-ppr-verdict-qa-attestation-pre-handoff
tracker_ref: AUT-136
execution_id: "f3c98fff-6b95-4122-85b8-7c64463f3f5d"
---
# Plan — AUT-136

## T1 — Visibilidad `ppr_verdict_aprobado`

En `qa_attestation.rs`, cambiar `fn ppr_verdict_aprobado` → `pub(crate) fn ppr_verdict_aprobado` (sin cambiar semántica).

## T2 — Promote post-Veredicto

En `pull_request_review.rs`, implementar `promote_ppr_verdict_after_veredicto` según spec (no-op si ya aprobado o si hay bloqueo; promote si `ppr_verdict_aprobado` / F5 limpia).

## T3 — Hook residual ordenado

En `residual_runner.rs`, tras fase «Veredicto y bloqueo»: llamar promote y después `maybe_emit_qa_attestation_before_handoff`. Conservar fallback terminal de atestación.

## T4 — Tests

- Unit: promote positivo / negativo (bloqueo documental, argos block).
- Unit: orden conceptual promote → emit (state.verdict presente cuando try_write_from_ppr).
- `cargo test -p execute-process --lib` filtros PPR/attestation.

## T5 — Evolución + cierre

Registro evolution + `delivery-close-cycle` en rama `fix/aut-136-ppr-verdict-qa-attestation-pre-handoff`.
