---
feature_name: tracker-source-by-provider
created: "2026-10-25"
process: bug-fix
branch: fix/tracker-source-by-provider
persist_ref: docs/fixes/tracker-source-by-provider
tracker_ref: MAN-21
document_id: PBI-FIX-TRACKER-SOURCE-BY-PROVIDER
execution_id: "fd88acb8-0104-4101-875d-aef751fa47e6"
---

# Spec — fuente del tracker según el proveedor

## Hecho

MAN-21, hijo de MAN-20. `tracker_config_from_manifest` solo devolvía `TrackerConfig` con `provider: linear`. La ausencia del objeto y `provider: file` caían en el mismo `None`. Los handlers de ciclo abrían `pending`, `done` e `historias` o comparaban el nombre de la cápsula.

## Corte

`resolve_target` lee `tracker.provider` y devuelve una implementación:

- `linear` → la fila única `tracker:lifecycle` (`linear-tracker-adapter`). No abre el árbol de backlog.
- `file` → `file-tracker-adapter`. No instancia la cápsula remota.
- sin clave `provider` → `Unconfigured`. Los verbos de materialización (fractura, Kaizen, CI crónico) persisten por la implementación de fichero. `forge-pbi` no registra issue.

Los handlers piden `invoke` con uno de los seis verbos: `create_issue`, `list_issues`, `fetch_issue`, `update_issue_state`, `update_issue_description`, `create_comment`. El árbol de backlog lo abre la implementación de fichero. `historias/` entra en esa implementación.

## Fuera

`persist_ref`, `docs/features`, `docs/fixes`. Rebautizar `linear-tracker-adapter`. Segundo id de capacidad. Adaptador GitHub. Renombrar `tracker-linear-markdown-sync`. Reescritura del YAML de los ocho procesos.

## Criterios

| ID | Criterio |
|----|----------|
| FIX-CA1 | Con `file` el resolvedor no instancia la cápsula remota. Con `linear` no abre el árbol de backlog. |
| FIX-CA2 | Los seis verbos se ejercitan con el mismo input en las dos implementaciones. |
| FIX-CA3 | `refine`, `forge_pbi`, `tracker_backlog_query`, `tracker-description-sync`, `tracker_markdown_apply`, `tracker_stamp` y `tracker_sync_replay` no abren el árbol ni comparan el literal del provider. |
| FIX-CA4 | Fractura, alerta Kaizen y CI crónico materializan por `invoke`. |
| FIX-CA5 | Un solo id `tracker:lifecycle`. `linear-tracker-adapter` conserva el nombre. `github-raw-fetcher` sigue en `asset:fetch`. |
| FIX-CA6 | Los buckets siguen en `cumulo.paths.json` y `docs_layout`. La implementación de fichero los consulta. |
