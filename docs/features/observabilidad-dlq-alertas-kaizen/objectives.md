---

id: aut-117-objectives
tracker_ref: AUT-117
process: feature
persist_ref: docs/features/observabilidad-dlq-alertas-kaizen
branch: feat/observabilidad-dlq-alertas-kaizen
execution_id: "9778fc29-5146-4076-bfc8-6a9be0a12ba7"
---
# Objetivos — AUT-117: Observabilidad DLQ y Alertas Kaizen

## Objetivo Principal
Eliminar la ceguera operativa ante eventos caídos en Dead-Letter Queue (DLQ) o fallos repetidos de suscriptores, conectando el fallo terminal en `route_domain_core` y la auditoría en `event-bus-audit` con la emisión controlada e idempotente de `Kaizen_Alert_Required`, además de remediar silencios `let _ = ...` en el pipeline EDA.

## Criterios de Éxito
1. **Disparo reactivo (AC-1):** Una transición a DLQ / nack terminal emite `Kaizen_Alert_Required` en `< 1` ciclo sin requerir intervención manual.
2. **Idempotencia por firma (AC-2):** Deduplicación persistente por `hash(error_fingerprint + subscriber_id + event_type)` garantizando exactamente 1 alerta activa por firma.
3. **Independencia entre firmas (AC-3):** Firmas distintas (por suscriptor, `event_type` o huella de error) emiten alertas independientes.
4. **Escalado en auditoría (AC-4):** `event-bus-audit` activa `needs_kaizen: true` y puebla `implicated` ante firmas activas en DLQ sin disparar por volumen histórico aislado.
5. **Trazabilidad de silencios (AC-5):** Inventario y remediación de `let _ =` en `event-watcher`, `eda_sweep` y auditoría de `tracker_stamp` / `route_domain_core`.
6. **Validación y gates (AC-6):** Tests unitarios herméticos verdes, `cargo check --workspace` con `-Dwarnings` verde y gates `sddia-qa` verdes.
