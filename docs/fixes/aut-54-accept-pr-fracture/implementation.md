---
feature_name: aut-54-accept-pr-fracture
created: "2026-11-02"
process: bug-fix
phase: implementation
agents: tekton
branch_name: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-54-accept-pr-fracture
tracker_ref: AUT-54
execution_id: "199ebb34-9a6a-400a-8731-b0c683c9dd11"
---

# Implementation — aut-54-accept-pr-fracture

## C1 — `diff_seal.rs`

- `persist_ref_is_lab_fixture`; filtro en fallback `validate_for_accept_pr_merge`.
- Tests: `aut54_legacy_sweep_*`, `persist_ref_is_lab_fixture_detects_slug`.

## C2 — `accept_pr.rs`

- Sin cambio (regresión AUT-53).

## C3

- `tests/fixtures/fracture-corpus/d8310429-accept-pr-seal-hygiene.json` (`expected_signature: null`).

## C4

- Deuda documentada en `validacion.md` (emit fractura / dedupe).

## C5 — `accept-pr.md` v1.0.4

- Fallback legacy excluye `_fixture-*`.
