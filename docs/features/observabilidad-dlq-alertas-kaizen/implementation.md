---
id: aut-117-implementation
tracker_ref: AUT-117
process: feature
persist_ref: docs/features/observabilidad-dlq-alertas-kaizen
---

# Registro de Implementación — AUT-117

## Archivos Modificados / Creados
- `SddIA/engine/execute-process/src/engine/dlq_kaizen_alert.rs` (nuevo): deduplicación por firma (`dlq_signature_hash`), trait `DlqAlertSink`, `ProdDlqAlertSink`, anti-recursión `Kaizen_Alert_Required` y 3 tests herméticos.
- `SddIA/engine/execute-process/src/engine/mod.rs`: exporta `pub mod dlq_kaizen_alert;`.
- `SddIA/engine/execute-process/src/engine/route_domain_core.rs`: conecta `notify_dlq_failure_fail_soft` al enviar un evento a `dead_lettered`.
- `SddIA/daemons/event-watcher/src/main.rs`: reemplaza `let _ = with_productive_port(..., "record_delivery", ...)` por trazado `eprintln!("[WATCHER] record_delivery error ...")`.
- `SddIA/sddia-daemon-runtime/src/eda_sweep.rs`: reemplaza 4 `let _ = port.purge_event(...)` / `let _ = port.move_to_dlq(...)` en `sweep_fractal_family`, `finalize_kaizen_terminal` y `archive_event_after_sweep` por trazado `[EDA-SWEEP] ... failed`.
- `SddIA/tools/event-bus-audit/src/main.rs`: actualiza `needs_kaizen_actionable(..., active_dlq_signatures)`, puebla `implicated` con `dlq:<err_key>` cuando `active_dlq_signatures > 0`, y añade `aut117_active_dlq_signatures_trigger_needs_kaizen`.

## Inventario de Puntos Silenciosos Auditados (AC-5)
| Componente | Punto auditado | Estado previo | Acción |
| :--- | :--- | :--- | :--- |
| `event-watcher/src/main.rs` | `record_delivery` (`L469`) | `let _ = with_productive_port(...)` | Remediado con `eprintln!` |
| `eda_sweep.rs` | `sweep_fractal_family` (`L111, L119`) | `let _ = port.purge_event/move_to_dlq` | Remediado con `eprintln!` |
| `eda_sweep.rs` | `finalize_kaizen_terminal` / `archive_event_after_sweep` | `let _ = port.move_to_dlq/purge_event` | Remediado con `eprintln!` |
| `tracker_stamp.rs` | `no-op: sin tracker_ref` / `emit-tracker-sync-failed` | Retorno `Some("no-op: ...")` explícito y emisión fail-soft | Conservado (trazado por caller) |
