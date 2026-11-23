---
id: aut-117-clarify
tracker_ref: AUT-117
process: feature
persist_ref: docs/features/observabilidad-dlq-alertas-kaizen
---

# Clarificaciones y Límites — AUT-117

## Decisiones Cerradas
1. **Deduplicación:** Se normaliza la huella de error (primeras 200 chars colapsando espacios) y se calcula `FNV-1a` 64-bit sobre `"{error_fingerprint}|{subscriber_id}|{event_type}"` en `dlq_signature_hash`.
2. **Anti-recursión:** Si el propio evento fallido es `Kaizen_Alert_Required`, `notify_dlq_failure_with_sink` retorna `Ok(false)` inmediatamente para evitar bucles infinitos de alertas.
3. **No reprocesamiento histórico:** No se reencolan ni reprocesan los 15 eventos históricos en `.SddIA/runtime/events.redb`; la auditoría distingue volumen histórico (`active_dlq_signatures == 0`) de firmas activas (`active_dlq_signatures > 0` o `escalate_active_dlq: true`).
4. **Inyección de Dependencias:** Se introduce el trait `DlqAlertSink` (`ProdDlqAlertSink` en producción con estado en `.SddIA/state/dlq-kaizen-alerts.json`, y `MockDlqAlertSink` en tests unitarios herméticos).
