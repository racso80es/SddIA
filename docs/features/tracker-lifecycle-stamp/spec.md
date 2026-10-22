---
feature_name: tracker-lifecycle-stamp
created: "2026-10-22"
process: feature
base: main
scope: sello por capacidad
execution_id: "1d8790f8-abfe-4880-a3a5-ec0c9093812b"
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-03
---

# Spec — tracker-lifecycle-stamp

## Proceso

`SddIA/process/tracker-stamp.md` sigue en `1.1.0` y en contexto `tracker-operations`. La fase `Sello Linear` exige `tracker:lifecycle`. El literal de la tool no está en el fichero.

## Handler

`tracker_stamp.rs` resuelve el artefacto con el resolver de la fila. `fetch_issue`, `update_issue_state` y `create_comment` salen por ese nombre.

`file_sink` lee `instance.projects`, el `manifest_ref` del slug y `tracker.provider`. Si el valor es `file`, `stamp_event` devuelve `no-op: provider file` y no llama a la cápsula.

Sin `project_slug`, el camino de laboratorio no cambia.

## Salida

El stdout de `update_issue_state` (cuerpo `emit_v2`) valida contra `tracker.lifecycle.schema.json`: `success` y `exitCode` obligatorios.
