---
feature_name: eda-catalog-delivery-plan
created: "2026-10-26"
process: refactorization
tracker_ref: AUT-17
---

# Objetivos — plan de entregas desde el catálogo

Todo suscriptor declarado en el catálogo de dominio queda informado de su evento. El alta no puede persistir un evento de dominio con `delivery_state` vacío.

## Alcance

- Puerto `stamp_published_event` en `sddia-eda-catalog`.
- `record_ecst` rechaza dominio sin plan (`EDA_DELIVERY_PLAN_MISSING`) salvo `_disposition: no_subscriber`.
- Reconciliación de eventos ya guardados sin entregas, y apagado de filas cuyo id no sale del catálogo.

## Fuera

No se edita el contenido de `event-domain-subscriptions.json`.
