---
feature_name: tracker-process-decouple
process: refactorization
created: "2026-10-23"
branch_name: refactor/tracker-process-decouple
persist_ref: docs/features/tracker-process-decouple
---

# Ejecución

`cargo test -p execute-process --lib` sobre `tracker_lifecycle_port`, `handlers::tracker_stamp`, `handlers::tracker_sync_replay`, `handlers::tracker_markdown_apply`, `handlers::tracker_backlog_query` y `handlers::forge_pbi`: 29 en verde. `handlers::refine` (4) en verde en una segunda pasada; la primera coincidió con el candado de `events.redb`.

Sello, réplica y replay leen `substrate()`. Ninguno de los ocho handlers llama ya a `tracker_config_for_inputs`.

`entity-manager` selló la glosa opaca de `tracker_ref` en `pull-request-presented`, `pull-request-merged`, `pull-request-audited`, `work-initiated`, `tracker-issue-changed` y `tracker-sync-failed`. El primer sello de `tracker-issue-changed` chocó con el candado de `events.redb`; el reintento acusó `Domain_Entity_Updated`.

El trabajo está en `refactor/tracker-process-decouple` y no tiene commit. `validacion.md` sigue en el `NO_APTO` anterior a este corte.
