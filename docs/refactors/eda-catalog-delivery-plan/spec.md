---
feature_name: eda-catalog-delivery-plan
created: "2026-10-26"
process: refactorization
tracker_ref: AUT-17
---

# Spec — plan de entregas

`stamp_published_event` lee `SddIA/core/event-domain-subscriptions.json` y, si `.SddIA/project.md` declara `codex-software-engineering`, también el `subscriptions.json` del códice. Escribe `delivery_state` con claves `agente.proceso` y valor `pending`. Sin suscriptores aplicables, o sin fichero de catálogo, sella `_disposition: no_subscriber`. Si el evento ya trae claves de suscriptor, no las pisa.

`record_ecst` en familia `domain` exige esas claves o la disposición. El valor `success` no queda como entrega pendiente. Las claves `last_batch_anchor_error` y `transaction_digest` no son suscriptores.

`reconcile_unstamped` recorre el dominio del store: siembra filas pendientes que falten y marca `Skipped` las pendientes cuyo id no está en el plan. `event-watcher` lo llama antes de reclamar.
