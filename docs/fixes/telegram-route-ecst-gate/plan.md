---
feature_name: telegram-route-ecst-gate
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/telegram-route-ecst-gate
tracker_ref: AUT-60
historia_ref: AUT-20
phases:
  - L0-plan
  - L1-catalog-copy
  - L2-fixtures-asserts
---

# Plan — AUT-60 en `Fix_Events_Multiple`

Sin `workspace-init`. La rama ya es `Fix_Events_Multiple`. Este sello es L0.

## L0 — Planificación

- [x] AUT-60 forjado y refinado (`refinement: refined`), hijo de AUT-20
- [x] `objectives.md`, `spec.md`, `plan.md` bajo `docs/fixes/telegram-route-ecst-gate/`

## L1 — Catálogo en el repo temporal

- [x] `temp_repo_with_bus_skeleton` copia `SddIA/events/` del checkout

## L2 — Fixtures y aserciones

- [x] Ambos `route_domain_telegram_*` envían `chat_id` y `source`; el vacío conserva `text` en blanco
- [x] El test de texto vacío exige `skipped-empty-text` y rechaza el corte `ecst-gate`
- [x] El test de texto presente rechaza un `delivery_status` que sea solo `ecst-gate: failed`

Verificación: `cargo test -p execute-process --test eda_subscriber_ecst_redb route_domain_telegram -- --nocapture --test-threads=1` desde `SddIA/`. `validacion.md` queda para esa corrida, no para este sello.
