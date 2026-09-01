---
feature_name: dcc-pr-title-metachar-451dc8707819
created: "2026-09-01"
process: bug-fix
base: main
scope: dcc-pr-title-metachar-mayeuta
branch_name: fix/dcc-pr-title-metachar-451dc8707819
persist_ref: docs/fixes/dcc-pr-title-metachar-451dc8707819
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (451dc8707819).md
document_id: PBI-FIX-FRACTURE-451dc8707819
uuid: "bc16d090-2f7c-4845-8134-032989b094dc"
execution_id: "3326bf22-765a-4305-8fdf-a200b23cad10"
---

# Especificación — fractura `451dc8707819` (`pr_title` en argv)

## Problema

`delivery-close-cycle` fase **Apertura en forja** abortó con:

```
[PR_BODY_METACHAR] arguments[3] contains forbidden shell metacharacters
```

`arguments[3]` es `--title` / `pr_title`, no el body. K2 (`--body-file`, PR #129) ya desvía markdown multilínea al fichero.

| ID | Defecto |
|----|---------|
| F1 | `pr_title` (y head/base) van a `shell-executor` sin preflight; `>` GitHub-legal rompe argv |
| F2 | `classify_delivery_error` etiqueta cualquier metacaracter como `PR_BODY_METACHAR`; fractura sella `F-DCC-APERTURA-EN-FORJA` |
| F3 | Mayeuta no tiene cubo; catch-all «Auditar proceso» |

La guarda `SDDIA_HOOK_DELIVERY_CLOSE` **ya existe**. Prohibido reimplementarla. Prohibido relajar `assert_safe_token`.

## Reproducción (especimen)

1.ª DCC de `feat/kaizen-ci-step-runtime-gt-1min` (2026-09-01): `pr_title` = `feat: kaizen CI — steps >1 min (cache integrity + ingest itest)`. Hash SHA-256 12 hex = `451dc8707819`. 2.ª DCC con título saneado → PR #246. Residual fuera de ese PR.

## Cambio requerido

Motor `execute-process`. No mutar genoma `delivery-close-cycle.md` ni `shell-executor`. `gh` no tiene `--title-file`.

### F1 — Saneo de título + preflight argv

`phase_capsules.rs`:

```text
fn sanitize_shell_argv_token(token: &str) -> String
```

Sustituciones deterministas (paridad `is_shell_token_safe`), orden: `$(` → `(`, `&&` → ` and `, `&` → ` and `, `>` → `gt`, `<` → `lt`, `|` → `/`, `;` → `,`, `\n`/`\r` → espacio, backtick → `'`. Colapsar espacios; trim.

`capsule_delivery_gh_pr`:

1. Título = `sanitize_shell_argv_token(delivery_pr_title(inputs))`. Si queda vacío o sigue inseguro → `blocked` `PR_TITLE_METACHAR` + `friction_id: F-DCC-PR-TITLE-METACHAR` + campo `pr_title` + índice `3`.
2. `branch_name` / `target_branch`: **no** sanear (refs). Si inseguros → `blocked` `SHELL_METACHAR` + campo + índice.
3. Path `--body-file`: sigue `PR_BODY_METACHAR` (K2).
4. Preflight de **todos** los tokens argv antes de `invoke_shell_executor`.
5. Envelope: `pr_title_original` / `pr_title` si hubo saneo.

Specimen con `>`: Apertura **executed** (título argv sin `>`). No `arguments[3]`.

### F2 — Códigos distintos + friction

`classify_delivery_error`:

| Traza | `error_code` |
|-------|----------------|
| `path --body-file` o `arguments[9]` | `PR_BODY_METACHAR` |
| `arguments[3]` o `pr_title` | `PR_TITLE_METACHAR` |
| otro `forbidden shell metacharacters` | `SHELL_METACHAR` |

`delivery_phase_failed` sella `friction_id`:

| `error_code` | `friction_id` |
|--------------|---------------|
| `PR_TITLE_METACHAR` | `F-DCC-PR-TITLE-METACHAR` |
| `PR_BODY_METACHAR` | `F-DCC-PR-BODY-METACHAR` |
| `SHELL_METACHAR` | `F-DCC-SHELL-METACHAR` |

`emit_dcc_phase_fractures` ya prefiere `report.friction_id`. Test `arguments[9]` permanece `PR_BODY_METACHAR`.

### F3 — Cubo Mayeuta

`analyze_fracture_kaizen`, match **solo** `error_trace`: `PR_TITLE_METACHAR` / `PR_BODY_METACHAR` / `SHELL_METACHAR` / (`forbidden shell metacharacters` + `arguments[`). Veredicto `process_fix`. Texto: argv title/head/base; no recursión hook; no reabrir K2; no «Auditar proceso». Traza specimen ≠ hook.

### F4 — No Kintsugi tras preflight residual

Si Apertura queda `blocked`/`failed` con `PR_TITLE_METACHAR` / `F-DCC-PR-TITLE-METACHAR`: **no** emitir `System_Fracture_Detected` (paridad F4c). El specimen original sí debía emitir (deuda); el runtime post-fix no re-materializa PBI por título inseguro ya clasificado.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | `sanitize_shell_argv_token("feat: kaizen CI — steps >1 min (x)")` no contiene `>`; `is_shell_token_safe` |
| CA2 | `classify_delivery_error("arguments[3]…")` → `PR_TITLE_METACHAR`; `arguments[9]` → `PR_BODY_METACHAR` |
| CA3 | `delivery_phase_failed(..., "PR_TITLE_METACHAR", …)` incluye `friction_id: F-DCC-PR-TITLE-METACHAR` |
| CA4 | Mayeuta: traza `[PR_BODY_METACHAR] arguments[3]…` → `process_fix`; sin «Recursión»; sin «Auditar proceso» |
| CA5 | Hook recursion test vigente sigue verde |
| CA6 | `emit_dcc_phase_fractures` Apertura + `PR_TITLE_METACHAR` → pending vacío; `no se pudo resolver pr_url` sigue emitiendo |
| CA7 | `cargo test -p execute-process` filtros `classify_delivery` / `sanitize_shell` / `analyze_fracture_kaizen` / `dcc_fracture` |
| CA8 | Cascada spec/plan/implementation/execution/validacion + PBI en `done/` |

## Fuera de alcance

Allowlist `shell-executor`. K2 body-file. Producto CI PR #246. Snapshot untracked todos. `SDDIA_HOOK_DELIVERY_CLOSE`. Telemetría `persist_ref` en payload de fractura.
