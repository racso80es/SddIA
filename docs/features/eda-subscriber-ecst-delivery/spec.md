---
feature_name: eda-subscriber-ecst-delivery
pbi_ref: AUT-46
---

# Spec — ECST desde store y entrega honesta

## Implementado (núcleo)

- `ecst_input_loader`: carga ECST desde FS o `load_dispatched_event` (redb/dual).
- Suscriptores: `tracker-markdown-apply`, `memory-evolution-ingest`, email/preferencias, `tracker-stamp` (unificado).
- `tracker-stamp`: `delivery_status` (`success` | `skipped-*` | `failed` | `failed-bus-mismatch`); verificación ECST vs bus antes de sello remoto.
- `dispatch_process_subscriber` / `dispatch_subscriber`: propagan `skipped-*` y `delivery_status`.
- `promote_witness`: persiste `DeliveryStatus` según `result_status` y `skip_reason` en store.

## Pendiente de cierre

- Test de integración por suscriptor en redb sin `.events/` (markdown-apply, route fractal).
- Test de concurrencia N≥4 en `route-domain-event` (AC5).
- `delivery-close-cycle` + PR + validacion APTO.
