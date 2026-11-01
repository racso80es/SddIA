---
feature_name: dcc-pr-url-presented-gate
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/dcc-pr-url-presented-gate
tracker_ref: AUT-31
execution_id: "705a9de9-556d-4f5b-945a-de5f8f8f02a5"
phases:
  - id: T1
    title: Último pr view solo si la forja no llegó a gh
  - id: T2
    title: Sello blocked en lugar de Err
  - id: T3
    title: Firma F-DCC-PR-URL-MISSING, excludes y corpus
  - id: T4
    title: Regresión emit_presented_tests
---

# Plan — AUT-31

Rama `Fix_Events_Multiple`. Un commit por tarea en ejecución. Sin rama `fix/dcc-pr-url-presented-gate`.

Contraste con HEAD (2026-11-01): el `Err` del sello no se ha movido. `gh pr view` ya vive al final de `capsule_delivery_gh_pr`. Gate pre-PR QA y sello diff no arman `halt_after_push`.

## T1 — Resolución acotada

En `phase_capsules.rs`, helper que lee `state.pr_url` y luego `inputs.pr_url`.

`gh pr view {branch} --json url` en el sello solo si la forja no lo ejecutó. Señal: la fase **Apertura en forja** quedó `blocked` con `error_code` `PR_TITLE_METACHAR`, `PR_BODY_METACHAR`, `SHELL_METACHAR` o `BRANCH_SYMBOLIC_HEAD` (salida anterior a `gh`). Si esa fase ya devolvió `failed` tras `pr view`, el sello no repite la llamada.

Extraer el bloque view de `capsule_delivery_gh_pr` al helper para una sola implementación. El comportamiento de la forja no cambia.

## T2 — Puerta de sello

`capsule_delivery_emit_presented`: con URL, el flujo de `emit-pr-presented-event` queda igual. Sin URL, `Ok(delivery_phase_blocked("delivery-emit-pr-presented", "PR_URL_MISSING", msg))`. `friction_for_delivery_error_code` mapea `PR_URL_MISSING` a `F-DCC-PR-URL-MISSING`.

El mensaje del `blocked` incluye la traza `pr_url obligatorio para Sello Presentación ECST`. Desaparece el `Err` de ese camino.

## T3 — Firma

En `SddIA/core/fracture-signatures.json`:

- Firma `F-DCC-PR-URL-MISSING`, `fracture_policy: suppress`, alcance fase Sello Presentación y status `blocked`.
- Añadirla a `excludes` de `F-DCC-HOOK-RECURSION`.
- Fila `186c785103d8` en el corpus de regresión.

En `delivery_close.rs`, un `dcc_pr_url_missing_suppresses_fracture` junto a las demás supresiones, llamado desde `emit_dcc_phase_fractures` antes de materializar. Nota en `SddIA/evolution/` vinculada a AUT-31.

## T4 — Tests

`emit_presented_rejects_missing_pr_url` pasa a esperar `blocked` y `F-DCC-PR-URL-MISSING`, no `Err`.

Segundo caso: `state.pr_url` presente → `executed`.

`cargo test -p execute-process emit_presented --lib`.

## Hecho que este plan no repite

`inputs.pr_url` y `SDDIA_LAB_SIMULATE_GH_PR` ya escriben `state.pr_url` en la forja. `F-DCC-PR-TITLE-METACHAR` ya clasifica el preflight de título. `halt_after_push` no se amplía a las aduanas pre-push.
