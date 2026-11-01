---
feature_name: dcc-pr-url-presented-gate
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/dcc-pr-url-presented-gate
tracker_ref: AUT-31
execution_id: "705a9de9-556d-4f5b-945a-de5f8f8f02a5"
global: APTO
pbi_archived: false
branch: Fix_Events_Multiple
approval_status: aprobado
verdict: aprobado
validated_base_sha: 0fe568c28437a6d28315ae8f4c5cec1e2d6b7f41
checks:
  AUT31-T1-RESOLUCION-ACOTADA: APTO
  AUT31-T2-SELLO-BLOCKED: APTO
  AUT31-T3-FIRMA-SUPPRESS: APTO
  AUT31-T4-REGRESSION: APTO
git_changes:
  - SddIA/core/fracture-signatures.json
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/evolution/e31a186c-7851-4d8b-a9c0-31f0dc01a931.md
  - docs/fixes/dcc-pr-url-presented-gate/
  - docs/audits/AUT-31.md
---

# Validación — AUT-31

## Veredicto

**APTO** — El Sello Presentación ECST ya no fractura con `Err` por `pr_url` ausente: envelope `blocked` + `F-DCC-PR-URL-MISSING`, recuperación acotada (`state`/`inputs`, `gh pr view` solo tras forja blocked pre-`gh`), supresión DCC alineada con firma.

## Evidencia

| Check | Medio | Resultado |
|-------|--------|-----------|
| T1 | Flags `dcc_forja_*`, tests `presented_seal_skips_gh_view_*` | OK |
| T2 | `emit_presented_blocks_missing_pr_url` → `Ok(blocked)` | OK |
| T3 | `dcc_fracture_suppressed_on_sello_pr_url_missing`, exclude hook, `fracture_corpus_regression` | OK |
| T4 | `cargo test -p execute-process emit_presented --lib` | 5/5 OK |

## Cierre tracker

Provider `linear`: **AUT-31** → Done con el PR que incluya este diff; sin fichero en `docs/todos/done/`.
