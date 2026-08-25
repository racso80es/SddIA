---
feature_name: kaizen-paciente0-redeploy-fricciones
created: "2026-08-25"
process: feature
branch: feat/kaizen-paciente0-redeploy-fricciones
global: APTO
pbi_archived: true
persist_ref: docs/features/kaizen-paciente0-redeploy-fricciones
document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824
uuid: "56aff1d3-d5f6-4502-9b5b-e5a57dc718e3"
execution_id: "c95fa63f-be71-481b-a927-475e7c885fd0"
checks:
  O1_bundle_cicatriz: APTO
  O2_systemd_instance_root: APTO
  O3_starter_paths: APTO
  O4_start_sddia_bundle: APTO
  O5_route_domain: APTO_COND
  O6_O7_subject_elevation: APTO
  O8_infer_tokens: APTO
  O9_g5_wui_agenda: APTO
  O10_audit: APTO
  O11_pr: EN_CIERRE
git_changes:
  - SddIA/scripts/build-release-bundle.sh
  - start-sddia.sh
  - SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs
  - SddIA/engine/execute-process/src/engine/handlers/email_triage.rs
  - SddIA/process/instance-creator.md
  - SddIA/process/index.md
  - SddIA/norms/sddia-distribution-protocol.md
  - SddIA/library/norms/email-triage-matrix.md
  - SddIA/library/norms/index.md
  - SddIA/core/eda-coverage.json
  - docs/features/kaizen-paciente0-redeploy-fricciones/
  - docs/audits/kaizen-paciente0-redeploy-20260825.md
  - docs/todos/done/[KAIZEN] Paciente 0 SddIA_AP — redeploy y fricciones operativas.md
  - SddIA/evolution/916bf0f9-05ea-4408-8b6e-294e7efcc5f9.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — kaizen-paciente0-redeploy-fricciones

## Veredicto

**APTO** para merge de código y gates de instancia. **O11** se cierra con `delivery-close-cycle` en este estímulo.

## Checks

| ID | Juicio | Evidencia |
|----|--------|-----------|
| O1 | APTO | Bundle 20260825T111733Z; skip-build fail-closed luego OK; 0 `.rs`; sin `execute-process.py` |
| O2 | APTO | `ExecStart` bajo `SddIA_AP/SddIA/` post-creator, sin sed operador |
| O3 | APTO | `local.paths.json` no `{}` |
| O4 | APTO | Ignición instancia: 0 `cargo build` |
| O5 | APTO_COND | `Email_Triaged` `413e6edf-…` enrutado `route-domain` (purgado). Probe clase no ECST → gate fail (no es centinela) |
| O6/O7 | APTO | Unit `llm_passive_meeting_subject_elevates_to_actionable` + G5 `subject_elevation: true` |
| O8 | APTO | T-INFER: `mark_classification_degraded` (tokens 0 + asunto sin fecha → degraded; elevación o tokens >0 → no). G5 tokens 0 cubierto por guard |
| O9 | APTO | Agenda + WUI inbox. Telegram: `send-telegram-notification` instancia `message_id=9` |
| O10 | APTO | `docs/audits/kaizen-paciente0-redeploy-20260825.md` |
| O11 | EN_CIERRE | `delivery-close-cycle` este estímulo |

## Residual Argos

- IMAP First Blood real: spec §5 no muta buzón; G5 sintético es el contrato de handler + WUI.
- DLQ Telegram previo (`Dns Failed` ~10:51Z) es red del host, no el handler. Poke posterior `message_id=9`.
- No incluir `dist/` ni instancia `SddIA_AP` en el PR.
