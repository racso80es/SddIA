---

feature_name: man-45-accept-pr-ppr-tree-sha
document: ka-ppr-5-counterfactual
pbi_ref: MAN-45
kaizen_ref: KZ-2
audit_ref: docs/audits/man-43-h1-milestone-razonamiento-20261111.md
created: "2026-11-20"
execution_id: "52da40f9-4dde-44a6-88d6-ce6efb0a1ec8"
---
# KA-PPR-5 — Contrafactual KZ-2 (AC-5)

Origen: auditoría `docs/audits/man-43-h1-milestone-razonamiento-20261111.md` (KZ-2). En el cierre del epílogo de **MAN-43** (merge histórico PR #453, **sin revertir**):

1. `pull-request-review` full quedó `blocked`/detached — no había atestación `qa-attestation/1.0` `aprobado` usable del `tree_sha` fusionado.
2. El push usó atestación `docs-only` o JSON `full` escrito fuera de `try_write_from_ppr` (sin sello de emisor canónico).
3. La Auditoría Genómica de `accept-pr` solo comprobó `PullRequest_Presented` y trató `skipped-ppr-dedupe` como crédito de calidad.

Con la aduana activa (`validate_for_accept_pr_with_state` en «Auditoría Genómica»):

- Ausencia / no-aprobado ⇒ `qa_attestation_required`
- `docs-only` + delta `active` ⇒ `qa_attestation_profile_blocked`
- JSON `full` sin `issuer_process: pull-request-review` ⇒ `qa_attestation_issuer_invalid`
- Solo `skipped-ppr-dedupe` ⇒ `qa_attestation_dedupe_not_approval`

Cualquiera de esas condiciones habría **abortado** la fusión del PR #453 **antes** de «Fusión Soberana», sin emitir `System_Fracture_Detected`. Este documento es métrica contrafactual (AC-5); no ordena revert histórico.
