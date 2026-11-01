---
feature_name: dcc-pr-url-presented-gate
created: "2026-11-01"
process: bug-fix
tracker_ref: AUT-31
execution_id: "705a9de9-556d-4f5b-945a-de5f8f8f02a5"
---

# Implementación — AUT-31

## Motor (`phase_capsules.rs`)

- `pr_url_from_state_or_inputs`, `try_gh_pr_view_url`, `resolve_pr_url_for_presented_seal`.
- Marcadores en `state`: `dcc_forja_blocked_before_gh` (preflight blocked en forja), `dcc_forja_gh_attempted` (tras pasar argv-safe, antes de `gh pr create`).
- **Forja:** `try_gh_pr_view_url` solo tras create si stdout/stderr no traen URL.
- **Sello:** preset → view solo con `blocked_before_gh` y sin `gh_attempted`; si no hay URL → `Ok(blocked)` `PR_URL_MISSING` / `F-DCC-PR-URL-MISSING` (sin `Err`).

## Firma y DCC

- `F-DCC-PR-URL-MISSING` en `fracture-signatures.json`; exclude en `F-DCC-HOOK-RECURSION`.
- `dcc_pr_url_missing_block_suppresses_fracture` en `emit_dcc_phase_fractures`.
- Corpus `186c785103d8` en `enrich_fracture_pbi_kaizen.rs`.

## Evolución

- `SddIA/evolution/e31a186c-7851-4d8b-a9c0-31f0dc01a931.md`.
