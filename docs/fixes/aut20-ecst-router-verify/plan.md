---
feature_name: aut20-ecst-router-verify
created: "2026-11-01"
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/aut20-ecst-router-verify
tracker_ref: AUT-57
historia_ref: AUT-20
phases:
  - L0-plan
  - L1-dispatch-ecst
  - L2-ingest-sin-fichero
  - L3-verify-linear
  - L4-tests
---

# Plan — AUT-57 en `Fix_Events_Multiple`

Sin `workspace-init`. La rama ya es `Fix_Events_Multiple`. Este sello es L0. La ejecución de código es L1–L4.

## L0 — Planificación

- [x] `objectives.md`, `spec.md`, `plan.md` bajo `docs/fixes/aut20-ecst-router-verify/`
- [x] AUT-57 forjado y refinado (`refinement: refined`), hijo de AUT-20

## L1 — Despacho

- [x] Extraer el ensamblado de inputs de proceso en `route_domain_core.rs` con el contrato del spec
- [x] Sustituir el corte global `branch missing` por los cortes propios de `pull-request-review`, `telegram-fallback-responder`, `execute-suite` y SDLC
- [x] Las 15 claves del catálogo de dominio pasan por ese ensamblado, incluidas las que hoy ya llaman a `build_ecst_subscriber_inputs`

## L2 — Ingesta sin fichero

- [x] `run_memory_evolution_ingest` carga con `load_ecst_from_inputs`
- [x] El despacho de `memory-evolution-ingest` y `tracker-markdown-apply` entrega `event` y la ruta lógica

## L3 — Tracker remoto

- [x] `verify_ecst_matches_bus` al inicio de `tracker_sync_replay::run` y `tracker_markdown_apply::run`, antes de cualquier `invoke_tool_for_process`
- [x] Mismatch → `failed-bus-mismatch` y retorno sin escritura

## L4 — Pruebas

- [x] Test de tabla: 15 claves, `event` presente e igual al ECST
- [x] `route_domain_event` redb sin pending para `Tracker_Issue_Changed` y `Domain_Entity_Telemetry_Captured`
- [x] Dos tests de mismatch (replay y markdown-apply) con mock Linear sin operaciones de escritura

`cargo test -p execute-process` acotado a esos módulos. `validacion.md` queda para la fase de verificación, no para este sello.
