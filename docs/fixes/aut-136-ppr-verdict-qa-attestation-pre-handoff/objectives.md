---
feature_name: aut-136-ppr-verdict-qa-attestation-pre-handoff
created: "2026-11-27"
process: bug-fix
branch_name: fix/aut-136-ppr-verdict-qa-attestation-pre-handoff
persist_ref: /home/racso/Proyectos/SddIA/docs/fixes/aut-136-ppr-verdict-qa-attestation-pre-handoff
pbi_ref: AUT-136
execution_id: "f3c98fff-6b95-4122-85b8-7c64463f3f5d"
---

# Objetivos — aut-136-ppr-verdict-qa-attestation-pre-handoff

## Misión

AUT-136: pull-request-review debe propagar state.verdict=aprobado y qa-attestation/1.0 antes de Fase 7 Handoff materialización cuando Fase 5 Veredicto y bloqueo ejecuta sin bloqueos previos. Causa: promote ausente entre F5 y maybe_emit_qa_attestation_before_handoff. Superficies: pull_request_review.rs promote_ppr_verdict_after_veredicto, residual_runner.rs hook post-Veredicto, qa_attestation.rs pp

## Alcance (manifiesto)

Inicialización de contexto vía orquestador nativo `execute-process` (laboratorio).

## Ley aplicada

- Git exclusivamente vía `skill:git-manager`.
- Jerarquía: Acción → Agente → Skill → Tools.
