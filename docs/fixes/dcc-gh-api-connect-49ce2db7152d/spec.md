---
feature_name: dcc-gh-api-connect-49ce2db7152d
created: "2026-09-08"
process: bug-fix
base: main
scope: dcc-f4c-gh-api-connect
branch_name: fix/dcc-gh-api-connect-49ce2db7152d
persist_ref: docs/fixes/dcc-gh-api-connect-49ce2db7152d
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (49ce2db7152d).md
document_id: PBI-FIX-FRACTURE-49ce2db7152d
execution_id: "0165ad09-f245-4465-aa03-280770a4ac93"
---

# Especificación — fractura `49ce2db7152d` (gh API connect ≠ colapso)

## Problema

`delivery-close-cycle` fase **Apertura en forja** abortó con:

```
no se pudo resolver pr_url desde gh; gh_stdout=; gh_stderr=error connecting to api.github.com
check your internet connection or https://githubstatus.com
; view_stdout=; view_stderr=no pull requests found for branch "feat/nucleo-aiua-tormentosa-motor"
```

El abort de `gh` es **correcto**. Defecto: F4c (`dcc_transient_network_trace`) no reconoce el token del CLI `gh`. La traza se trata como fallo ontológico → `System_Fracture_Detected` / Kintsugi. Reinyección DCC (laudo) abrió el PR.

F4c vigente (`d0cfd5b66ff1`) cubre DNS/timeout/unreachable. CA-4 de aquel fix exige que `no se pudo resolver pr_url desde gh` **sin** token de red **siga** emitiendo fractura.

## Cambio requerido

Motor: `SddIA/engine/execute-process/src/engine/delivery_close.rs`. **No** genoma `delivery-close-cycle.md`. **No** retry/backoff (DA-5). **No** `gh`/`git` raw.

### Predicado F4c — tokens añadidos (case-insensitive)

| Token | Origen |
|-------|--------|
| `error connecting to api.github.com` | `gh` CLI (stderr) |
| `check your internet connection or https://githubstatus.com` | `gh` CLI (línea hermana) |

Tokens F4c previos intactos.

### Sello

Misma receta F4c: `friction_id: F-DCC-DNS-UNRESOLVED`, status `blocked`, envelope DCC `success: false`, sin `fail_soft`. Aplicar también cuando **Apertura en forja** retorna `Ok(failed)` (handler no va por `Err`).

### CA-4 intacto

`error: "no se pudo resolver pr_url desde gh"` **sin** tokens de red → sigue emitiendo `System_Fracture_Detected`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA-1 | Traza PBI (pr_url + `error connecting to api.github.com`) en Apertura en forja **no** materializa `System_Fracture_Detected` |
| CA-2 | Mismo caso sella `friction_id: F-DCC-DNS-UNRESOLVED` y status `blocked` |
| CA-3 | `no se pudo resolver pr_url desde gh` opaco **sí** emite (regresión CA-4 F4c) |
| CA-4 | Tokens DNS previos siguen positivos |
| CA-5 | Tests unitarios `execute-process` |
| CA-6 | Cascada documental + PBI en `done/` en el mismo PR; CA-CI con `run_id` verde |

## Fuera de alcance

- Resolver conectividad del host.
- Retry/polling de `gh`.
- Mutar genoma DCC.
- PBI `41717b4bb229` (`route-domain-event` / IOTA).
- Bypass raw.
