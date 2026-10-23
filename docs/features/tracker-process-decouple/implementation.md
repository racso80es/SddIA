---
feature_name: tracker-process-decouple
process: refactorization
created: "2026-10-23"
branch_name: refactor/tracker-process-decouple
persist_ref: docs/features/tracker-process-decouple
---

# Implementación

| Pieza | Efecto |
|---|---|
| `tracker_lifecycle_port.rs` | Layout, buckets, escaneo y `manifest_provider_is_local`. |
| `tracker-description-sync.md` | Sustituye el nombre que citaba el producto. Mismo uuid. |
| `refine-hu` / `forge-pbi` | Fase `Registro en Tracker`. Un `create_issue`. |
| `materialize_kaizen_alert_doc.rs` / `materialize_ci_chronic_failure_pbi.rs` | Alta y búsqueda por el puerto. |
| `phase_capsules.rs` | El archivo compara prefijos del layout. |
| `tracker_markdown_apply.rs` | El movimiento a cerrado usa `done_dir`. |
| `tracker_stamp.rs` | La nota de no-op es `sumidero local`. La fase se llama `Sello de ciclo`. |
| `substrate` | `Local` o `Remote`. Lo leen backlog, volcado, sello, réplica, replay, refine-hu y forge-pbi. |
| Clases ECST | `tracker_ref` pasa a identificador opaco vía `entity-manager`. El nombre de la clase no cambia. |
