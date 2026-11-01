---
feature_name: dcc-pr-url-presented-gate
created: "2026-11-01"
purpose: decisiones de alcance AUT-31
process: bug-fix
branch_name: Fix_Events_Multiple
tracker_ref: AUT-31
execution_id: "705a9de9-556d-4f5b-945a-de5f8f8f02a5"
---

# Clarificación — AUT-31

## Decisiones

1. El ECST de presentación exige `pr_url`. El cambio es el modo de fallo y un `gh pr view` solo cuando la forja no lo ejecutó.
2. Gate pre-PR QA y sello diff no cierran el hueco: no arman `halt_after_push`.
3. Sin URL, `blocked` + `F-DCC-PR-URL-MISSING`. La firma entra en `excludes` de `F-DCC-HOOK-RECURSION`.
4. Sin flag extra `forja_without_pr_url`. La señal es el `error_code` ya escrito por la forja.
5. Rama: `Fix_Events_Multiple`.
