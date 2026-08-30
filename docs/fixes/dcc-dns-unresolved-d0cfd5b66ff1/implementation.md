---
feature_name: dcc-dns-unresolved-d0cfd5b66ff1
created: "2026-08-30"
process: bug-fix
branch_name: fix/dcc-dns-unresolved-d0cfd5b66ff1
persist_ref: docs/fixes/dcc-dns-unresolved-d0cfd5b66ff1
items:
  - delivery_close/transient_network_classifier
  - delivery_close/f4c_net_fracture_suppression
  - delivery_close/stamp_friction_dns
  - enrich_fracture_pbi_kaizen/hook_blob_without_process_name
---

# Implementation — fractura `d0cfd5b66ff1`

## Touchpoints

| Artefacto | Cambio |
|-----------|--------|
| `delivery_close.rs` | `dcc_transient_network_trace` + `dcc_net_block_suppresses_fracture` (F4c) |
| `delivery_close.rs` | `stamp_dcc_network_block`: Err de Publicación remota / Apertura en forja con traza DNS → `blocked` + `F-DCC-DNS-UNRESOLVED`, sin `fail_soft` |
| `delivery_close.rs` | `emit_dcc_phase_fractures` no emite dominio si F4c |
| `enrich_fracture_pbi_kaizen.rs` | regla hook sobre `error_trace`+`attempted_action`; sin token unario `delivery-close` |

## Contrato (implementado)

- DNS/red transitoria en push/forja: veredicto accionable, no `System_Fracture_Detected`.
- F4b intacto (aduana evolution/EDA `blocked`).
- Forja failed no-red (`pr_url` opaco) sigue emitiendo fractura.
- Mayeuta: traza DNS + `process_name=delivery-close-cycle` no es «recursión hook»; `pre-push hook` sí.

## Fuera de alcance (respetado)

- Sin mutación de genoma `delivery-close-cycle.md`.
- Sin `offline: true` en push. Sin retry/polling. Sin reimplementar `SDDIA_HOOK_DELIVERY_CLOSE`.
