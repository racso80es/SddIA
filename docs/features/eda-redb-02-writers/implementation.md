---
feature_name: eda-redb-02-writers
created: "2026-10-21"
items:
  - SddIA/sddia-event-store/src/gate.rs
  - SddIA/sddia-event-store/src/lib.rs
  - SddIA/engine/execute-process
  - SddIA/sddia-daemon-runtime
---

# Implementation — eda-redb-02-writers

`EventStore::upsert_delivery` crea o sustituye el par `(event_id, subscriber_id)` si el evento existe.

`record_ecst` inserta la familia, el ECST y una fila `pending` por clave de `delivery_state`. `append_progress_monotonic` asigna `seq` como último + 1. `copy_dlt_seal` guarda `tx_hash` y `digest` con scope `dlt-telemetry`. `load_dispatched_event` es la lectura de despacho.

El default no cambia. `cumulo.paths.json` no se edita.
