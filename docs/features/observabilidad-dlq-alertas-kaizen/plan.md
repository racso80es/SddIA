---
id: aut-117-plan
tracker_ref: AUT-117
process: feature
persist_ref: docs/features/observabilidad-dlq-alertas-kaizen
---

# Plan de Implementación — AUT-117

1. Crear `SddIA/engine/execute-process/src/engine/dlq_kaizen_alert.rs` con contrato `DlqAlertSink`, `ProdDlqAlertSink`, anti-recursión, deduplicación por firma y 3 tests unitarios herméticos.
2. Exponer `pub mod dlq_kaizen_alert;` en `SddIA/engine/execute-process/src/engine/mod.rs` y conectarlo en la transición a DLQ de `route_domain_core.rs`.
3. Auditar y remediar silencios `let _ =` en `SddIA/daemons/event-watcher/src/main.rs` y `SddIA/sddia-daemon-runtime/src/eda_sweep.rs`, documentando `tracker_stamp.rs`.
4. Extender `needs_kaizen_actionable` e `implicated` en `SddIA/tools/event-bus-audit/src/main.rs` con tests unitarios para firmas DLQ activas vs históricas.
5. Ejecutar `cargo check --workspace` (`-Dwarnings`), tests unitarios y gates `sddia-qa`, registrar entrada en `SddIA/evolution/` y cerrar ciclo hacia `main`.
