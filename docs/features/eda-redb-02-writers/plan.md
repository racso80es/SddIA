---
feature_name: eda-redb-02-writers
created: "2026-10-21"
phases:
  - compuerta en el crate
  - escritores del orquestador
  - escritores del daemon
  - tests AC-1, AC-3, CA-04, CA-05
---

# Plan — eda-redb-02-writers

1. `upsert_delivery` y `gate.rs` en `sddia-event-store`.
2. Dependencia de path en `execute-process` y `sddia-daemon-runtime`.
3. Store primero, fichero solo si el modo lo pide.
4. Tests de crate, de materialize y del side-channel de heartbeat.
5. Registro de evolución con las rutas materiales, incluido `Cargo.lock`.
