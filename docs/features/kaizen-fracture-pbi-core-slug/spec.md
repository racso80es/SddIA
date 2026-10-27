---
feature_name: kaizen-fracture-pbi-core-slug
created: "2026-10-27"
process: refactorization
---

# Spec — materialize-fracture-pbi y slug core por defecto

## Invariante de sumidero

| Condición | Sumidero |
|-----------|----------|
| `project_slug` en inputs | Manifiesto del slug (actual). |
| Sin slug y existe `.SddIA/projects/core.md` | Manifiesto `core`. |
| Sin slug y sin índice `core` | Fichero (`file_tracker_adapter`). |
| `provider: linear` | Sin escritura en `paths.todos.pending` / `done`. Dedupe: `find_issue_with_marker(trace_hash)`. |
| `provider: file` | Ledger `scan_fracture_ledger` sin cambios. |

## Cambios en `materialize_fracture_pbi::run`

1. `effective_inputs`: clonar inputs; si falta `project_slug` y `core.md` existe → `project_slug: "core"`.
2. Resolver `linear: Option<TrackerConfig>` **antes** del `match resolution.reason`.
3. Si `linear.is_some()`: ignorar `AlreadyOpen` / `DedupedByProcess` del disco; usar idempotencia remota; cuerpo = `build_pbi_body`; título con hash para marker.
4. Si `linear.is_none()`: flujo actual.

## No tocar

- `project_binding::bind` (sigue `None` sin slug).
- `LifecycleTarget::Unconfigured` global en `tracker_lifecycle_port`.

## Referencias

- `SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs`
- `SddIA/engine/execute-process/src/engine/delivery_close.rs` (`emit_dcc_phase_fractures`)
- `SddIA/engine/execute-process/src/engine/route_domain_core.rs` (`emit_dlt_batch_fracture`)
- HU tracker lifecycle §3.7 (`todos-jurisdiction`)
