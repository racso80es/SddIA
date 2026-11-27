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
| `SddIA/engine/execute-process/src/engine/qa_attestation.rs` | `ppr_verdict_aprobado` → `pub(crate)` (T1; ya en tronco de trabajo) |
| `SddIA/engine/execute-process/src/engine/pull_request_review.rs` | `promote_ppr_verdict_after_veredicto` + helpers; aceptación F5 `executed`/`simulated`; veto F5 `blocked`/`failed`; tests +/− (argos block, documental blocked, F5 blocked, camino positivo pre-handoff) |
| `SddIA/engine/execute-process/src/engine/residual_runner.rs` | Hook post-«Veredicto y bloqueo»: **promote → maybe_emit_qa_attestation_before_handoff**; fallback terminal de atestación conservado |
| `SddIA/evolution/8cc2a86d-a2fc-49f0-b18d-dc4d92a54f4b.md` | Registro canónico |
| `SddIA/evolution/Evolution_log.md` | Índice canónico AUT-136 |

## Orden estricto (AC)

1. Fase 5 «Veredicto y bloqueo» cierra.
2. `promote_ppr_verdict_after_veredicto` escribe `state.verdict=aprobado` (y `argos_verdict=pass` si aplica).
3. `maybe_emit_qa_attestation_before_handoff` → `qa-attestation/1.0`.
4. Fase 7 Handoff ve veredicto ya propagado (evita `verdict_not_aprobado`).

## Sin cambio

- Contrato `accept-pr` / restore worktree / reconcile agent_runtime (AUT-134).
