---
feature_name: aut20-ecst-residual-dispatch
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/aut20-ecst-residual-dispatch
tracker_ref: AUT-58
historia_ref: AUT-20
phases:
  - L0-plan
  - L1-ensamblado
  - L2-telegram-sdlc
  - L3-tests
---

# Plan — AUT-58 en `Fix_Events_Multiple`

Sin `workspace-init`. La rama ya es `Fix_Events_Multiple`. Este sello es L0. El código es L1–L3.

## L0 — Planificación

- [x] AUT-58 forjado y refinado (`refinement: refined`), hijo de AUT-20
- [x] `objectives.md`, `spec.md`, `plan.md` bajo `docs/fixes/aut20-ecst-residual-dispatch/`

## L1 — Ensamblado

- [x] Función única de inputs por clave de proceso en `route_domain_core.rs`
- [x] `dispatch_subscriber` deja de armar mapas ad hoc en las ramas que invocan proceso

## L2 — Ramas residuales

- [x] `telegram-fallback-responder` fusiona el ECST cuando hay texto
- [x] `sdlc_process_request_inputs` (o el llamador) fusiona el ECST en el camino de éxito de `task-queue-manager`
- [x] Cortes propios de telegram, SDLC, `execute-suite` y `pull-request-review` sin cambio de motivo

## L3 — Pruebas

- [x] Test de las 15 claves contra la función de ensamblado, con la clave como argumento
- [x] `route_domain_event` redb sin pending: `TelegramMessage_Received` y `Kalma2_Process_Requested`
- [x] Regresión de texto vacío y de proceso SDLC no permitido

`cargo test -p execute-process` acotado a ese módulo y a `eda_subscriber_ecst_redb`. `validacion.md` queda para la verificación, no para este sello.
