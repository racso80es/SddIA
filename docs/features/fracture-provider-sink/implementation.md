---
feature_name: fracture-provider-sink
created: "2026-10-30"
process: refactorization
items:
  - T1
  - T2
tracker_ref: AUT-30
---

# Implementación

## T1

- `project_binding`: `self_hosted_project_slug`, `inputs_with_self_hosted_slug`.
- `tracker_lifecycle_port::invoke` rechaza `Unconfigured`.
- `materialize_fracture_pbi`: rama remota y rama fichero. El fichero conserva el ledger local.
- `linear-tracker-adapter`: `description` en la query y en el listado de laboratorio.
- Gate: `materialize-fracture-pbi` puede `create_issue`.

## T2

- Evento `Fracture_Pbi_Materialized` y suscripción de Mayeuta.
- `enrich_fracture_pbi_kaizen` comenta `tracker_ref` o `target_path` opaco.
- `dispatch_subscriber` no sustituye una acción o tool por `AGENT_PHASE`.
- `emit_fracture_for_denial` devuelve `Result`. El envelope lleva `fracture_deposited`.
- `claim_store_pending` registra el fallo de apertura y suelta el store antes del route.

## Contrato

`materialize-fracture-pbi.md`: `target_path` es la referencia opaca del sumidero.
