---
id: aut-117-spec
tracker_ref: AUT-117
process: feature
persist_ref: docs/features/observabilidad-dlq-alertas-kaizen
---

# Especificación Técnica — AUT-117

## 1. Módulo `dlq_kaizen_alert` (`execute-process`)
- `dlq_signature_hash(error_fingerprint: &str, subscriber_id: &str, event_type: &str) -> String`: hash determinista hexadecimal.
- `DlqAlertSink`:
  - `has_active_signature(&self, sig: &str) -> Result<bool, String>`
  - `record_active_signature(&mut self, sig: &str, event_id: &str, event_type: &str, subscriber_id: &str) -> Result<(), String>`
  - `emit_kaizen_alert(&mut self, event_id: &str, event_type: &str, subscriber_id: &str, error_fingerprint: &str, signature: &str) -> Result<String, String>`
- `notify_dlq_failure_with_sink`:
  1. Si `event_type == "Kaizen_Alert_Required"`, no emite (anti-recursión).
  2. Calcula `sig = dlq_signature_hash(...)`.
  3. Si `sink.has_active_signature(&sig)?` es `true`, omite la emisión (`Ok(false)`).
  4. En caso contrario, emite `Kaizen_Alert_Required` y registra `sig` (`Ok(true)`).

## 2. Integración en `route_domain_core.rs`
- En la rama `dead_lettered` de `handle_subscriber`, invoca `super::dlq_kaizen_alert::notify_dlq_failure_fail_soft(repo, event_uuid, event_type, &sid, err_trace)`.

## 3. Escalado en `event-bus-audit`
- `needs_kaizen_actionable(circuit_alert: bool, actionable_stale_pending_count: u64, active_dlq_signatures: u64) -> bool`: retorna `true` si cualquiera de los 3 criterios es activo (`active_dlq_signatures > 0`).
- Cuando `active_dlq_signatures > 0`, `implicated` incorpora entradas `dlq:<err_key> (count=<n>)` desde `state.dead_letter_errors` para garantizar que `write_pending_kaizen` no quede bloqueado por `implicated.is_empty()`.

## 4. Remediación de silencios
- `event-watcher/src/main.rs`: logging explícito `[WATCHER] record_delivery error ...` en lugar de `let _ =`.
- `sddia-daemon-runtime/src/eda_sweep.rs`: logging explícito `[EDA-SWEEP] purge_event failed ...` y `[EDA-SWEEP] move_to_dlq failed ...` en lugar de `let _ =`.
