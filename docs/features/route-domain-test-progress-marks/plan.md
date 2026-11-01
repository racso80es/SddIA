---
feature_name: route-domain-test-progress-marks
created: "2026-11-01"
process: feature
branch_name: Fix_Events_Multiple
persist_ref: docs/features/route-domain-test-progress-marks
tracker_ref: AUT-59
historia_ref: AUT-20
phases:
  - L0-plan
  - L1-helper
  - L2-call-sites
---

# Plan — AUT-59 en `Fix_Events_Multiple`

Sin `workspace-init`. La rama ya es `Fix_Events_Multiple`. Este sello es L0.

## L0 — Planificación

- [x] AUT-59 forjado y refinado (`refinement: refined`, tipo kaizen), hijo de AUT-20
- [x] `objectives.md`, `spec.md`, `plan.md` bajo `docs/features/route-domain-test-progress-marks/`

## L1 — Helper

- [x] Función de marca en `tests/eda_subscriber_ecst_redb.rs`: stderr con flush y append a `.tmp/route-domain-test-progress.log` del checkout

## L2 — Call sites

- [x] `enter` / `leave` en los cinco tests `route_domain_*` listados en el spec
- [x] Sin diff en `route_domain_core.rs`

`validacion.md` queda para la verificación. La prueba de la métrica es una corrida con `--nocapture --test-threads=1` y el fichero de log, no un test que afirme el texto del log.
