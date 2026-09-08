---
feature_name: dcc-gh-api-connect-49ce2db7152d
created: "2026-09-08"
process: bug-fix
branch: fix/dcc-gh-api-connect-49ce2db7152d
branch_name: fix/dcc-gh-api-connect-49ce2db7152d
persist_ref: docs/fixes/dcc-gh-api-connect-49ce2db7152d
pbi_ref: docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (49ce2db7152d).md
document_id: PBI-FIX-FRACTURE-49ce2db7152d
execution_id: "0165ad09-f245-4465-aa03-280770a4ac93"
global: APTO
pbi_archived: true
pr_url: "https://github.com/racso80es/SddIA/pull/272"
ci_run_id: "34228075656"
ci_run_url: "https://github.com/racso80es/SddIA/actions/runs/34228075656"
checks:
  CA-1: APTO
  CA-2: APTO
  CA-3: APTO
  CA-4: APTO
  CA-5: APTO
  CA-6: APTO
  CA-CI: APTO
git_changes:
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - docs/fixes/dcc-gh-api-connect-49ce2db7152d/
  - docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (49ce2db7152d).md
  - SddIA/evolution/0b3ef02b-debe-4532-91da-c265ed59942b.md
  - SddIA/evolution/Evolution_log.md
---

# Validación — fractura `49ce2db7152d`

## Veredicto

**APTO** — token `gh` `error connecting to api.github.com` entra en F4c; Apertura en forja sella `F-DCC-DNS-UNRESOLVED` / `blocked`; `pr_url` opaco sigue emitiendo.

## Checks

| Check | Estado | Evidencia |
|-------|--------|-----------|
| CA-1 | APTO | `dcc_fracture_suppressed_on_forge_gh_api_connect` |
| CA-2 | APTO | `stamp_dcc_network_block_gh_api_connect`; DCC reinyección `blocked` + `F-DCC-DNS-UNRESOLVED` |
| CA-3 | APTO | `dcc_fracture_emits_on_failed_forge_phase` |
| CA-4 | APTO | `dcc_transient_network_trace_positives_and_pr_url_negative` |
| CA-5 | APTO | 15 tests `cargo test -p execute-process --lib -- dcc_transient dcc_fracture stamp_dcc_network` |
| CA-6 | APTO | cascada + PBI `done/` en esta rama |
| CA-CI | APTO | run [34228075656](https://github.com/racso80es/SddIA/actions/runs/34228075656) `206e072` SUCCESS |
