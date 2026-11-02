---

feature_name: aut-52-refine-fracture-route
created: "2026-11-02"
process: bug-fix
phase: implementation
agents: tekton
branch_name: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-52-refine-fracture-route
pbi_ref: AUT-52
tracker_ref: AUT-52
uuid: c55f04de-f956-4f1e-89ab-580b2c5bab34
execution_id: "f2880068-385c-4403-8c31-9d11bd4b08a1"
---
# Implementation — aut-52-refine-fracture-route

## C1 — `refine_item/run.rs`

- `emit_store_lock_fracture`: añade `friction_id: F-REFINE-ITEM-REDB-LOCK` al payload de `System_Fracture_Detected`.

## C2 — `route_domain_core.rs`

- Helpers: `pending_virtual_event_path`, `is_runtime_db_event_path`, `store_lock_error_msg`, `find_fracture_stored_event_id`.
- `write_pending_domain_event_file`: no devuelve `runtime_db_path`; sidechannel JSON atómico en lock al emitir fractura; path virtual en redb-only.
- `route_domain_event`: error si la ruta resuelta es el DB runtime.
- `route_domain_batch`: carga desde store cuando el path virtual no es fichero; rechazo `.redb`; `batch_errors` + `success: false`.
- Tests: `aut52_runtime_db_path_is_detected`, `aut52_route_domain_batch_rejects_redb_path`.

## C3 — `capsules.rs`

- `tool_invoke_error_message`: concatena `code` y `message` (excepto auth-only).
- Test `aut52_tool_invoke_error_message_includes_linear_transport_detail`.

## C4 — `SddIA/core/fracture-signatures.json`

- Firma `F-REFINE-ITEM-REDB-LOCK` con scope `refine-item` / `STORE_LOCK_CONTENTION`.

## C5 — `materialize_fracture_pbi.rs`

- Sin diff: camino nativo ya usa `build_pbi_body` y labels `[pbi, fix]` (`run_remote`).

## Evolution

- `SddIA/evolution/c55f04de-f956-4f1e-89ab-580b2c5bab34.md` + fila en `Evolution_log.md`.
- Corpus `tests/fixtures/fracture-corpus/684ebbf7-refine-route-redb.json`.

## Estado código

Entregado en commit `49e02b5c` (esta pasada alinea doc post re-refino `ca843b5c`).
