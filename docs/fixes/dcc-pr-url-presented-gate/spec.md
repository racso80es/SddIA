---
feature_name: dcc-pr-url-presented-gate
created: "2026-11-01"
base: main
scope: pr_url obligatorio antes de PullRequest_Presented
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/dcc-pr-url-presented-gate
tracker_ref: AUT-31
execution_id: "705a9de9-556d-4f5b-945a-de5f8f8f02a5"
fracture_hash: 186c785103d8
---

# Especificación — puerta pr_url en Sello Presentación

## Corte

`capsule_delivery_emit_presented` sigue así en HEAD:

```text
state.pr_url | inputs.pr_url  →  emit-pr-presented-event
else  →  Err("pr_url obligatorio…")  →  failed  →  System_Fracture_Detected
```

`halt_after_push` solo mira **Publicación remota**. Gate pre-PR QA, sello diff y un `blocked` temprano de **Apertura en forja** dejan correr el sello. `gh pr view` ya está en la forja después de `pr create`. No está en las salidas `PR_TITLE_METACHAR`, `PR_BODY_METACHAR`, `SHELL_METACHAR` y `BRANCH_SYMBOLIC_HEAD`.

La traza del sello no casa con ninguna firma de `fracture-signatures.json`. `F-DCC-HOOK-RECURSION` tampoco la excluye.

## Decisión

1. Resolver en el sello: `state.pr_url`, luego `inputs.pr_url`. `gh pr view` solo si la forja no llegó a `gh`.
2. Con URL, `emit-pr-presented-event` no cambia de payload.
3. Sin URL, `Ok(blocked)` con `PR_URL_MISSING` y `F-DCC-PR-URL-MISSING`. El `error` cita la traza histórica. Prohibido `Err` y prohibido el ECST sin `pr_url`.
4. Firma con `fracture_policy: suppress`, entrada en `excludes` de `F-DCC-HOOK-RECURSION`, fila `186c785103d8`, y predicado de supresión en `emit_dcc_phase_fractures`.

## Regresión

| Caso | Esperado |
|------|----------|
| `state.pr_url` presente | `executed` |
| Sin URL | `blocked`, `F-DCC-PR-URL-MISSING`, sin `Err` |
| Forja `blocked` antes de `gh` y PR ya existente | el sello recupera la URL con `pr view` |
| Forja `failed` tras `pr view` vacío | el sello no repite `gh`; queda `blocked` |

`emit_presented_rejects_missing_pr_url` migra a la fila «Sin URL».

## Fuera de alcance

`F-DCC-WORKFLOW-SCOPE`. Snapshot AUT-22. AUT-26 y AUT-27. Reescritura de Gate pre-PR QA y del sello diff. La firma de título metachar se queda; este corte evita la fractura muda posterior.

## Criterios de aceptación

Los del cuerpo sellado de AUT-31: recuperación acotada, `blocked` canónico, sin ECST incompleto, firma con exclude y corpus, `validacion.md` APTO, AUT-31 en Done.
